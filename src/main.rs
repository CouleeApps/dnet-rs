use crate::bitstream::BitStream;
use tokio::net::UdpSocket;
use anyhow::Result;
use crate::packet::Packet;
use crate::packet::NetClassGroups::NetClassGroupGame;
use crate::connection::GameConnection;

pub mod bitstream;
pub mod connection;
pub mod dnet;
pub mod packet;

#[tokio::main]
async fn main() -> Result<()> {
    let mut connection = GameConnection::connect("0.0.0.0:30000", "127.0.0.1:28000", 1).await?;

    loop {
        let packet = connection.read_packet().await?;
        println!("Parsed: {:?}", packet);

        match packet {
            Some(Packet::Raw(raw_packet)) => {
                connection.process_raw_packet(raw_packet).await?;
            }
            Some(Packet::ConnectChallengeResponse { sequence, address_digest }) => {
                connection.send_packet(Packet::ConnectRequest {
                    sequence,
                    address_digest,
                    class_name: "GameConnection".to_string(),
                    net_class_group: NetClassGroupGame,
                    class_crc: 0xffffffff,
                    game_string: "Test".to_string(),
                    current_protocol_version: 12,
                    min_required_protocol_version: 9,
                    join_password: "".to_string(),
                    connect_argv: vec![],
                }).await?;
            },
            _ => {}
        }

    }

    Ok(())
}
