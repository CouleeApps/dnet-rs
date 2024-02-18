use crate::packet::BitStream;
use anyhow::{Error, Result};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NetPacketType {
    DataPacket = 0,
    PingPacket = 1,
    AckPacket = 2,
    InvalidPacketType = 3,
}

pub struct DNet {
    last_seq_recvd_at_send: [u32; 32],
    last_seq_received: u32,
    highest_acked_seq: u32,
    last_send_seq: u32,
    ack_mask: u32,
    connect_sequence: u32,
    last_recv_ack_ack: u32,
    connection_established: bool,
}

pub enum DNetResult {
    SendPacket(BitStream),
    KeepAlive,
    HandleConnectionEstablished,
    HandleNotify(bool),
    HandlePacket(BitStream),
}

impl DNet {
    pub fn new(connect_sequence: u32) -> Self {
        DNet {
            last_seq_recvd_at_send: [0; 32],
            last_seq_received: 0,
            highest_acked_seq: 0,
            last_send_seq: 0,
            ack_mask: 0,
            connect_sequence,
            last_recv_ack_ack: 0,
            connection_established: false,
        }
    }

    pub fn window_full(&self) -> bool {
        return self.last_send_seq - self.highest_acked_seq >= 30;
    }

    pub fn process_raw_packet(&mut self, mut stream: BitStream) -> Result<Vec<DNetResult>> {
        let mut results = vec![];

        stream.read_flag()?;
        let connect_seq_bit = stream.read_int(1)?;
        let mut seq_num = stream.read_int(9)?;
        let mut highest_ack = stream.read_int(9)?;
        let packet_type = stream.read_int(2)?;
        let ack_byte_count = stream.read_int(3)?;

        if connect_seq_bit != (self.connect_sequence & 1) {
            return Err(Error::msg("Bad seq bit"));
        }
        if ack_byte_count > 4 {
            return Err(Error::msg("Too many ack bytes"));
        }
        if packet_type >= NetPacketType::InvalidPacketType as u32 {
            return Err(Error::msg("Invalid packet type"));
        }

        let ack_mask = stream.read_int((8 * ack_byte_count) as usize)?;

        // Check if packet number is within sequence window
        seq_num |= self.last_seq_received & 0xFFFFFE00;

        if seq_num < self.last_seq_received {
            seq_num += 0x200;
        }

        if seq_num > self.last_seq_received + 31 {
            // Out of order
            return Ok(vec![]);
        }

        highest_ack |= self.highest_acked_seq & 0xFFFFFE00;

        if highest_ack < self.highest_acked_seq {
            highest_ack += 0x200;
        }

        if highest_ack > self.last_send_seq {
            // Out of order
            return Ok(vec![]);
        }

        for i in (self.last_seq_received + 1)..seq_num {
            println!("Not recv: {}", i);
        }
        println!(
            "Recv: {} {}",
            seq_num,
            match packet_type {
                a if a == NetPacketType::DataPacket as u32 => "DataPacket",
                a if a == NetPacketType::PingPacket as u32 => "PingPacket",
                a if a == NetPacketType::AckPacket as u32 => "AckPacket",
                _ => "??",
            }
        );

        self.ack_mask <<= seq_num - self.last_seq_received;

        // ack data packets
        if packet_type == NetPacketType::DataPacket as u32 {
            self.ack_mask |= 1;
        }

        for i in (self.highest_acked_seq + 1)..=highest_ack {
            let transmit_success = (ack_mask & (1 << (highest_ack - i))) != 0;
            results.push(DNetResult::HandleNotify(transmit_success));

            println!("Ack {} {}", i, transmit_success);

            if transmit_success {
                self.last_recv_ack_ack = self.last_seq_recvd_at_send[(i & 0x1F) as usize];
                if !self.connection_established {
                    self.connection_established = true;
                    results.push(DNetResult::HandleConnectionEstablished);
                }
            }
        }

        if seq_num - self.last_recv_ack_ack > 32 {
            self.last_recv_ack_ack = seq_num - 32;
        }

        self.highest_acked_seq = highest_ack;

        if packet_type == NetPacketType::PingPacket as u32 {
            results.push(DNetResult::SendPacket(self.make_ack_packet()?));
        }

        results.push(DNetResult::KeepAlive);

        if self.last_seq_received != seq_num && packet_type == NetPacketType::DataPacket as u32 {
            results.push(DNetResult::HandlePacket(stream));
        }

        self.last_seq_received = seq_num;
        Ok(results)
    }

    fn make_ping_packet(&mut self) -> Result<BitStream> {
        let mut stream = BitStream::new();
        self.build_send_packet_header(&mut stream, NetPacketType::PingPacket);
        println!("Send ping: {}", self.last_send_seq);

        Ok(stream)
    }

    fn make_ack_packet(&mut self) -> Result<BitStream> {
        let mut stream = BitStream::new();
        self.build_send_packet_header(&mut stream, NetPacketType::AckPacket);
        println!("Send ack: {}", self.last_send_seq);

        Ok(stream)
    }

    pub fn build_send_packet_header(&mut self, stream: &mut BitStream, packet_type: NetPacketType) {
        let ack_byte_count = (self.last_seq_received - self.last_recv_ack_ack + 7) >> 3;
        assert!(ack_byte_count <= 4);

        if packet_type == NetPacketType::DataPacket {
            self.last_send_seq += 1;
        }

        println!("build hdr {} {:?}", self.last_send_seq, packet_type);

        stream.write_flag(true);
        stream.write_int(self.connect_sequence & 1, 1);
        stream.write_int(self.last_send_seq, 9);
        stream.write_int(self.last_seq_received, 9);
        stream.write_int(packet_type as u32, 2);
        stream.write_int(ack_byte_count, 3);
        stream.write_int(self.ack_mask, (ack_byte_count * 8) as usize);

        if packet_type == NetPacketType::DataPacket {
            self.last_seq_recvd_at_send[(self.last_send_seq & 0x1F) as usize] =
                self.last_seq_received;
        }
    }
}
