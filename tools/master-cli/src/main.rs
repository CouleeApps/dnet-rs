use anyhow::{anyhow, Result};
use dnet::connection::GameConnection;
use dnet::master::MasterServer;
use dnet::packet::Packet;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let mut master = MasterServer::connect("0.0.0.0:29000", "127.0.0.1:28002", 1).await?;
    master
        .send_packet(Packet::MasterServerListRequest {
            flags: 0,
            key: 123,
            session: 1,
            packet_index: 0,
            game_type: "Test".to_string(),
            mission_type: "any".to_string(),
            min_players: 0,
            max_players: 128,
            region_mask: 2,
            version: 0,
            filter_flag: 0x80,
            max_bots: 0xff,
            min_cpu: 0,
            buddy_list: vec![],
        })
        .await?;

    let mut found_servers = vec![];

    loop {
        match master.read_packet().await? {
            Some(Packet::MasterServerListResponse {
                flags,
                key,
                packet_index,
                packet_total,
                servers,
            }) => {
                found_servers.extend(servers.into_iter());
                if packet_index + 1 >= packet_total {
                    break;
                }
            }
            _ => {
                return Err(anyhow!("Expected a master response"));
            }
        }
    }

    for (addr, port) in found_servers {
        println!("{}:{}", addr, port);

        // Ask the server for its info
        let mut connection = GameConnection::connect("0.0.0.0:30000", (addr, port), 1).await?;

        connection
            .send_packet(Packet::GameInfoRequest { flags: 0, key: 123 })
            .await?;

        let info_response = select! {
            _ = sleep(Duration::from_secs(3)) => {
                Err(anyhow!("Timed out"))
            }
            packet = connection.read_packet() => {
                packet
            }
        };

        match info_response {
            Ok(Some(r @ Packet::GameInfoResponse { .. })) => {
                println!("{:#?}", r);
            }
            _ => {}
        }

        connection
            .send_packet(Packet::GameMasterInfoRequest { flags: 0, key: 123 })
            .await?;

        let info_response = select! {
            _ = sleep(Duration::from_secs(3)) => {
                Err(anyhow!("Timed out"))
            }
            packet = connection.read_packet() => {
                packet
            }
        };
        match info_response {
            Ok(Some(r @ Packet::GameMasterInfoResponse { .. })) => {
                println!("{:#?}", r);
            }
            _ => {}
        }
    }

    Ok(())
}
