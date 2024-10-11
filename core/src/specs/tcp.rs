use std::{
    fmt::{Display, Formatter, Write},
    ops::Deref,
};

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::{
        base::{Context, Frame, PacketBuilder, PacketContext, TCPDetail, TCPInfo, TCPPAYLOAD},
        io::{AReader, Reader},
        Description, MultiBlock, PortPacket, Ref2, FIELDSTATUS,
    },
    constants::tcp_option_kind_mapper,
};

use super::ProtocolData;

#[derive(Default)]
pub struct TCPState {
    pub head: u8,
    cwr: bool,
    ece: bool,
    urg: bool,
    ack: bool,
    push: bool,
    reset: bool,
    sync: bool,
    fin: bool,
}
pub const ACK: u8 = 16;
pub const PUSH: u8 = 8;
pub const RESET: u8 = 4;
pub const SYNC: u8 = 2;
pub const FIN: u8 = 1;

const C_ACK: &str = "ACK";
const C_PUSH: &str = "PUSH";
const C_RESET: &str = "RESET";
const C_SYN: &str = "SYN";
const C_FIN: &str = "FIN";
impl TCPState {
    fn update(&mut self, head: u16) {
        self.head = (head & 0xff) as u8;
        let lann = |off: u8| -> bool { 1 == (head >> off) & 0x01 };
        self.cwr = lann(7);
        self.ece = lann(6);
        self.urg = lann(5);
        self.ack = lann(4);
        self.push = lann(3);
        self.reset = lann(2);
        self.sync = lann(1);
        self.fin = lann(0);
    }
    fn to_string(&self) -> String {
        let mut str = String::from("[");
        let mut list = Vec::new();
        if self.ack {
            list.push(C_ACK)
        };
        if self.push {
            list.push(C_PUSH);
        }
        if self.reset {
            list.push(C_RESET);
        }
        if self.sync {
            list.push(C_SYN);
        }
        if self.fin {
            list.push(C_FIN);
        }
        str.write_str(list.join(",").as_str()).unwrap();
        str.write_str("]").unwrap();
        str
    }
    pub fn check(&self, mask: u8) -> bool {
        (self.head & mask) == mask
    }
    pub fn _match(&self, mask: u8) -> bool {
        (self.head & mask) > 0
    }
}
// struct TCPExtra {
//     dump: bool,
// }
#[derive(Default, Packet)]
pub struct TCPOption {
    kind: u8,
    len: u8,
    pub data: TCPOptionKind,
}

// pub struct TCPOptionKindBlock {
//     data: Vec<u8>,
// }
//
// pub struct TCPTIMESTAMP {
//     sender: u32,
//     reply: u32,
// }
pub struct TCPUserTimeout;
impl TCPUserTimeout {
    fn desc(data: u16) -> String {
        let g = data >> 15;
        let v = data & 0x7fff;
        match g {
            1 => format!("{} minus", v),
            _ => format!("{} second", v),
        }
    }
}
#[derive(Default)]
pub enum TCPOptionKind {
    #[default]
    NOP,
    EOL,
    MSS(u16),
    SCALE(u8),
    SACK,
    TIMESTAMP((u32, u32)),
    USERTIMEOUT(u16),
    BLOCK(Vec<u8>),
}
impl Display for TCPOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("TCP Option - {}", self.kind()).as_str())
    }
}
impl TCPOption {
    fn kind(&self) -> String {
        format!("Kind: {} ({})", tcp_option_kind_mapper(self.kind as u16), self.kind)
    }
}

type TCPOptions = Ref2<MultiBlock<TCPOption>>;
#[derive(Default, Packet)]
pub struct TCP {
    pub sequence: u32,
    pub acknowledge: u32,
    pub source_port: u16,
    pub target_port: u16,
    head: u16,
    pub len: u16,
    pub payload_len: u16,
    window: u16,
    pub crc: u16,
    urgent: u16,
    pub options: Option<TCPOptions>,
    pub state: TCPState,
    pub info: Option<TCPInfo>,
}

impl std::fmt::Display for TCP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Transmission Control Protocol, Src Port: {}, Dst Port: {}, Seq: {}, Ack: {}, Len: {}", self.source_port, self.target_port, self.sequence, self.acknowledge, self.len))
    }
}
impl PortPacket for TCP {
    fn source_port(&self) -> u16 {
        self.source_port
    }

