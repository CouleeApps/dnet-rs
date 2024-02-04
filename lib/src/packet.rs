#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::bitstream::BitStream;
use std::net::Ipv4Addr;

pub mod PacketTypes {
    pub const MasterServerGameTypesRequest: u8 = 2;
    pub const MasterServerGameTypesResponse: u8 = 4;
    pub const MasterServerListRequest: u8 = 6;
    pub const MasterServerListResponse: u8 = 8;
    pub const GameMasterInfoRequest: u8 = 10;
    pub const GameMasterInfoResponse: u8 = 12;
    pub const GamePingRequest: u8 = 14;
    pub const GamePingResponse: u8 = 16;
    pub const GameInfoRequest: u8 = 18;
    pub const GameInfoResponse: u8 = 20;
    pub const GameHeartbeat: u8 = 22;
    pub const GGCPacket: u8 = 24;
    pub const ConnectChallengeRequest: u8 = 26;
    pub const ConnectChallengeReject: u8 = 28;
    pub const ConnectChallengeResponse: u8 = 30;
    pub const ConnectRequest: u8 = 32;
    pub const ConnectReject: u8 = 34;
    pub const ConnectAccept: u8 = 36;
    pub const Disconnect: u8 = 38;

    /// OpenMBU Hole Punching Extensions

    pub const Punch: u8 = 40;
    pub const ArrangedConnectRequest: u8 = 42;
    pub const MasterServerRequestArrangedConnection: u8 = 46;
    pub const MasterServerClientRequestedArrangedConnection: u8 = 48;
    pub const MasterServerAcceptArrangedConnection: u8 = 50;
    pub const MasterServerArrangedConnectionAccepted: u8 = 52;
    pub const MasterServerRejectArrangedConnection: u8 = 54;
    pub const MasterServerArrangedConnectionRejected: u8 = 56;
    pub const MasterServerGamePingRequest: u8 = 58;
    pub const MasterServerGamePingResponse: u8 = 60;
    pub const MasterServerGameInfoRequest: u8 = 62;
    pub const MasterServerGameInfoResponse: u8 = 64;
    pub const MasterServerRelayRequest: u8 = 66;
    pub const MasterServerRelayResponse: u8 = 68;
    pub const MasterServerRelayReady: u8 = 72;
    pub const MasterServerJoinInvite: u8 = 74;
    pub const MasterServerJoinInviteResponse: u8 = 76;
}

pub mod NetClassGroups {
    pub const NetClassGroupGame: u32 = 0;
    pub const NetClassGroupCommunity: u32 = 1;
    pub const NetClassGroup3: u32 = 2;
    pub const NetClassGroup4: u32 = 3;
    pub const NetClassGroupsCount: u32 = 4;
}

pub mod QueryFlags {
    pub const OnlineQuery: u8 = 0;
    pub const OfflineQuery: u8 = 1;
    pub const NoStringCompress: u8 = 2;
}

pub mod FilterFlags {
    pub const Dedicated: u8 = 0;
    pub const NotPasworded: u8 = 1;
    pub const Linux: u8 = 2;
    pub const CurrentVersion: u8 = 128;
}

