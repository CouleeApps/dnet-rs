use anyhow::{anyhow, Result};
use dnet::connection::GameConnection;
use dnet::MasterServer;
use dnet::Packet;
use std::future::Future;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::time::{sleep, Duration};

async fn run_with_timeout<T>(timeout: Duration, f: impl Future<Output = Result<T>>) -> Result<T> {
    select! {
        _ = sleep(timeout) => {
            Err(anyhow!("Timed out"))
        }
        result = f => {
            result
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut master = MasterServer::connect("0.0.0.0:29000", "127.0.0.1:28002").await?;
    let found_servers = master
        .query_servers(
            0,
            "Test".to_string(),
            "any".to_string(),
            0,
            255,
            2,
            0,
            64,
            0,
            0,
            vec![],
        )
        .await?;

    println!("{found_servers:?}");

    for (addr, port) in found_servers {
        println!("{}:{}", addr, port);

        // Ask the server for its info
        let mut connection = GameConnection::connect("0.0.0.0:30000", (addr, port), 1).await?;

        connection
            .send_packet(Packet::GameInfoRequest {
                flags: 0,
                key: 123,
                session: 0,
            })
            .await?;

        master
            .send_packet(Packet::MasterServerGameInfoRequest {
                address: (addr, port),
                flags: 0,
                key: 123,
                session: 0,
            })
            .await?;

        let info_response = select! {
            _ = sleep(Duration::from_secs(5)) => {
                Err(anyhow!("Timed out"))
            }
            // master_packet = master.read_packet() => {
            //     master_packet
            // }
            conn_packet = connection.read_packet() => {
                conn_packet.and_then(|a| a.ok_or_else(|| anyhow!("Didn't parse")))
            }
        }?;

        match info_response {
            r @ Packet::GameInfoResponse { .. } => {
                println!("{:#?}", r);
            }
            _ => {}
        }

        connection
            .send_packet(Packet::GameMasterInfoRequest {
                flags: 0,
                key: 123,
                session: 0,
            })
            .await?;

        let info_response =
            run_with_timeout(Duration::from_secs(3), connection.read_packet()).await?;
        match info_response {
            Some(r @ Packet::GameMasterInfoResponse { .. }) => {
                println!("{:#?}", r);
            }
            _ => {}
        }
    }

    Ok(())
}
