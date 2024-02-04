#![allow(non_snake_case)]

use crate::bitstream::BitStream;
use crate::dnet::{DNet, DNetResult, NetPacketType};
use crate::packet::Packet;
use anyhow::{Error, Result};
use rand::Rng;
use std::time::Duration;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct GameConnection {
    socket: UdpSocket,
    connect_sequence: u32,
    dnet: DNet,
}

impl GameConnection {
    pub async fn connect<B: ToSocketAddrs, C: ToSocketAddrs>(
        bind_address: B,
        connect_address: C,
        connect_sequence: u32,
    ) -> Result<Self> {
        let socket = UdpSocket::bind(bind_address).await?;
        let std_socket = socket.into_std()?;
        std_socket.set_write_timeout(Some(Duration::from_secs(30)))?;
        std_socket.set_read_timeout(Some(Duration::from_secs(30)))?;
        let socket = UdpSocket::from_std(std_socket)?;
        socket.connect(connect_address).await?;

        let mut connection = GameConnection {
            socket,
            connect_sequence,
            dnet: DNet::new(connect_sequence),
        };

        Ok(connection)
    }

    pub async fn send_packet(&mut self, packet: Packet) -> Result<()> {
        println!("Send {:?}", packet);
        let bytes = packet.into_bytes();
        println!(">>> {:?}", &bytes);
        self.socket.send(bytes.as_slice()).await?;
        Ok(())
    }

    pub async fn send_raw(&mut self, stream: BitStream) -> Result<()> {
        let bytes = stream.into_bytes();
        println!(">>> {:?}", &bytes);
        self.socket.send(bytes.as_slice()).await?;
        Ok(())
    }

    pub async fn read_packet(&mut self) -> Result<Option<Packet>> {
        let mut bytes = vec![];
        bytes.resize(1440, 0); // UDP MTR
        let len = self.socket.recv(bytes.as_mut_slice()).await?;

        println!("<<< {:?}", &bytes[0..len]);
        let packet = Packet::try_from_bytes(&bytes[0..len]);

        Ok(packet)
    }

    pub async fn process_raw_packet(&mut self, stream: BitStream) -> Result<()> {
        for result in self.dnet.process_raw_packet(stream)? {
            match result {
                DNetResult::SendPacket(packet) => self.send_raw(packet).await?,
                DNetResult::KeepAlive => {
                    // Nothing
                }
                DNetResult::HandleConnectionEstablished => {
                    println!("Connection established");
                }
                DNetResult::HandleNotify(recvd) => {
                    println!("Last packet was recvd: {}", recvd);
                }
                DNetResult::HandlePacket(packet) => {
                    println!("Packet: {:?}", packet);

                    self.send_raw_packet().await?;
                }
            }
        }
        Ok(())
    }

    pub async fn send_raw_packet(&mut self) -> Result<()> {
        let mut packet = BitStream::new();
        self.dnet
            .build_send_packet_header(&mut packet, NetPacketType::DataPacket);

        // packet.write_flag(false); // update mCurRate
        // packet.write_flag(false); // more rate control
        //
        // packet.write_int(0, 32); // move packet start
        // packet.write_int(0, 5); // move packet count
        //
        // packet.write_flag(false); // camera pos
        // packet.write_flag(false); // control force mismatch
        // packet.write_flag(false); // changed control scheme
        // packet.write_flag(false); // changed first person
        // packet.write_flag(false); // changed fov

        // LOL crash
        for _ in 0..2000 {
            packet.write_flag(rand::thread_rng().gen_bool(0.5));
        }

        self.send_raw(packet).await?;

        Ok(())
    }
}