#[derive(Debug)]
pub enum Packet {
    Raw(BitStream),
    MasterServerGameTypesRequest {
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGameTypesResponse {
        flags: u8,
        key: u32,
        game_types: Vec<String>,
        mission_types: Vec<String>,
    },
    MasterServerListRequest {
        flags: u8,
        key: u16,
        session: u16,
        packet_index: u8,
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
    },
    MasterServerListResponse {
        flags: u8,
        key: u32,
        packet_index: u8,
        packet_total: u8,
        servers: Vec<(Ipv4Addr, u16)>,
    },
    GameMasterInfoRequest {
        flags: u8,
        key: u32,
    },
    GameMasterInfoResponse {
        flags: u8,
        key: u32,
        game_type: String,
        mission_type: String,
        max_players: u8,
        region_mask: u32,
        version: u32,
        filter_flag: u8,
        bot_count: u8,
        cpu_speed: u32,
        player_count: u8,
        guid_list: Vec<u32>,
    },
    GamePingRequest {
        flags: u8,
        key: u32,
    },
    GamePingResponse {
        flags: u8,
        key: u32,
        version_string: String,
        current_protocol_version: u32,
        min_required_protocol_version: u32,
        version: u32,
        name: String,
    },
    GameInfoRequest {
        flags: u8,
        key: u32,
    },
    GameInfoResponse {
        flags: u8,
        key: u32,
        game_type: String,
        mission_type: String,
        mission_name: String,
        filter_flag: u8,
        player_count: u8,
        max_players: u8,
        bot_count: u8,
        cpu_speed: u16,
        server_info: String,
        server_info_query: String,
    },
    GameHeartbeat {
        flags: u8,
        key: u16,
        session: u16,
    },
    GGCPacket {},
    ConnectChallengeRequest {
        sequence: u32,
    },
    ConnectChallengeReject {
        sequence: u32,
        reason: String,
    },
    ConnectChallengeResponse {
        sequence: u32,
        address_digest: [u32; 4],
    },
    ConnectRequest {
        sequence: u32,
        address_digest: [u32; 4],
        class_name: String,
        net_class_group: u32,
        class_crc: u32,
        game_string: String,
        current_protocol_version: u32,
        min_required_protocol_version: u32,
        join_password: String,
        connect_argv: Vec<String>,
    },
    ConnectReject {
        sequence: u32,
        reason: String,
    },
    ConnectAccept {
        sequence: u32,
        protocol_version: u32,
    },
    Disconnect {
        sequence: u32,
        reason: String,
    },
    Punch {},
    ArrangedConnectRequest {
        sequence: u32,
        debug_object_sizes: bool,
    },
    MasterServerRequestArrangedConnection {
        address: (Ipv4Addr, u16),
    },
    MasterServerClientRequestedArrangedConnection {
        client_id: u16,
        possible_addresses: Vec<(Ipv4Addr, u16)>,
    },
    MasterServerAcceptArrangedConnection {
        client_id: u16,
    },
    MasterServerArrangedConnectionAccepted {
        possible_addresses: Vec<(Ipv4Addr, u16)>,
    },
    MasterServerRejectArrangedConnection {
        reason: u8, // todo: not implemented in engine
    },
    MasterServerArrangedConnectionRejected {
        reason: u8, // todo: not implemented in engine
    },
    MasterServerGamePingRequest {
        address: (Ipv4Addr, u16),
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGamePingResponse {
        address: (Ipv4Addr, u16),
        cmd: u8,
        flags: u8,
        key: u32,
    },
    MasterServerGameInfoRequest {
        address: (Ipv4Addr, u16),
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGameInfoResponse {
        address: (Ipv4Addr, u16),
        cmd: u8,
        flags: u8,
        key: u32,
    },
    MasterServerRelayRequest {
        address: (Ipv4Addr, u16),
    },
    MasterServerRelayResponse {
        is_host: bool,
        address: (Ipv4Addr, u16),
    },
    MasterServerRelayReady {},
    MasterServerJoinInvite {
        invite_code: String,
    },
    MasterServerJoinInviteResponse {
        flags: u8,
        key: u32,
        found: u8,
        address: (Ipv4Addr, u16),
    },
}

impl Packet {
    fn read_maybe_compressed_string(packet: &mut BitStream, flags: u8) -> String {
        if (flags & QueryFlags::NoStringCompress) == QueryFlags::NoStringCompress {
            packet.read_cstring()
        } else {
            packet.read_string()
        }
    }

    fn write_maybe_compressed_string(packet: &mut BitStream, flags: u8, string: String) {
        if (flags & QueryFlags::NoStringCompress) == QueryFlags::NoStringCompress {
            packet.write_cstring(string);
        } else {
            packet.write_string(string);
        }
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut stream = BitStream::from_buffer(Vec::<u8>::from(bytes));
        let packet_type = stream.read_u8();

        if packet_type & 0x1 == 1 {
            // Raw packet
            return Some(Self::Raw(BitStream::from_buffer(Vec::<u8>::from(bytes))));
        }

        match packet_type {
            PacketTypes::MasterServerGameTypesRequest => {
                let flags = stream.read_u8();
                let key_session = stream.read_u32();
                let key = (key_session >> 16) as u16;
                let session = (key_session & 0xffff) as u16;

                Some(Self::MasterServerGameTypesRequest {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGameTypesResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();

                let game_type_count = stream.read_u8() as usize;
                let mut game_types = vec![];
                for _ in 0..game_type_count {
                    game_types.push(stream.read_cstring());
                }

                let mission_type_count = stream.read_u8() as usize;
                let mut mission_types = vec![];
                for _ in 0..mission_type_count {
                    mission_types.push(stream.read_cstring());
                }

                Some(Self::MasterServerGameTypesResponse {
                    flags,
                    key,
                    game_types,
                    mission_types,
                })
            }
            PacketTypes::MasterServerListRequest => {
                let flags = stream.read_u8();
                let key_session = stream.read_u32();
                let key = (key_session >> 16) as u16;
                let session = (key_session & 0xffff) as u16;
                let packet_index = stream.read_u8();
                let game_type = stream.read_cstring();
                let mission_type = stream.read_cstring();
                let min_players = stream.read_u8();
                let max_players = stream.read_u8();
                let region_mask = stream.read_u32();
                let version = stream.read_u32();
                let filter_flag = stream.read_u8();
                let max_bots = stream.read_u8();
                let min_cpu = stream.read_u16();

                let buddy_count = stream.read_u8();
                let mut buddy_list = vec![];
                for _ in 0..buddy_count {
                    buddy_list.push(stream.read_u32());
                }

                Some(Self::MasterServerListRequest {
                    flags,
                    key,
                    session,
                    packet_index,
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
                })
            }
            PacketTypes::MasterServerListResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();
                let packet_index = stream.read_u8();
                let packet_total = stream.read_u8();

                let server_count = stream.read_u16() as usize;
                let mut servers = vec![];
                for _ in 0..server_count {
                    servers.push((
                        Ipv4Addr::new(
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                        ),
                        stream.read_u16(),
                    ));
                }

                Some(Self::MasterServerListResponse {
                    flags,
                    key,
                    packet_index,
                    packet_total,
                    servers,
                })
            }
            PacketTypes::GameMasterInfoRequest => {
                let flags = stream.read_u8();
                let key = stream.read_u32();

                Some(Self::GameMasterInfoRequest { flags, key })
            }
            PacketTypes::GameMasterInfoResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();
                let game_type = stream.read_cstring();
                let mission_type = stream.read_cstring();
                let max_players = stream.read_u8();
                let region_mask = stream.read_u32();
                let version = stream.read_u32();
                let filter_flag = stream.read_u8();
                let bot_count = stream.read_u8();
                let cpu_speed = stream.read_u32();
                let player_count = stream.read_u8();

                let mut guid_list = vec![];
                for _ in 0..player_count {
                    guid_list.push(stream.read_u32());
                }

                Some(Self::GameMasterInfoResponse {
                    flags,
                    key,
                    game_type,
                    mission_type,
                    max_players,
                    region_mask,
                    version,
                    filter_flag,
                    bot_count,
                    cpu_speed,
                    player_count,
                    guid_list,
                })
            }
            PacketTypes::GamePingRequest => {
                let flags = stream.read_u8();
                let key = stream.read_u32();

                Some(Self::GamePingRequest { flags, key })
            }
            PacketTypes::GamePingResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();
                let version_string = stream.read_string();
                let current_protocol_version = stream.read_u32();
                let min_required_protocol_version = stream.read_u32();
                let version = stream.read_u32();
                let name = stream.read_string();

                Some(Self::GamePingResponse {
                    flags,
                    key,
                    version_string,
                    current_protocol_version,
                    min_required_protocol_version,
                    version,
                    name,
                })
            }
            PacketTypes::GameInfoRequest => {
                let flags = stream.read_u8();
                let key = stream.read_u32();

                Some(Self::GameInfoRequest { flags, key })
            }
            PacketTypes::GameInfoResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();
                let game_type = Self::read_maybe_compressed_string(&mut stream, flags);
                let mission_type = Self::read_maybe_compressed_string(&mut stream, flags);
                let mission_name = Self::read_maybe_compressed_string(&mut stream, flags);
                let filter_flag = stream.read_u8();
                let player_count = stream.read_u8();
                let max_players = stream.read_u8();
                let bot_count = stream.read_u8();
                let cpu_speed = stream.read_u16();
                let server_info = Self::read_maybe_compressed_string(&mut stream, flags);
                let server_info_query = stream.read_long_cstring();

                Some(Self::GameInfoResponse {
                    flags,
                    key,
                    game_type,
                    mission_type,
                    mission_name,
                    filter_flag,
                    player_count,
                    max_players,
                    bot_count,
                    cpu_speed,
                    server_info,
                    server_info_query,
                })
            }
            PacketTypes::GameHeartbeat => {
                let flags = stream.read_u8();
                let key_session = stream.read_u32();
                let key = (key_session >> 16) as u16;
                let session = (key_session & 0xffff) as u16;
                Some(Self::GameHeartbeat {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::GGCPacket => {
                todo!()
            }
            PacketTypes::ConnectChallengeRequest => {
                let sequence = stream.read_u32();
                Some(Self::ConnectChallengeRequest { sequence })
            }
            PacketTypes::ConnectChallengeReject => {
                let sequence = stream.read_u32();
                let reason = stream.read_string();

                Some(Self::ConnectChallengeReject { sequence, reason })
            }
            PacketTypes::ConnectChallengeResponse => {
                let sequence = stream.read_u32();
                let address_digest = [
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32(),
                ];

                Some(Self::ConnectChallengeResponse {
                    sequence,
                    address_digest,
                })
            }
            PacketTypes::ConnectRequest => {
                let sequence = stream.read_u32();
                let address_digest = [
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32(),
                ];
                let class_name = stream.read_string();

                // NetConnection::writeConnectRequest
                let net_class_group = stream.read_u32();
                let class_crc = stream.read_u32();

                // GameConnection::writeConnectRequest
                let game_string = stream.read_string();
                let current_protocol_version = stream.read_u32();
                let min_required_protocol_version = stream.read_u32();
                let join_password = stream.read_string();
                let num_connect_argv = stream.read_u32();

                let mut connect_argv = vec![];
                for _ in 0..num_connect_argv {
                    connect_argv.push(stream.read_string());
                }

                Some(Self::ConnectRequest {
                    sequence,
                    address_digest,
                    class_name,
                    net_class_group,
                    class_crc,
                    game_string,
                    current_protocol_version,
                    min_required_protocol_version,
                    join_password,
                    connect_argv,
                })
            }
            PacketTypes::ConnectReject => {
                let sequence = stream.read_u32();
                let reason = stream.read_string();
                Some(Self::ConnectReject { sequence, reason })
            }
            PacketTypes::ConnectAccept => {
                let sequence = stream.read_u32();
                let protocol_version = stream.read_u32();
                Some(Self::ConnectAccept {
                    sequence,
                    protocol_version,
                })
            }
            PacketTypes::Disconnect => {
                let sequence = stream.read_u32();
                let reason = stream.read_string();

                Some(Self::Disconnect { sequence, reason })
            }
            PacketTypes::Punch => Some(Self::Punch {}),
            PacketTypes::ArrangedConnectRequest => {
                let sequence = stream.read_u32();
                let debug_object_sizes = stream.read_flag();

                Some(Self::ArrangedConnectRequest {
                    sequence,
                    debug_object_sizes,
                })
            }
            PacketTypes::MasterServerRequestArrangedConnection => {
                let address = Ipv4Addr::new(
                    stream.read_u8(),
                    stream.read_u8(),
                    stream.read_u8(),
                    stream.read_u8(),
                );
                let port = stream.read_u16();

                Some(Self::MasterServerRequestArrangedConnection {
                    address: (address, port),
                })
            }
            PacketTypes::MasterServerClientRequestedArrangedConnection => {
                let client_id = stream.read_u16();
                let mut possible_addresses = vec![];

                let possible_address_len = stream.read_u8();
                for _ in 0..possible_address_len {
                    possible_addresses.push((
                        Ipv4Addr::new(
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                        ),
                        stream.read_u16(),
                    ));
                }

                Some(Self::MasterServerClientRequestedArrangedConnection {
                    client_id,
                    possible_addresses,
                })
            }
            PacketTypes::MasterServerAcceptArrangedConnection => {
                let client_id = stream.read_u16();

                Some(Self::MasterServerAcceptArrangedConnection { client_id })
            }
            PacketTypes::MasterServerArrangedConnectionAccepted => {
                let mut possible_addresses = vec![];

                let possible_address_len = stream.read_u8();
                for _ in 0..possible_address_len {
                    possible_addresses.push((
                        Ipv4Addr::new(
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                            stream.read_u8(),
                        ),
                        stream.read_u16(),
                    ));
                }

                Some(Self::MasterServerArrangedConnectionAccepted { possible_addresses })
            }
            PacketTypes::MasterServerRejectArrangedConnection => {
                let reason = stream.read_u8();
                Some(Self::MasterServerRejectArrangedConnection { reason })
            }
            PacketTypes::MasterServerArrangedConnectionRejected => {
                let reason = stream.read_u8();
                Some(Self::MasterServerArrangedConnectionRejected { reason })
            }
            PacketTypes::MasterServerGamePingRequest => {
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                let flags = stream.read_u8();
                let key_session = stream.read_u32();
                let key = (key_session >> 16) as u16;
                let session = (key_session & 0xffff) as u16;
                Some(Self::MasterServerGamePingRequest {
                    address,
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGamePingResponse => {
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                let cmd = stream.read_u8();
                let flags = stream.read_u8();
                let key = stream.read_u32();
                Some(Self::MasterServerGamePingResponse {
                    address,
                    cmd,
                    flags,
                    key,
                })
            }
            PacketTypes::MasterServerGameInfoRequest => {
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                let flags = stream.read_u8();
                let key_session = stream.read_u32();
                let key = (key_session >> 16) as u16;
                let session = (key_session & 0xffff) as u16;
                Some(Self::MasterServerGameInfoRequest {
                    address,
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGameInfoResponse => {
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                let cmd = stream.read_u8();
                let flags = stream.read_u8();
                let key = stream.read_u32();
                Some(Self::MasterServerGameInfoResponse {
                    address,
                    cmd,
                    flags,
                    key,
                })
            }
            PacketTypes::MasterServerRelayRequest => {
                let address = Ipv4Addr::new(
                    stream.read_u8(),
                    stream.read_u8(),
                    stream.read_u8(),
                    stream.read_u8(),
                );
                let port = stream.read_u16();
                Some(Self::MasterServerRelayRequest {
                    address: (address, port),
                })
            }
            PacketTypes::MasterServerRelayResponse => {
                let is_host = stream.read_flag();
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                Some(Self::MasterServerRelayResponse { is_host, address })
            }
            PacketTypes::MasterServerRelayReady => Some(Self::MasterServerRelayReady {}),
            PacketTypes::MasterServerJoinInvite => {
                let invite_code = stream.read_cstring();
                Some(Self::MasterServerJoinInvite { invite_code })
            }
            PacketTypes::MasterServerJoinInviteResponse => {
                let flags = stream.read_u8();
                let key = stream.read_u32();
                let found = stream.read_u8();
                let address = (
                    Ipv4Addr::new(
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                        stream.read_u8(),
                    ),
                    stream.read_u16(),
                );
                Some(Self::MasterServerJoinInviteResponse {
                    flags,
                    key,
                    found,
                    address,
                })
            }
            _ => {
                todo!("Unknown packet type: {} {:?}", packet_type, bytes)
            }
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = BitStream::new();
        match self {
            Packet::Raw(raw_packet) => {
                return raw_packet.into_bytes();
            }
            Packet::MasterServerGameTypesRequest {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGameTypesRequest);
                out.write_u8(flags);
                out.write_u32((session as u32) << 16 | key as u32);
            }
            Packet::MasterServerGameTypesResponse {
                flags,
                key,
                game_types,
                mission_types,
            } => {
                out.write_u8(PacketTypes::MasterServerGameTypesResponse);
                out.write_u8(flags);
                out.write_u32(key);

                out.write_u32(game_types.len() as u32);
                for game_type in game_types {
                    out.write_cstring(game_type);
                }
                out.write_u32(mission_types.len() as u32);
                for mission_type in mission_types {
                    out.write_cstring(mission_type);
                }
            }
            Packet::MasterServerListRequest {
                flags,
                key,
                session,
                packet_index,
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
            } => {
                out.write_u8(PacketTypes::MasterServerListRequest);
                out.write_u8(flags);
                out.write_u32((session as u32) << 16 | key as u32);
                out.write_u8(packet_index);
                out.write_cstring(game_type);
                out.write_cstring(mission_type);
                out.write_u8(min_players);
                out.write_u8(max_players);
                out.write_u32(region_mask);
                if (filter_flag & FilterFlags::CurrentVersion) == FilterFlags::CurrentVersion {
                    out.write_u32(version);
                } else {
                    out.write_u32(0);
                }
                out.write_u8(filter_flag);
                out.write_u8(max_bots);
                out.write_u16(min_cpu);
                out.write_u8(buddy_list.len() as u8);
                for buddy in buddy_list {
                    out.write_u32(buddy);
                }
            }
            Packet::MasterServerListResponse {
                flags,
                key,
                packet_index,
                packet_total,
                servers,
            } => {
                out.write_u8(PacketTypes::MasterServerListResponse);
                out.write_u8(flags);
                out.write_u32(key);
                out.write_u8(packet_index);
                out.write_u8(packet_total);

                out.write_u16(servers.len() as u16);
                for server in servers {
                    out.write_u8(server.0.octets()[0]);
                    out.write_u8(server.0.octets()[1]);
                    out.write_u8(server.0.octets()[2]);
                    out.write_u8(server.0.octets()[3]);
                    out.write_u16(server.1);
                }
            }
            Packet::GameMasterInfoRequest { flags, key } => {
                out.write_u8(PacketTypes::GameMasterInfoRequest);
                out.write_u8(flags);
                out.write_u32(key);
            }
            Packet::GameMasterInfoResponse {
                flags,
                key,
                game_type,
                mission_type,
                max_players,
                region_mask,
                version,
                filter_flag,
                bot_count,
                cpu_speed,
                player_count,
                guid_list,
            } => {
                out.write_u8(PacketTypes::GameMasterInfoResponse);
                out.write_u8(flags);
                out.write_u32(key);
                out.write_cstring(game_type);
                out.write_cstring(mission_type);
                out.write_u8(max_players);
                out.write_u32(region_mask);
                out.write_u32(version);
                out.write_u8(filter_flag);
                out.write_u8(bot_count);
                out.write_u32(cpu_speed);

                out.write_u8(player_count);
                for &guid in &guid_list {
                    out.write_u32(guid);
                }
                for _ in guid_list.len()..(player_count as usize) {
                    out.write_u32(0);
                }
            }
            Packet::GamePingRequest { flags, key } => {
                out.write_u8(PacketTypes::GamePingRequest);
                out.write_u8(flags);
                out.write_u32(key);
            }
            Packet::GamePingResponse {
                flags,
                key,
                version_string,
                current_protocol_version,
                min_required_protocol_version,
                version,
                name,
            } => {
                out.write_u8(PacketTypes::GamePingResponse);
                out.write_u8(flags);
                out.write_u32(key);
                Self::write_maybe_compressed_string(&mut out, flags, version_string);
                out.write_u32(current_protocol_version);
                out.write_u32(min_required_protocol_version);
                out.write_u32(version);
                Self::write_maybe_compressed_string(&mut out, flags, name);
            }
            Packet::GameInfoRequest { flags, key } => {
                out.write_u8(PacketTypes::GameInfoRequest);
                out.write_u8(flags);
                out.write_u32(key);
            }
            Packet::GameInfoResponse {
                flags,
                key,
                game_type,
                mission_type,
                mission_name,
                filter_flag,
                player_count,
                max_players,
                bot_count,
                cpu_speed,
                server_info,
                server_info_query,
            } => {
                out.write_u8(PacketTypes::GameInfoResponse);
                out.write_u8(flags);
                out.write_u32(key);
                Self::write_maybe_compressed_string(&mut out, flags, game_type);
                Self::write_maybe_compressed_string(&mut out, flags, mission_type);
                Self::write_maybe_compressed_string(&mut out, flags, mission_name);
                out.write_u8(filter_flag);
                out.write_u8(player_count);
                out.write_u8(max_players);
                out.write_u8(bot_count);
                out.write_u16(cpu_speed);
                Self::write_maybe_compressed_string(&mut out, flags, server_info);
                out.write_long_cstring(server_info_query);
            }
            Packet::GameHeartbeat {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::GameHeartbeat);
                out.write_u8(flags);
                out.write_u32((session as u32) << 16 | key as u32);
            }
            Packet::GGCPacket {} => {
                out.write_u8(PacketTypes::GGCPacket);
                todo!();
            }
            Packet::ConnectChallengeRequest { sequence } => {
                out.write_u8(PacketTypes::ConnectChallengeRequest);
                out.write_u32(sequence);
            }
            Packet::ConnectChallengeReject { sequence, reason } => {
                out.write_u8(PacketTypes::ConnectChallengeReject);
                out.write_u32(sequence);
                out.write_string(reason);
            }
            Packet::ConnectChallengeResponse {
                sequence,
                address_digest,
            } => {
                out.write_u8(PacketTypes::ConnectChallengeResponse);
                out.write_u32(sequence);
                out.write_u32(address_digest[0]);
                out.write_u32(address_digest[1]);
                out.write_u32(address_digest[2]);
                out.write_u32(address_digest[3]);
            }
            Packet::ConnectRequest {
                sequence,
                address_digest,
                class_name,
                net_class_group,
                class_crc,
                game_string,
                current_protocol_version,
                min_required_protocol_version,
                join_password,
                connect_argv,
            } => {
                out.write_u8(PacketTypes::ConnectRequest);
                out.write_u32(sequence);
                out.write_u32(address_digest[0]);
                out.write_u32(address_digest[1]);
                out.write_u32(address_digest[2]);
                out.write_u32(address_digest[3]);
                out.write_string(class_name);

                // NetConnection::writeConnectRequest
                out.write_u32(net_class_group);
                out.write_u32(class_crc);

                // GameConnection::writeConnectRequest
                out.write_string(game_string);
                out.write_u32(current_protocol_version);
                out.write_u32(min_required_protocol_version);
                out.write_string(join_password);
                out.write_u32(connect_argv.len() as u32);

                for arg in connect_argv {
                    out.write_string(arg);
                }
            }
            Packet::ConnectReject { sequence, reason } => {
                out.write_u8(PacketTypes::ConnectReject);
                out.write_u32(sequence);
                out.write_string(reason);
            }
            Packet::ConnectAccept {
                sequence,
                protocol_version,
            } => {
                out.write_u8(PacketTypes::ConnectAccept);
                out.write_u32(sequence);

                // NetConnection::readConnectAccept
                // GameConnection::readConnectAccept
                out.write_u32(protocol_version);
            }
            Packet::Disconnect { sequence, reason } => {
                out.write_u8(PacketTypes::Disconnect);
                out.write_u32(sequence);
                out.write_string(reason);
            }
            Packet::Punch {} => {
                out.write_u8(PacketTypes::Punch);
            }
            Packet::ArrangedConnectRequest {
                sequence,
                debug_object_sizes,
            } => {
                out.write_u8(PacketTypes::ArrangedConnectRequest);
                out.write_u32(sequence);
                out.write_flag(debug_object_sizes);
            }
            Packet::MasterServerRequestArrangedConnection {
                address: (address, port),
            } => {
                out.write_u8(PacketTypes::MasterServerRequestArrangedConnection);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
            }
            Packet::MasterServerClientRequestedArrangedConnection {
                client_id,
                possible_addresses,
            } => {
                out.write_u8(PacketTypes::MasterServerClientRequestedArrangedConnection);
                out.write_u16(client_id);
                out.write_u8(possible_addresses.len() as u8);
                for (address, port) in possible_addresses {
                    out.write_u8(address.octets()[0]);
                    out.write_u8(address.octets()[1]);
                    out.write_u8(address.octets()[2]);
                    out.write_u8(address.octets()[3]);
                    out.write_u16(port);
                }
            }
            Packet::MasterServerAcceptArrangedConnection { client_id } => {
                out.write_u8(PacketTypes::MasterServerAcceptArrangedConnection);
                out.write_u16(client_id);
            }
            Packet::MasterServerArrangedConnectionAccepted { possible_addresses } => {
                out.write_u8(PacketTypes::MasterServerArrangedConnectionAccepted);
                out.write_u8(possible_addresses.len() as u8);
                for (address, port) in possible_addresses {
                    out.write_u8(address.octets()[0]);
                    out.write_u8(address.octets()[1]);
                    out.write_u8(address.octets()[2]);
                    out.write_u8(address.octets()[3]);
                    out.write_u16(port);
                }
            }
            Packet::MasterServerRejectArrangedConnection { reason } => {
                out.write_u8(PacketTypes::MasterServerRejectArrangedConnection);
                out.write_u8(reason);
            }
            Packet::MasterServerArrangedConnectionRejected { reason } => {
                out.write_u8(PacketTypes::MasterServerArrangedConnectionRejected);
                out.write_u8(reason);
            }
            Packet::MasterServerGamePingRequest {
                address: (address, port),
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGamePingRequest);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
                out.write_u8(flags);
                out.write_u32(((session as u32) << 16) | ((key as u32) & 0xffff));
            }
            Packet::MasterServerGamePingResponse {
                address: (address, port),
                cmd,
                flags,
                key,
            } => {
                out.write_u8(PacketTypes::MasterServerGamePingResponse);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
                out.write_u8(cmd);
                out.write_u8(flags);
                out.write_u32(key);
            }
            Packet::MasterServerGameInfoRequest {
                address: (address, port),
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGameInfoRequest);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
                out.write_u8(flags);
                out.write_u32(((session as u32) << 16) | ((key as u32) & 0xffff));
            }
            Packet::MasterServerGameInfoResponse {
                address: (address, port),
                cmd,
                flags,
                key,
            } => {
                out.write_u8(PacketTypes::MasterServerGameInfoResponse);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
                out.write_u8(cmd);
                out.write_u8(flags);
                out.write_u32(key);
            }
            Packet::MasterServerRelayRequest {
                address: (address, port),
            } => {
                out.write_u8(PacketTypes::MasterServerRelayRequest);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
            }
            Packet::MasterServerRelayResponse {
                is_host,
                address: (address, port),
            } => {
                out.write_u8(PacketTypes::MasterServerRelayResponse);
                out.write_flag(is_host);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
            }
            Packet::MasterServerRelayReady {} => {
                out.write_u8(PacketTypes::MasterServerRelayReady);
            }
            Packet::MasterServerJoinInvite { invite_code } => {
                out.write_u8(PacketTypes::MasterServerJoinInvite);
                out.write_cstring(invite_code);
            }
            Packet::MasterServerJoinInviteResponse {
                flags,
                key,
                found,
                address: (address, port),
            } => {
                out.write_u8(PacketTypes::MasterServerJoinInviteResponse);
                out.write_u8(flags);
                out.write_u32(key);
                out.write_u8(found);
                out.write_u8(address.octets()[0]);
                out.write_u8(address.octets()[1]);
                out.write_u8(address.octets()[2]);
                out.write_u8(address.octets()[3]);
                out.write_u16(port);
            }
        }

        out.into_bytes()
    }
}
