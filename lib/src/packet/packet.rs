#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::bitstream::BitStream;
use anyhow::Result;
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
    pub const MasterServerRelayDelete: u8 = 70;
    pub const MasterServerRelayReady: u8 = 72;
    pub const MasterServerJoinInvite: u8 = 74;
    pub const MasterServerJoinInviteResponse: u8 = 76;
    pub const MasterServerRelayHeartbeat: u8 = 78;
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

#[derive(Debug, Copy, Clone)]
pub enum PacketSource {
    GameToGame,
    GameToMaster,
    MasterToRelay,
}

#[derive(Debug, Clone)]
pub enum Packet {
    Raw(Vec<u8>),
    MasterServerGameTypesRequest {
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGameTypesResponse {
        flags: u8,
        key: u16,
        session: u16,
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
        key: u16,
        session: u16,
        packet_index: u8,
        packet_total: u8,
        servers: Vec<(Ipv4Addr, u16)>,
    },
    GameMasterInfoRequest {
        flags: u8,
        key: u16,
        session: u16,
    },
    GameMasterInfoResponse {
        flags: u8,
        key: u16,
        session: u16,
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
        key: u16,
        session: u16,
    },
    GamePingResponse {
        flags: u8,
        key: u16,
        session: u16,
        version_string: String,
        current_protocol_version: u32,
        min_required_protocol_version: u32,
        version: u32,
        name: String,
    },
    GameInfoRequest {
        flags: u8,
        key: u16,
        session: u16,
    },
    GameInfoResponse {
        flags: u8,
        key: u16,
        session: u16,
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
        flags: u8,
        key: u16,
        session: u16,
        client_id: u16,
        possible_addresses: Vec<(Ipv4Addr, u16)>,
    },
    MasterServerAcceptArrangedConnection {
        client_id: u16,
    },
    MasterServerArrangedConnectionAccepted {
        flags: u8,
        key: u16,
        session: u16,
        possible_addresses: Vec<(Ipv4Addr, u16)>,
    },
    MasterServerRejectArrangedConnection {
        client_id: u16,
    },
    MasterServerArrangedConnectionRejected {
        flags: u8,
        key: u16,
        session: u16,
        reason: u8,
    },
    MasterServerGamePingRequest {
        address: (Ipv4Addr, u16),
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGamePingResponse {
        flags: u8,
        key: u16,
        session: u16,
        address: (Ipv4Addr, u16),
        packet: Box<Packet>,
    },
    MasterServerGameInfoRequest {
        address: (Ipv4Addr, u16),
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerGameInfoResponse {
        flags: u8,
        key: u16,
        session: u16,
        address: (Ipv4Addr, u16),
        packet: Box<Packet>,
    },
    MasterServerRelayRequestToMaster {
        address: (Ipv4Addr, u16),
    },
    MasterServerRelayRequestToRelay {
        relay_id: u32,
        server_addr: (Ipv4Addr, u16),
        client_addr: Ipv4Addr,
    },
    MasterServerRelayResponseFromRelay {
        relay_id: u32,
        relay_port: u16,
    },
    MasterServerRelayResponseFromMaster {
        flags: u8,
        key: u16,
        session: u16,
        is_host: bool,
        address: (Ipv4Addr, u16),
    },
    MasterServerRelayDelete {},
    MasterServerRelayReady {
        flags: u8,
        key: u16,
        session: u16,
    },
    MasterServerJoinInvite {
        invite_code: String,
    },
    MasterServerJoinInviteResponse {
        flags: u8,
        key: u16,
        session: u16,
        address: Option<(Ipv4Addr, u16)>,
    },
    MasterServerRelayHeartbeat {},
}

impl Packet {
    fn read_maybe_compressed_string(packet: &mut BitStream, flags: u8) -> Result<String> {
        if (flags & QueryFlags::NoStringCompress) == QueryFlags::NoStringCompress {
            packet.read_cstring()
        } else {
            packet.read_string()
        }
    }

    fn read_flags_key_session(stream: &mut BitStream) -> Result<(u8, u16, u16)> {
        let flags = stream.read_u8()?;
        let key_session = stream.read_u32()?;
        let key = (key_session & 0xffff) as u16;
        let session = (key_session >> 16) as u16;

        Ok((flags, key, session))
    }

    fn read_address(stream: &mut BitStream) -> Result<Ipv4Addr> {
        Ok(Ipv4Addr::new(
            stream.read_u8()?,
            stream.read_u8()?,
            stream.read_u8()?,
            stream.read_u8()?,
        ))
    }

    fn read_address_and_port(stream: &mut BitStream) -> Result<(Ipv4Addr, u16)> {
        Ok((
            Ipv4Addr::new(
                stream.read_u8()?,
                stream.read_u8()?,
                stream.read_u8()?,
                stream.read_u8()?,
            ),
            stream.read_u16()?,
        ))
    }

    fn read_list<F, R>(stream: &mut BitStream, count: usize, f: F) -> Result<Vec<R>>
    where
        F: Fn(&mut BitStream) -> Result<R>,
    {
        let mut result = vec![];

        for _ in 0..count {
            result.push(f(stream)?);
        }

        Ok(result)
    }

    fn read_u8_list<F, R>(stream: &mut BitStream, f: F) -> Result<Vec<R>>
    where
        F: Fn(&mut BitStream) -> Result<R>,
    {
        let length = stream.read_u8()?;
        Self::read_list(stream, length as usize, f)
    }

    fn read_u16_list<F, R>(stream: &mut BitStream, f: F) -> Result<Vec<R>>
    where
        F: Fn(&mut BitStream) -> Result<R>,
    {
        let length = stream.read_u16()?;
        Self::read_list(stream, length as usize, f)
    }

    fn read_u32_list<F, R>(stream: &mut BitStream, f: F) -> Result<Vec<R>>
    where
        F: Fn(&mut BitStream) -> Result<R>,
    {
        let length = stream.read_u32()?;
        Self::read_list(stream, length as usize, f)
    }

    fn write_maybe_compressed_string(packet: &mut BitStream, flags: u8, string: &String) {
        if (flags & QueryFlags::NoStringCompress) == QueryFlags::NoStringCompress {
            packet.write_cstring(string);
        } else {
            packet.write_string(string);
        }
    }

    fn write_flags_key_session(packet: &mut BitStream, flags: u8, key: u16, session: u16) {
        packet.write_u8(flags);
        packet.write_u32((session as u32) << 16 | key as u32);
    }

    fn write_address(packet: &mut BitStream, address: Ipv4Addr) {
        packet.write_u8(address.octets()[0]);
        packet.write_u8(address.octets()[1]);
        packet.write_u8(address.octets()[2]);
        packet.write_u8(address.octets()[3]);
    }

    fn write_address_and_port(packet: &mut BitStream, address: (Ipv4Addr, u16)) {
        packet.write_u8(address.0.octets()[0]);
        packet.write_u8(address.0.octets()[1]);
        packet.write_u8(address.0.octets()[2]);
        packet.write_u8(address.0.octets()[3]);
        packet.write_u16(address.1);
    }

    pub fn try_from_bytes(bytes: &[u8], source: PacketSource) -> Option<Self> {
        let mut stream = BitStream::from_buffer(Vec::<u8>::from(bytes));

        match Self::try_from_stream(&mut stream, source) {
            Ok(result) => result,
            Err(_) => None,
        }
    }

    pub fn try_from_stream(stream: &mut BitStream, source: PacketSource) -> Result<Option<Self>> {
        let packet_type = stream.read_u8()?;

        if packet_type & 0x1 == 1 {
            // Raw packet
            return Ok(Some(Self::Raw(Vec::<u8>::from(stream.as_bytes()))));
        }

        Ok(match packet_type {
            PacketTypes::MasterServerGameTypesRequest => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                Some(Self::MasterServerGameTypesRequest {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGameTypesResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                let game_types = Self::read_u8_list(stream, |stream| stream.read_cstring())?;
                let mission_types = Self::read_u8_list(stream, |stream| stream.read_cstring())?;

                Some(Self::MasterServerGameTypesResponse {
                    flags,
                    key,
                    session,
                    game_types,
                    mission_types,
                })
            }
            PacketTypes::MasterServerListRequest => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let packet_index = stream.read_u8()?;
                let game_type = stream.read_cstring()?;
                let mission_type = stream.read_cstring()?;
                let min_players = stream.read_u8()?;
                let max_players = stream.read_u8()?;
                let region_mask = stream.read_u32()?;
                let version = stream.read_u32()?;
                let filter_flag = stream.read_u8()?;
                let max_bots = stream.read_u8()?;
                let min_cpu = stream.read_u16()?;

                let buddy_list = Self::read_u8_list(stream, |stream| stream.read_u32())?;

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
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let packet_index = stream.read_u8()?;
                let packet_total = stream.read_u8()?;

                let servers = Self::read_u16_list(stream, Self::read_address_and_port)?;

                Some(Self::MasterServerListResponse {
                    flags,
                    key,
                    session,
                    packet_index,
                    packet_total,
                    servers,
                })
            }
            PacketTypes::GameMasterInfoRequest => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                Some(Self::GameMasterInfoRequest {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::GameMasterInfoResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let game_type = stream.read_cstring()?;
                let mission_type = stream.read_cstring()?;
                let max_players = stream.read_u8()?;
                let region_mask = stream.read_u32()?;
                let version = stream.read_u32()?;
                let filter_flag = stream.read_u8()?;
                let bot_count = stream.read_u8()?;
                let cpu_speed = stream.read_u32()?;
                let player_count = stream.read_u8()?;

                let guid_list = Self::read_list(stream, player_count as usize, |stream| {
                    // Sometimes this isn't sent
                    Ok(stream.read_u32().unwrap_or(0))
                })?;

                Some(Self::GameMasterInfoResponse {
                    flags,
                    key,
                    session,
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
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                Some(Self::GamePingRequest {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::GamePingResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let version_string = stream.read_string()?;
                let current_protocol_version = stream.read_u32()?;
                let min_required_protocol_version = stream.read_u32()?;
                let version = stream.read_u32()?;
                let name = stream.read_string()?;

                Some(Self::GamePingResponse {
                    flags,
                    key,
                    session,
                    version_string,
                    current_protocol_version,
                    min_required_protocol_version,
                    version,
                    name,
                })
            }
            PacketTypes::GameInfoRequest => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                Some(Self::GameInfoRequest {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::GameInfoResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let game_type = Self::read_maybe_compressed_string(stream, flags)?;
                let mission_type = Self::read_maybe_compressed_string(stream, flags)?;
                let mission_name = Self::read_maybe_compressed_string(stream, flags)?;
                let filter_flag = stream.read_u8()?;
                let player_count = stream.read_u8()?;
                let max_players = stream.read_u8()?;
                let bot_count = stream.read_u8()?;
                let cpu_speed = stream.read_u16()?;
                let server_info = Self::read_maybe_compressed_string(stream, flags)?;
                let server_info_query = stream.read_long_cstring()?;

                Some(Self::GameInfoResponse {
                    flags,
                    key,
                    session,
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
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
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
                let sequence = stream.read_u32()?;
                Some(Self::ConnectChallengeRequest { sequence })
            }
            PacketTypes::ConnectChallengeReject => {
                let sequence = stream.read_u32()?;
                let reason = stream.read_string()?;

                Some(Self::ConnectChallengeReject { sequence, reason })
            }
            PacketTypes::ConnectChallengeResponse => {
                let sequence = stream.read_u32()?;
                let address_digest = [
                    stream.read_u32()?,
                    stream.read_u32()?,
                    stream.read_u32()?,
                    stream.read_u32()?,
                ];

                Some(Self::ConnectChallengeResponse {
                    sequence,
                    address_digest,
                })
            }
            PacketTypes::ConnectRequest => {
                let sequence = stream.read_u32()?;
                let address_digest = [
                    stream.read_u32()?,
                    stream.read_u32()?,
                    stream.read_u32()?,
                    stream.read_u32()?,
                ];
                let class_name = stream.read_string()?;

                // NetConnection::writeConnectRequest
                let net_class_group = stream.read_u32()?;
                let class_crc = stream.read_u32()?;

                // GameConnection::writeConnectRequest
                let game_string = stream.read_string()?;
                let current_protocol_version = stream.read_u32()?;
                let min_required_protocol_version = stream.read_u32()?;
                let join_password = stream.read_string()?;

                let connect_argv = Self::read_u32_list(stream, |stream| stream.read_string())?;

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
                let sequence = stream.read_u32()?;
                let reason = stream.read_string()?;
                Some(Self::ConnectReject { sequence, reason })
            }
            PacketTypes::ConnectAccept => {
                let sequence = stream.read_u32()?;
                let protocol_version = stream.read_u32()?;
                Some(Self::ConnectAccept {
                    sequence,
                    protocol_version,
                })
            }
            PacketTypes::Disconnect => {
                let sequence = stream.read_u32()?;
                let reason = stream.read_string()?;

                Some(Self::Disconnect { sequence, reason })
            }
            PacketTypes::Punch => Some(Self::Punch {}),
            PacketTypes::ArrangedConnectRequest => {
                let sequence = stream.read_u32()?;
                let debug_object_sizes = stream.read_flag()?;

                Some(Self::ArrangedConnectRequest {
                    sequence,
                    debug_object_sizes,
                })
            }
            PacketTypes::MasterServerRequestArrangedConnection => {
                let address = Self::read_address_and_port(stream)?;

                Some(Self::MasterServerRequestArrangedConnection { address })
            }
            PacketTypes::MasterServerClientRequestedArrangedConnection => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let client_id = stream.read_u16()?;

                let possible_addresses = Self::read_u8_list(stream, Self::read_address_and_port)?;

                Some(Self::MasterServerClientRequestedArrangedConnection {
                    flags,
                    key,
                    session,
                    client_id,
                    possible_addresses,
                })
            }
            PacketTypes::MasterServerAcceptArrangedConnection => {
                let client_id = stream.read_u16()?;

                Some(Self::MasterServerAcceptArrangedConnection { client_id })
            }
            PacketTypes::MasterServerArrangedConnectionAccepted => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                let possible_addresses = Self::read_u8_list(stream, Self::read_address_and_port)?;

                Some(Self::MasterServerArrangedConnectionAccepted {
                    flags,
                    key,
                    session,
                    possible_addresses,
                })
            }
            PacketTypes::MasterServerRejectArrangedConnection => {
                let client_id = stream.read_u16()?;
                Some(Self::MasterServerRejectArrangedConnection { client_id })
            }
            PacketTypes::MasterServerArrangedConnectionRejected => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let reason = stream.read_u8()?;
                Some(Self::MasterServerArrangedConnectionRejected {
                    flags,
                    key,
                    session,
                    reason,
                })
            }
            PacketTypes::MasterServerGamePingRequest => {
                // Why tho
                let address = Self::read_address_and_port(stream)?;
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                Some(Self::MasterServerGamePingRequest {
                    address,
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGamePingResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let address = Self::read_address_and_port(stream)?;
                let buffer = Vec::from(&stream.as_bytes()[(stream.get_bit_pos() / 8)..]);
                let packet = Packet::try_from_bytes(buffer.as_slice(), source)
                    .unwrap_or_else(|| Self::Raw(buffer));
                Some(Self::MasterServerGamePingResponse {
                    flags,
                    key,
                    session,
                    address,
                    packet: Box::new(packet),
                })
            }
            PacketTypes::MasterServerGameInfoRequest => {
                let address = Self::read_address_and_port(stream)?;
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                Some(Self::MasterServerGameInfoRequest {
                    address,
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerGameInfoResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let address = Self::read_address_and_port(stream)?;
                let buffer = Vec::from(&stream.as_bytes()[(stream.get_bit_pos() / 8)..]);
                let packet = Packet::try_from_bytes(buffer.as_slice(), source)
                    .unwrap_or_else(|| Self::Raw(buffer));

                Some(Self::MasterServerGameInfoResponse {
                    flags,
                    key,
                    session,
                    address,
                    packet: Box::new(packet),
                })
            }
            PacketTypes::MasterServerRelayRequest => match source {
                PacketSource::GameToGame => None,
                PacketSource::GameToMaster => {
                    let address = Self::read_address_and_port(stream)?;
                    Some(Self::MasterServerRelayRequestToMaster { address })
                }
                PacketSource::MasterToRelay => {
                    let relay_id = stream.read_u32()?;
                    let server_addr = Self::read_address_and_port(stream)?;
                    let client_addr = Self::read_address(stream)?;
                    Some(Self::MasterServerRelayRequestToRelay {
                        relay_id,
                        server_addr,
                        client_addr,
                    })
                }
            },
            PacketTypes::MasterServerRelayResponse => match source {
                PacketSource::GameToGame => None,
                PacketSource::GameToMaster => {
                    let (flags, key, session) = Self::read_flags_key_session(stream)?;
                    let is_host = stream.read_flag()?;
                    let address = Self::read_address_and_port(stream)?;
                    Some(Self::MasterServerRelayResponseFromMaster {
                        flags,
                        key,
                        session,
                        is_host,
                        address,
                    })
                }
                PacketSource::MasterToRelay => {
                    let relay_id = stream.read_u32()?;
                    let relay_port = stream.read_u16()?;
                    Some(Self::MasterServerRelayResponseFromRelay {
                        relay_id,
                        relay_port,
                    })
                }
            },
            PacketTypes::MasterServerRelayDelete => Some(Self::MasterServerRelayDelete {}),
            PacketTypes::MasterServerRelayReady => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;

                Some(Self::MasterServerRelayReady {
                    flags,
                    key,
                    session,
                })
            }
            PacketTypes::MasterServerJoinInvite => {
                let invite_code = stream.read_cstring()?;
                Some(Self::MasterServerJoinInvite { invite_code })
            }
            PacketTypes::MasterServerJoinInviteResponse => {
                let (flags, key, session) = Self::read_flags_key_session(stream)?;
                let found = stream.read_u8()?;
                let address = if found == 1 {
                    Some(Self::read_address_and_port(stream)?)
                } else {
                    None
                };
                Some(Self::MasterServerJoinInviteResponse {
                    flags,
                    key,
                    session,
                    address,
                })
            }
            PacketTypes::MasterServerRelayHeartbeat => Some(Self::MasterServerRelayHeartbeat {}),
            _ => {
                eprintln!(
                    "Unknown packet type: {} {:?}",
                    packet_type,
                    stream.as_bytes()
                );
                None
            }
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = BitStream::new();
        match self {
            Packet::Raw(raw_packet) => {
                return raw_packet;
            }
            Packet::MasterServerGameTypesRequest {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGameTypesRequest);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::MasterServerGameTypesResponse {
                flags,
                key,
                session,
                game_types,
                mission_types,
            } => {
                out.write_u8(PacketTypes::MasterServerGameTypesResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);

                out.write_u8(game_types.len() as u8);
                for game_type in game_types {
                    out.write_cstring(&game_type);
                }
                out.write_u8(mission_types.len() as u8);
                for mission_type in mission_types {
                    out.write_cstring(&mission_type);
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
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_u8(packet_index);
                out.write_cstring(&game_type);
                out.write_cstring(&mission_type);
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
                session,
                packet_index,
                packet_total,
                servers,
            } => {
                out.write_u8(PacketTypes::MasterServerListResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_u8(packet_index);
                out.write_u8(packet_total);

                out.write_u16(servers.len() as u16);
                for server in servers {
                    Self::write_address_and_port(&mut out, server);
                }
            }
            Packet::GameMasterInfoRequest {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::GameMasterInfoRequest);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::GameMasterInfoResponse {
                flags,
                key,
                session,
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
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_cstring(&game_type);
                out.write_cstring(&mission_type);
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
            Packet::GamePingRequest {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::GamePingRequest);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::GamePingResponse {
                flags,
                key,
                session,
                version_string,
                current_protocol_version,
                min_required_protocol_version,
                version,
                name,
            } => {
                out.write_u8(PacketTypes::GamePingResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                Self::write_maybe_compressed_string(&mut out, flags, &version_string);
                out.write_u32(current_protocol_version);
                out.write_u32(min_required_protocol_version);
                out.write_u32(version);
                Self::write_maybe_compressed_string(&mut out, flags, &name);
            }
            Packet::GameInfoRequest {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::GameInfoRequest);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::GameInfoResponse {
                flags,
                key,
                session,
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
                Self::write_flags_key_session(&mut out, flags, key, session);
                Self::write_maybe_compressed_string(&mut out, flags, &game_type);
                Self::write_maybe_compressed_string(&mut out, flags, &mission_type);
                Self::write_maybe_compressed_string(&mut out, flags, &mission_name);
                out.write_u8(filter_flag);
                out.write_u8(player_count);
                out.write_u8(max_players);
                out.write_u8(bot_count);
                out.write_u16(cpu_speed);
                Self::write_maybe_compressed_string(&mut out, flags, &server_info);
                out.write_long_cstring(&server_info_query);
            }
            Packet::GameHeartbeat {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::GameHeartbeat);
                Self::write_flags_key_session(&mut out, flags, key, session);
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
                out.write_string(&reason);
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
                out.write_string(&class_name);

                // NetConnection::writeConnectRequest
                out.write_u32(net_class_group);
                out.write_u32(class_crc);

                // GameConnection::writeConnectRequest
                out.write_string(&game_string);
                out.write_u32(current_protocol_version);
                out.write_u32(min_required_protocol_version);
                out.write_string(&join_password);

                out.write_u32(connect_argv.len() as u32);
                for arg in connect_argv {
                    out.write_string(&arg);
                }
            }
            Packet::ConnectReject { sequence, reason } => {
                out.write_u8(PacketTypes::ConnectReject);
                out.write_u32(sequence);
                out.write_string(&reason);
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
                out.write_string(&reason);
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
            Packet::MasterServerRequestArrangedConnection { address } => {
                out.write_u8(PacketTypes::MasterServerRequestArrangedConnection);
                Self::write_address_and_port(&mut out, address);
            }
            Packet::MasterServerClientRequestedArrangedConnection {
                flags,
                key,
                session,
                client_id,
                possible_addresses,
            } => {
                out.write_u8(PacketTypes::MasterServerClientRequestedArrangedConnection);
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_u16(client_id);

                out.write_u8(possible_addresses.len() as u8);
                for address in possible_addresses {
                    Self::write_address_and_port(&mut out, address);
                }
            }
            Packet::MasterServerAcceptArrangedConnection { client_id } => {
                out.write_u8(PacketTypes::MasterServerAcceptArrangedConnection);
                out.write_u16(client_id);
            }
            Packet::MasterServerArrangedConnectionAccepted {
                flags,
                key,
                session,
                possible_addresses,
            } => {
                out.write_u8(PacketTypes::MasterServerArrangedConnectionAccepted);
                Self::write_flags_key_session(&mut out, flags, key, session);

                out.write_u8(possible_addresses.len() as u8);
                for address in possible_addresses {
                    Self::write_address_and_port(&mut out, address);
                }
            }
            Packet::MasterServerRejectArrangedConnection { client_id } => {
                out.write_u8(PacketTypes::MasterServerRejectArrangedConnection);
                out.write_u16(client_id);
            }
            Packet::MasterServerArrangedConnectionRejected {
                flags,
                key,
                session,
                reason,
            } => {
                out.write_u8(PacketTypes::MasterServerArrangedConnectionRejected);
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_u8(reason);
            }
            Packet::MasterServerGamePingRequest {
                address,
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGamePingRequest);
                // Backwards because fuck me that's why
                Self::write_address_and_port(&mut out, address);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::MasterServerGamePingResponse {
                flags,
                key,
                session,
                address,
                packet,
            } => {
                out.write_u8(PacketTypes::MasterServerGamePingResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                Self::write_address_and_port(&mut out, address);
                for b in packet.into_bytes() {
                    out.write_u8(b);
                }
            }
            Packet::MasterServerGameInfoRequest {
                address,
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerGameInfoRequest);
                Self::write_address_and_port(&mut out, address);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::MasterServerGameInfoResponse {
                flags,
                key,
                session,
                address,
                packet,
            } => {
                out.write_u8(PacketTypes::MasterServerGameInfoResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                Self::write_address_and_port(&mut out, address);
                for b in packet.into_bytes() {
                    out.write_u8(b);
                }
            }
            Packet::MasterServerRelayRequestToMaster { address } => {
                out.write_u8(PacketTypes::MasterServerRelayRequest);
                Self::write_address_and_port(&mut out, address);
            }
            Packet::MasterServerRelayRequestToRelay {
                relay_id,
                server_addr,
                client_addr,
            } => {
                out.write_u8(PacketTypes::MasterServerRelayRequest);
                out.write_u32(relay_id);
                Self::write_address_and_port(&mut out, server_addr);
                Self::write_address(&mut out, client_addr);
            }
            Packet::MasterServerRelayResponseFromMaster {
                flags,
                key,
                session,
                is_host,
                address,
            } => {
                out.write_u8(PacketTypes::MasterServerRelayResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                out.write_flag(is_host);
                Self::write_address_and_port(&mut out, address);
            }
            Packet::MasterServerRelayResponseFromRelay {
                relay_id,
                relay_port,
            } => {
                out.write_u8(PacketTypes::MasterServerRelayResponse);
                out.write_u32(relay_id);
                out.write_u16(relay_port);
            }
            Packet::MasterServerRelayDelete {} => {
                out.write_u8(PacketTypes::MasterServerRelayDelete);
            }
            Packet::MasterServerRelayReady {
                flags,
                key,
                session,
            } => {
                out.write_u8(PacketTypes::MasterServerRelayReady);
                Self::write_flags_key_session(&mut out, flags, key, session);
            }
            Packet::MasterServerJoinInvite { invite_code } => {
                out.write_u8(PacketTypes::MasterServerJoinInvite);
                out.write_cstring(&invite_code);
            }
            Packet::MasterServerJoinInviteResponse {
                flags,
                key,
                session,
                address,
            } => {
                out.write_u8(PacketTypes::MasterServerJoinInviteResponse);
                Self::write_flags_key_session(&mut out, flags, key, session);
                match address {
                    Some(address) => {
                        out.write_u8(1);
                        Self::write_address_and_port(&mut out, address);
                    }
                    None => {
                        out.write_u8(0);
                    }
                }
            }
            Packet::MasterServerRelayHeartbeat {} => {
                out.write_u8(PacketTypes::MasterServerRelayHeartbeat);
            }
        }

        out.into_bytes()
    }
}