    fn target_port(&self) -> u16 {
        self.target_port
    }
}
impl crate::common::base::InfoPacket for TCP {
    fn info(&self) -> String {
        let mut info = format!("{} → {} {} Seq={} Ack={} Win={} Len={}", self.source_port, self.target_port, self.state.to_string(), self.sequence, self.acknowledge, self.window, self.payload_len);
        if let Some(_info) = &self.info {
            match _info.detail {
                TCPDetail::KEEPALIVE => info = format!("[{}] {}", "Keeplive", info),
                TCPDetail::NOPREVCAPTURE => info = format!("[{}] {}", "no_previous_segment", info),
                TCPDetail::RETRANSMISSION => info = format!("[{}] {}", "retransmission", info),
                _ => {}
            }
        }
        info
    }

    fn status(&self) -> FIELDSTATUS {
        if self.state._match(RESET) {
            return FIELDSTATUS::ERROR;
        }
        match &self.info {
            Some(_info) => match &_info.detail {
                TCPDetail::DUMP => FIELDSTATUS::WARN,
                TCPDetail::NOPREVCAPTURE => FIELDSTATUS::WARN,
                TCPDetail::RETRANSMISSION => FIELDSTATUS::WARN,
                _ => FIELDSTATUS::INFO,
            },
            None => FIELDSTATUS::INFO,
        }
    }
}
impl TCP {
    fn set_head(&mut self, head: u16) {
        self.head = head;
        self.len = (head >> 12) & 0x0f;
        self.state.update(head);
    }
    fn sequence_desc(&self) -> String {
        match &self.info {
            Some(info) => {
                let _seq = info._seq;
                if _seq <= self.sequence {
                    return format!("Sequence Number : {} (raw: {})", self.sequence - _seq, self.sequence);
                }
                format!("Sequence Number (raw): {}", self.sequence)
            }
            None => {
                format!("Sequence Number (raw): {}", self.sequence)
            }
        }
    }
    fn acknowledge_desc(&self) -> String {
        match &self.info {
            Some(info) => {
                let _ack = info._ack;
                if _ack <= self.acknowledge {
                    return format!("Acknowlagde Number : {} (raw: {})", self.acknowledge - _ack, self.acknowledge);
                }
                format!("Acknowlagde Number (raw): {}", self.acknowledge)
            }
            None => {
                format!("Acknowlagde Number (raw): {}", self.acknowledge)
            }
        }
    }
    fn len_desc(&self) -> String {
        format!("{:04b} .... = Header Length: 32 bytes ({})", self.len, self.len)
    }
}
pub struct TCPVisitor;

impl TCPVisitor {
    fn read_option(reader: &Reader, _: Option<()>) -> Result<PacketContext<TCPOption>> {
        let packet: PacketContext<TCPOption> = Frame::create_packet();
        let mut option = packet.get().borrow_mut();
        option.kind = packet.build_lazy(reader, Reader::_read8, TCPOption::kind)?;
        match option.kind {
            5 | 29 | 30 => {
                let len = packet.build_format(reader, Reader::_read8, "Length: {}")?;
                option.len = len;
                let raw = reader.slice((len - 2) as usize);
                option.data = TCPOptionKind::BLOCK(raw.to_vec());
            }
            0 => {
                option.data = TCPOptionKind::EOL;
            }
            1 => {
                option.data = TCPOptionKind::NOP;
            }
            2 => {
                packet.build_format(reader, Reader::_read8, "Length: {}")?;
                let value = packet.build_format(reader, Reader::_read16_be, "MSS Value: {}")?;
                option.data = TCPOptionKind::MSS(value);
            }
            3 => {
                packet.build_format(reader, Reader::_read8, "Length: {}")?;
                let value = packet.build_format(reader, Reader::_read8, "Shift count: {}")?;
                option.data = TCPOptionKind::SCALE(value);
            }
            4 => {
                packet.build_format(reader, Reader::_read8, "Length: {}")?;
                option.data = TCPOptionKind::SACK;
            }
            8 => {
                let len = packet.build_format(reader, Reader::_read8, "Length: {}")?;
                match len {
                    10 => {
                        let sender = packet.build_format(reader, Reader::_read32_be, "sender: {}")?;
                        let reply = packet.build_format(reader, Reader::_read32_be, "reply: {}")?;
                        option.data = TCPOptionKind::TIMESTAMP((sender, reply))
                    }
                    _ => {
                        let raw = reader.slice((len - 2) as usize);
                        option.data = TCPOptionKind::BLOCK(raw.to_vec());
                    }
                }
            }
            28 => {
                //https://datatracker.ietf.org/doc/html/rfc5482
                packet.build_format(reader, Reader::_read8, "Length: {}")?;
                let value = packet.build_fn(reader, Reader::_read16_be, TCPUserTimeout::desc)?;
                option.data = TCPOptionKind::USERTIMEOUT(value);
            }
            _ => {}
        }
        drop(option);
        Ok(packet)
    }
    fn read_options(reader: &Reader, opt: Option<usize>) -> Result<PacketContext<MultiBlock<TCPOption>>> {
        let len = opt.unwrap();
        let packet: PacketContext<MultiBlock<TCPOption>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let start = reader.cursor();
        let end = start + (len - 5) * 4;
        while reader.cursor() < end {
            let item = packet.build_packet(reader, TCPVisitor::read_option, None, None)?;
            p.push(item);
        }
        drop(p);
        Ok(packet)
    }
}

