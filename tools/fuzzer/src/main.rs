use anyhow::{anyhow, Result};
use dnet::packet::Packet;
use dnet::GameConnection;
use dnet::NetClassGroups::NetClassGroupGame;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let mut connection = GameConnection::connect("0.0.0.0:30000", "127.0.0.1:28000", 1).await?;
        connection
            .send_packet(Packet::ConnectChallengeRequest { sequence: 1 })
            .await?;
        loop {
            select! {
                _ = sleep(Duration::from_secs(5)) => {
                    println!("Heyo we got em");
                    break;
                }
                packet = connection.read_packet() => {
                    println!("Packet {:?}", packet);
                    match packet {
                        Ok(Some(Packet::Raw(raw_packet))) => {
                            connection.process_raw_packet(raw_packet).await?;
                        }
                        Ok(Some(Packet::ConnectChallengeResponse { sequence, address_digest })) => {
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
                        Ok(Some(Packet::ConnectAccept { .. })) => {
                            connection.send_raw_packet().await?;
                        }
                        Ok(Some(Packet::Disconnect { .. })) => {
                            println!("Disconnected");
                            break;
                        }
                        e @ _ => {
                            println!("Error: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
