use std::{
    borrow::Borrow, cell::RefCell, fmt::{Display, Formatter, Write}, ops::Deref, rc::Rc
};

use anyhow::Result;
use log::info;
use pcap_derive::Packet;

use crate::{
    common::{Description, PortPacket, Reader},
    constants::tcp_option_kind_mapper,
    files::{Frame, Initer, MultiBlock, PacketContext, Ref2, TCPDetail, TCPInfo},
};

#[derive(Default)]
pub struct TCPState {
    head: u8,
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
    pub fn check(&self,mask: u8) -> bool {
        (self.head & mask) == mask
    }
}
// struct TCPExtra {
//     dump: bool,
// }
#[derive(Default, Packet)]
struct TCPOption {
    kind: u8,
    len: u8,
    data: TCPOptionKind,
}

struct TCPOptionKindBlock {
    data: Vec<u8>,
}
#[derive(Default)]
enum TCPOptionKind {
    #[default]
    NOP,
    BLOCK(TCPOptionKindBlock),
}
impl Display for TCPOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("TCP Option - {}", self.kind()).as_str())
    }
}
impl TCPOption {
    fn kind(&self) -> String {
        format!(
            "Kind: {} ({})",
            tcp_option_kind_mapper(self.kind as u16),
            self.kind
        )
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
    len: u16,
    pub payload_len: u16,
    window: u16,
    pub crc: u16,
    urgent: u16,
    options: Option<TCPOptions>,
    pub state: TCPState,
    info: Option<TCPInfo>,
}

impl std::fmt::Display for TCP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!(
            "Transmission Control Protocol, Src Port: {}, Dst Port: {}, Seq: {}, Ack: {}, Len: {}",
            self.source_port, self.target_port, self.sequence, self.acknowledge, self.len
        ))
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
impl crate::files::InfoPacket for TCP {
    fn info(&self) -> String {
        let mut info = format!(
            "{} → {} {} Seq={} Ack={} Win={} Len={}",
            self.source_port,
            self.target_port,
            self.state.to_string(),
            self.sequence,
            self.acknowledge,
            self.window,
            self.len
        );
        match &self.info {
            Some(_info) => {
                match _info.detail {
                    TCPDetail::KEEPALIVE => {
                        info = format!("[{}] {}", "Keeplive", info)
                    },
                    TCPDetail::NOPREVCAPTURE => {
                        info = format!("[{}] {}", "no_previous_segment", info)
                    }
                    TCPDetail::RETRANSMISSION => {
                        info = format!("[{}] {}", "retransmission", info)
                    }
                    _ => {}
                }
            },
            _ => {}
        }
        info
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
                    return format!("Sequence Number : {} (raw: {})", self.sequence - _seq, self.sequence)
                }
                format!("Sequence Number (raw): {}", self.sequence)
            },
            None => {
                format!("Sequence Number (raw): {}", self.sequence)
            },
        }
    }
    fn acknowledge_desc(&self) -> String {
        match &self.info {
            Some(info) => {
                let _ack = info._ack;
                if _ack <= self.sequence {
                    return format!("Acknowlagde Number : {} (raw: {})", self.acknowledge-_ack, self.acknowledge)
                }
                format!("Acknowlagde Number (raw): {}", self.acknowledge)
            },
            None => {
                format!("Acknowlagde Number (raw): {}", self.acknowledge)
            },
        }
    }
    fn len_desc(&self) -> String {
        format!(
            "{:04b} .... = Header Length: 32 bytes ({})",
            self.len, self.len
        )
    }
}
pub struct TCPVisitor;

impl TCPVisitor {
    fn read_option(reader: &Reader) -> Result<PacketContext<TCPOption>> {
        let packet: PacketContext<TCPOption> = Frame::create_packet();
        let mut option = packet.get().borrow_mut();
        option.kind = packet.read_with_string(reader, Reader::_read8, TCPOption::kind)?;
        match option.kind {
            2 | 3 | 4 | 5 | 8 | 28 | 29 | 30 => {
                let len =
                    packet._read_with_format_string_rs(reader, Reader::_read8, "Length: {}")?;
                option.len = len;
                // let read = |reader: &Reader|  TCPOptionKind::BLOCK(TCPOptionKindBlock{data: reader.slice((len-2) as usize).to_vec()});
                // option.data = packet._read_with_format_string_rs(reader,Reader::_read8, "Length: {}")?;
                let raw = reader.slice((len - 2) as usize);
                let block = TCPOptionKindBlock { data: raw.to_vec() };
                option.data = TCPOptionKind::BLOCK(block);
            }
            _ => {}
        }
        drop(option);
        Ok(packet)
    }
    fn read_options(reader: &Reader, len: usize) -> Result<PacketContext<MultiBlock<TCPOption>>> {
        let packet: PacketContext<MultiBlock<TCPOption>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let start = reader.cursor();
        let end = start + (len -5) * 4;
        while reader.cursor() < end {
            let item = packet.read_with_field(reader, TCPVisitor::read_option, None)?;
            p.push(item);
        }
        drop(p);
        Ok(packet)
    }
}

impl crate::files::Visitor for TCPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let ip_packet = frame.get_ip();
        let unwap = ip_packet.deref().borrow();
        let total = unwap.payload_len();
        let _start = reader.left()? as u16;
        let packet: PacketContext<TCP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.source_port =
            packet.read_with_string(reader, Reader::_read16_be, Description::source_port)?;
        p.target_port =
            packet.read_with_string(reader, Reader::_read16_be, Description::target_port)?;
        p.sequence = packet.read_with_string(reader, Reader::_read32_be, TCP::sequence_desc)?;
        p.acknowledge =
            packet.read_with_string(reader, Reader::_read32_be, TCP::acknowledge_desc)?;
        let head = packet.read_with_string(reader, Reader::_read16_be, TCP::len_desc)?;
        p.window = packet._read_with_format_string_rs(reader, Reader::_read16_be, "Window: {}")?;
        p.crc = packet._read_with_format_string_rs(reader, Reader::_read16_be, "Checksum: {}")?;
        p.urgent =
            packet._read_with_format_string_rs(reader, Reader::_read16_be, "Urgent Pointer: {}")?;
        p.set_head(head);
        let len = p.len;
        if len > 5 {
            let read = |reader: &Reader| TCPVisitor::read_options(reader, len as usize);
            let options = packet.read_with_field(reader, read, Some("Options".into()))?;
            p.options = Some(options);
        }
        let left_size = reader.left().unwrap_or(0) as u16;
        p.payload_len = left_size;
        if _start > total {
            p.payload_len = total + left_size - _start;
        }
        packet.read_txt(
            reader,
            reader.cursor(),
            p.payload_len.into(),
            format!("TCP payload ({} bytes)", left_size),
        );
        let info = frame.update_tcp(p.deref());
        p.info = Some(info);
        drop(p);
        frame.add_element(super::ProtocolData::TCP(packet));

        let is_http = super::http::HTTPVisitor::check(reader);
        if is_http {
            return super::http::HTTPVisitor.visit(frame, reader);
        }
        Ok(())
        // let method = reader._read_space(10);
        // match method {
        //     Some(_method) => {
        //         return match _method.as_str() {
        //             "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH" => {
        //                 return super::http::HTTPVisitor.visit(frame, reader);
        //             },
        //             "HTTP/1.1" => super::http::HTTPVisitor.visit(frame, reader),
        //             _ => Ok(())
        //         }
        //     },
        //     _ => {
        //         Ok(())
        //     }
        // }
    }
}
