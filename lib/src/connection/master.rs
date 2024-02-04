use super::dnet::DNet;
use crate::packet::Packet;
use crate::BitStream;
use anyhow::{Error, Result};
use std::time::Duration;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct MasterServer {
    socket: UdpSocket,
    connect_sequence: u32,
    dnet: DNet,
}

impl MasterServer {
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

        let mut connection = MasterServer {
            socket,
            connect_sequence,
            dnet: DNet::new(connect_sequence),
        };

        Ok(connection)
    }

    pub async fn send_packet(&mut self, packet: Packet) -> Result<()> {
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
}
