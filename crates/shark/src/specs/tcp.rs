use std::{
    fmt::{Display, Formatter, Write},
    ops::Deref,
};

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::{
        base::{BitFlag, BitType, Context, FlagData, Frame, FrameRefer, PacketContext, TCPDetail, TCPInfo, TCPSegment, TCPSegments},
        io::{AReader, Reader},
        Description, MultiBlock, PortPacket, Ref2, FIELDSTATUS,
    },
    constants::tcp_option_kind_mapper,
};

use super::ProtocolData;


struct Flag;

impl FlagData<u16> for Flag {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => Some((0xfe00, BitType::ONEoF(vec![(0x0, "Reverved: Not Set")]))),
            1 => Some((0x0100, BitType::ABSENT("Accurate ECN: set", "Accurate ECN: Not Set"))),
            2 => Some((0x0080, BitType::ABSENT("Congestion Window Reduced: set", "Congestion Window Reduced: Not Set"))),
            3 => Some((0x0040, BitType::ABSENT("ECN-Echo: set", "Echo: Not Set"))),
            4 => Some((0x0020, BitType::ABSENT("Urgent: set", "Urgent: Not Set"))),
            5 => Some((0x0010, BitType::ABSENT("Acknowledgment: set", "Acknowledgment: Not Set"))),
            6 => Some((0x0008, BitType::ABSENT("Push: set", "Push: Not Set"))),
            7 => Some((0x0004, BitType::ABSENT("Reset: set", "Reset: Not Set"))),
            8 => Some((0x0002, BitType::ABSENT("Syn: set", "Syn: Not Set"))),
            9 => Some((0x0001, BitType::ABSENT("Fin: set", "Fin: Not Set"))),
            _ => None,
        }
    }

    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("Flags: {:#04x}", value).as_str());
    }

    fn summary_ext(_: &mut String, _: &str, _: bool) {}
}


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
    flag: u16,
    head: u16,
    pub len: u16,
    pub payload_len: u16,
    window: u16,
    pub crc: u16,
    urgent: u16,
    pub options: Option<TCPOptions>,
    pub state: TCPState,
    pub info: Option<TCPInfo>,
    pub frame_refer: Ref2<FrameRefer>,
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
    // fn len_desc(&self) -> String {
    //     format!("{:04b} .... = Header Length: 32 bytes ({})", self.len, self.len)
    // }
    fn flag(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<Flag>(self.flag)
    }
    fn segments(&self) -> Option<PacketContext<TCPSegments>> {
        let reff = self.frame_refer.as_ref().borrow();
        if let Some(refs) = &reff.segments {
            let packet = Frame::_create(refs.clone());

            let p = packet.get().borrow();
            let mut _count = 0;
            for item in p.items.iter() {
                let TCPSegment { index, size } = item;
                let txt = format!("[Frame: {}, payload: {}-{} ({} bytes)]", *index, _count, _count + *size - 1, *size);
                packet.build_txt(txt);
                _count += *size;
            }
            packet.build_txt(format!("[Segment count: {}]", p.items.len()));
            drop(p);
            return Some(packet);
        }
        None
    }
}
pub struct TCPVisitor;

impl TCPVisitor {
    fn read_option(reader: &Reader, _: Option<()>) -> Result<PacketContext<TCPOption>> {
        let packet: PacketContext<TCPOption> = Frame::create_packet();
        let mut option = packet.get().borrow_mut();
        option.kind = packet.build_lazy(reader, Reader::_read8, Some("tcp.option.type"), TCPOption::kind)?;
        match option.kind {
            5 | 29 | 30 => {
                let len = packet.build_format(reader, Reader::_read8, Some("tcp.option.len"), "Length: {}")?;
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
                packet.build_format(reader, Reader::_read8, None, "Length: {}")?;
                let value = packet.build_format(reader, Reader::_read16_be, Some("tcp.mss.len"), "MSS Value: {}")?;
                option.data = TCPOptionKind::MSS(value);
            }
            3 => {
                packet.build_format(reader, Reader::_read8, None, "Length: {}")?;
                let value = packet.build_format(reader, Reader::_read8, Some("tcp.shift.count"), "Shift count: {}")?;
                option.data = TCPOptionKind::SCALE(value);
            }
            4 => {
                packet.build_format(reader, Reader::_read8, Some("tcp.sack.len"), "Length: {}")?;
                option.data = TCPOptionKind::SACK;
            }
            8 => {
                let len = packet.build_format(reader, Reader::_read8, None, "Length: {}")?;
                match len {
                    10 => {
                        let sender = packet.build_format(reader, Reader::_read32_be, Some("tcp.option.sender"), "sender: {}")?;
                        let reply = packet.build_format(reader, Reader::_read32_be, Some("tcp.option.reply"), "reply: {}")?;
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
                packet.build_format(reader, Reader::_read8, None, "Length: {}")?;
                let value = packet.build_fn(reader, Reader::_read16_be, None, TCPUserTimeout::desc)?;
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
    fn visit(&self, frame: &mut Frame, _ctx: &mut Context, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let _start = reader.left() as u16;
        let packet: PacketContext<TCP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.frame_refer = frame.refer.clone();
        p.source_port = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.source.port"), Description::source_port)?;
        p.target_port = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.target.port"), Description::target_port)?;
        p.sequence = packet.build_lazy(reader, Reader::_read32_be, Some("tcp.sequence"), TCP::sequence_desc)?;
        p.acknowledge = packet.build_lazy(reader, Reader::_read32_be, Some("tcp.acknowledge"), TCP::acknowledge_desc)?;
        // let head = packet.build_lazy(reader, Reader::_read16_be, Some("tcp.head.len"), TCP::len_desc)?;
        let head= packet.build_packet_lazy(reader, Reader::_read16_be, None, TCP::flag)?;
        p.flag = head;
        p.window = packet.build_format(reader, Reader::_read16_be, Some("tcp.window.size"), "Window: {}")?;
        p.crc = packet.build_format(reader, Reader::_read16_be, None, "Checksum: {}")?;
        p.urgent = packet.build_format(reader, Reader::_read16_be, None, "Urgent Pointer: {}")?;
        p.set_head(head);
        let len = p.len;
        if len > 5 {
            let options = packet.build_packet(reader, TCPVisitor::read_options, Some(len as usize), Some("Options".into()))?;
            p.options = Some(options);
        }
        let left_size = reader.left() as u16;
        p.payload_len = left_size;
        let ip_packet = frame.get_ip();
        let unwap = ip_packet.deref().borrow();
        let _total = unwap.payload_len();
        if let Some(total) = _total {
            if _start > total {
                p.payload_len = total + left_size - _start;
            }
        }
        if left_size > 0 {
            packet._build(reader, reader.cursor(), p.payload_len.into(), Some(("tcp.playload.len", p.payload_len.to_string().leak())), format!("TCP payload ({} bytes)", left_size));
        }
        packet.build_packet_no_position_lazy(TCP::segments);
        frame.add_tcp(packet._clone_obj());
        drop(p);
        Ok((ProtocolData::TCP(packet), "none"))
    }
}
