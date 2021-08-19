#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::bitstream::BitStream;

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
    pub const ConnectChallengeRequest: u8 = 26;
    pub const ConnectChallengeReject: u8 = 28;
    pub const ConnectChallengeResponse: u8 = 30;
    pub const ConnectRequest: u8 = 32;
    pub const ConnectReject: u8 = 34;
    pub const ConnectAccept: u8 = 36;
    pub const Disconnect: u8 = 38;
}

pub mod NetClassGroups {
    pub const NetClassGroupGame: u32 = 0;
    pub const NetClassGroupCommunity: u32 = 1;
    pub const NetClassGroup3: u32 = 2;
    pub const NetClassGroup4: u32 = 3;
    pub const NetClassGroupsCount: u32 = 4;
}

#[derive(Debug)]
pub enum Packet {
    Raw(BitStream),
    MasterServerGameTypesRequest(),
    MasterServerGameTypesResponse(),
    MasterServerListRequest(),
    MasterServerListResponse(),
    GameMasterInfoRequest(),
    GameMasterInfoResponse(),
    GamePingRequest(),
    GamePingResponse(),
    GameInfoRequest(),
    GameInfoResponse(),
    GameHeartbeat(),
    ConnectChallengeRequest {
        sequence: u32,
    },
    ConnectChallengeReject(),
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
    },
    ConnectAccept {
        sequence: u32,
    },
    Disconnect {
        sequence: u32,
        reason: String,
    },
}

impl Packet {
    pub fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut stream = BitStream::from_buffer(Vec::<u8>::from(bytes));
        let packet_type = stream.read_u8();

        if packet_type & 0x1 == 1 {
            // Raw packet
            return Some(Self::Raw(BitStream::from_buffer(Vec::<u8>::from(bytes))));
        }

        match packet_type {
            PacketTypes::ConnectChallengeRequest => {
                let sequence = stream.read_u32();
                Some(Self::ConnectChallengeRequest {
                    sequence
                })
            }
            PacketTypes::ConnectChallengeResponse => {
                let sequence = stream.read_u32();
                let address_digest = [
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32(),
                    stream.read_u32()
                ];

                Some(Self::ConnectChallengeResponse {
                    sequence,
                    address_digest
                })
            }
            PacketTypes::ConnectReject => {
                let sequence = stream.read_u32();
                Some(Self::ConnectAccept {
                    sequence
                })
            }
            PacketTypes::ConnectAccept => {
                let sequence = stream.read_u32();
                Some(Self::ConnectAccept {
                    sequence
                })
            }
            PacketTypes::Disconnect => {
                let sequence = stream.read_u32();
                let reason = stream.read_string();

                Some(Self::Disconnect {
                    sequence,
                    reason
                })
            }
            _ => {
                todo!("Unhandled packet type: {} {:?}", packet_type, bytes)
            }
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = BitStream::new();
        match self {
            Packet::Raw(raw_packet) => {
                return raw_packet.into_bytes();
            }
            Packet::MasterServerGameTypesRequest() =>  {
                out.write_u8(PacketTypes::MasterServerGameTypesRequest as u8);
                todo!();
            },
            Packet::MasterServerGameTypesResponse() =>  {
                out.write_u8(PacketTypes::MasterServerGameTypesResponse as u8);
                todo!();
            },
            Packet::MasterServerListRequest() =>  {
                out.write_u8(PacketTypes::MasterServerListRequest as u8);
                todo!();
            },
            Packet::MasterServerListResponse() =>  {
                out.write_u8(PacketTypes::MasterServerListResponse as u8);
                todo!();
            },
            Packet::GameMasterInfoRequest() =>  {
                out.write_u8(PacketTypes::GameMasterInfoRequest as u8);
                todo!();
            },
            Packet::GameMasterInfoResponse() =>  {
                out.write_u8(PacketTypes::GameMasterInfoResponse as u8);
                todo!();
            },
            Packet::GamePingRequest() =>  {
                out.write_u8(PacketTypes::GamePingRequest as u8);
                todo!();
            },
            Packet::GamePingResponse() =>  {
                out.write_u8(PacketTypes::GamePingResponse as u8);
                todo!();
            },
            Packet::GameInfoRequest() =>  {
                out.write_u8(PacketTypes::GameInfoRequest as u8);
                todo!();
            },
            Packet::GameInfoResponse() =>  {
                out.write_u8(PacketTypes::GameInfoResponse as u8);
                todo!();
            },
            Packet::GameHeartbeat() =>  {
                out.write_u8(PacketTypes::GameHeartbeat as u8);
                todo!();
            },
            Packet::ConnectChallengeRequest { sequence } =>  {
                out.write_u8(PacketTypes::ConnectChallengeRequest as u8);
                out.write_u32(sequence);
            },
            Packet::ConnectChallengeReject() =>  {
                out.write_u8(PacketTypes::ConnectChallengeReject as u8);
                todo!();
            },
            Packet::ConnectChallengeResponse { sequence, address_digest } =>  {
                out.write_u8(PacketTypes::ConnectChallengeResponse as u8);
                out.write_u32(sequence);
                out.write_u32(address_digest[0]);
                out.write_u32(address_digest[1]);
                out.write_u32(address_digest[2]);
                out.write_u32(address_digest[3]);
            },
            Packet::ConnectRequest { sequence, address_digest, class_name, net_class_group, class_crc, game_string, current_protocol_version, min_required_protocol_version, join_password, connect_argv } =>  {
                out.write_u8(PacketTypes::ConnectRequest as u8);
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
            },
            Packet::ConnectReject { sequence } =>  {
                out.write_u8(PacketTypes::ConnectReject as u8);
                out.write_u32(sequence);
            },
            Packet::ConnectAccept { sequence } =>  {
                out.write_u8(PacketTypes::ConnectAccept as u8);
                out.write_u32(sequence);
            },
            Packet::Disconnect { sequence, reason } =>  {
                out.write_u8(PacketTypes::Disconnect as u8);
                out.write_u32(sequence);
                out.write_string(reason);
            },
        }

        out.into_bytes()
    }
}