impl crate::common::base::Visitor for TCPVisitor {
    fn visit(&self, frame: &mut Frame, ctx: &mut Context, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let ip_packet = frame.get_ip();
        let unwap = ip_packet.deref().borrow();
        let total = unwap.payload_len();
        let _start = reader.left()? as u16;
        let packet: PacketContext<TCP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.source_port = packet.build_lazy(reader, Reader::_read16_be, Description::source_port)?;
        p.target_port = packet.build_lazy(reader, Reader::_read16_be, Description::target_port)?;
        p.sequence = packet.build_lazy(reader, Reader::_read32_be, TCP::sequence_desc)?;
        p.acknowledge = packet.build_lazy(reader, Reader::_read32_be, TCP::acknowledge_desc)?;
        let head = packet.build_lazy(reader, Reader::_read16_be, TCP::len_desc)?;
        p.window = packet.build_format(reader, Reader::_read16_be, "Window: {}")?;
        p.crc = packet.build_format(reader, Reader::_read16_be, "Checksum: {}")?;
        p.urgent = packet.build_format(reader, Reader::_read16_be, "Urgent Pointer: {}")?;
        p.set_head(head);
        let len = p.len;
        if len > 5 {
            let options = packet.build_packet(reader, TCPVisitor::read_options, Some(len as usize), Some("Options".into()))?;
            p.options = Some(options);
        }
        let left_size = reader.left().unwrap_or(0) as u16;
        p.payload_len = left_size;
        if _start > total {
            p.payload_len = total + left_size - _start;
        }
        if left_size > 0 {
            packet._build(reader, reader.cursor(), p.payload_len.into(), format!("TCP payload ({} bytes)", left_size));
        }
        frame.add_tcp(packet._clone_obj());
        // let _data = reader._slice(left_size as usize);
        // let info = frame.update_tcp(p.deref(), _data, ctx);
        
        drop(p);
        Ok((ProtocolData::TCP(packet), "none"))
        // match &info.detail {
        //     TCPDetail::NONE => {
        //         p.info = Some(info);
        //         drop(p);
        //         handle(frame, ctx, reader, packet)
        //     }
        //     _ => {
        //         p.info = Some(info);
        //         drop(p);
        //         Ok((ProtocolData::TCP(packet), "none"))
        //     }
        // }
    }
}

// fn handle(frame: &mut Frame, ctx: &mut Context, reader: &Reader, packet: PacketContext<TCP>) -> Result<(ProtocolData, &'static str)> {
//     let _len = reader.left()?;
//     if _len < 1 {
//         return Ok((ProtocolData::TCP(packet), "none"));
//     }
//     // let ep = frame.get_tcp_info(true,ctx);
//     let (key, arch) = frame.get_tcp_map_key();
//     let _map = &mut ctx.conversation_map;
//     let conn = _map.get(&key).unwrap().borrow_mut();
//     let _type = &conn.connec_type;
//     // end
//     match _type {
//         TCPPAYLOAD::TLS => {
//             return Ok((ProtocolData::TCP(packet), "tls"));
//         }
//         TCPPAYLOAD::NONE => {
//             let (is_tls, _) = super::tls::TLS::check(reader)?;
//             if is_tls {
//                 return Ok((ProtocolData::TCP(packet), "tls"));
//             } else if super::http::HTTPVisitor::check(reader) {
//                 return Ok((ProtocolData::TCP(packet), "http"));
//             }
//             Ok((ProtocolData::TCP(packet), "none"))
//         }
//     }
// }
