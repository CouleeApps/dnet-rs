use crate::packet::Packet;
use crate::BitStream;
use crate::PacketSource::GameToMaster;
use anyhow::{anyhow, Error, Result};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;

pub struct MasterServer {
    tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
    rx: tokio::sync::broadcast::Receiver<Packet>,
    tx_thread: JoinHandle<Result<()>>,
    rx_thread: JoinHandle<Result<()>>,
    ids: Arc<Mutex<Box<dyn Iterator<Item = usize>>>>,
}

impl MasterServer {
    pub async fn connect<B: ToSocketAddrs, C: ToSocketAddrs>(
        bind_address: B,
        connect_address: C,
    ) -> Result<Self> {
        let socket = UdpSocket::bind(bind_address).await?;
        let std_socket = socket.into_std()?;
        std_socket.set_write_timeout(Some(Duration::from_secs(30)))?;
        std_socket.set_read_timeout(Some(Duration::from_secs(30)))?;
        let socket = UdpSocket::from_std(std_socket)?;
        socket.connect(connect_address).await?;

        // Turn the socket into an Arc so that we can send it to both tasks
        let socket = Arc::new(socket);
        let tx_socket = socket.clone();
        let rx_socket = socket.clone();

        // Outgoing packets go in a channel to the worker
        let (tx_tx, mut tx_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

        // Incoming packets go into the worker, and then jobs can receive from
        // self.rx
        let (rx_tx, rx_rx) = tokio::sync::broadcast::channel::<Packet>(16);

        let tx_thread = tokio::spawn(async move {
            loop {
                if let Some(packet) = tx_rx.recv().await {
                    println!(">>> {:?}", &packet);
                    tx_socket.send(packet.as_slice()).await?;
                } else {
                    break;
                }
            }
            Ok(())
        });

        let rx_thread = tokio::spawn(async move {
            loop {
                let mut buf: [u8; 1400] = [0; 1400];
                let len = rx_socket.recv(&mut buf).await?;
                println!("<<< {:?}", &buf[0..len]);
                if let Some(packet) = Packet::try_from_bytes(&buf[0..len], GameToMaster) {
                    println!("<<< {:#?}", &packet);
                    rx_tx.send(packet)?;
                }
            }
        });

        let connection = MasterServer {
            tx: tx_tx,
            rx: rx_rx,
            tx_thread,
            rx_thread,
            ids: Arc::new(Mutex::new(Box::new(
                (0usize..).into_iter().map(|i| (i & 0xFFF) + 0x1000),
            ))),
        };

        Ok(connection)
    }

    pub async fn send_packet(&self, packet: Packet) -> Result<()> {
        let bytes = packet.into_bytes();
        self.tx.send(bytes)?;
        Ok(())
    }

    pub async fn send_raw(&self, stream: BitStream) -> Result<()> {
        let bytes = stream.into_bytes();
        println!(">>> {:?}", &bytes);
        self.tx.send(bytes)?;
        Ok(())
    }

    pub async fn query_servers(
        &self,
        flags: u8,
        game_type: String,
        mission_type: String,
        min_players: u8,
        max_players: u8,
        region_mask: u32,
        version: u32,
        filter_flag: u8,
        max_bots: u8,
        min_cpu: u16,
        buddy_list: Vec<u32>,
    ) -> Result<Vec<(Ipv4Addr, u16)>> {
        let mut rx = self.rx.resubscribe();
        let tx = self.tx.clone();
        let key = self.ids.lock().await.next().expect("never ends") as u16;
        let session = self.ids.lock().await.next().expect("never ends") as u16;

        task::spawn(async move {
            tx.send(
                Packet::MasterServerListRequest {
                    flags,
                    key,
                    session,
                    packet_index: 0,
                    game_type,
                    mission_type,
                    min_players,
                    max_players,
                    region_mask,
                    version,
                    filter_flag,
                    max_bots,
                    min_cpu,
                    buddy_list,
                }
                .into_bytes(),
            )?;

            let mut found_servers = vec![];

            loop {
                let packet = rx.recv().await?;
                match packet {
                    Packet::MasterServerListResponse {
                        flags,
                        key: response_key,
                        session: response_session,
                        packet_index,
                        packet_total,
                        servers,
                    } if response_key == key && response_session == session => {
                        found_servers.extend(servers.into_iter());
                        if packet_index + 1 >= packet_total {
                            break;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            return Ok(found_servers);
        })
        .await?
    }
}
