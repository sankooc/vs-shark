use std::{cell::RefMut, fmt::Formatter, ops::DerefMut, rc::Rc};

use anyhow::{bail, Result};
use log::info;
use pcap_derive::{Packet, NINFO};

use crate::{
    common::Reader,
    constants::{tls_cipher_suites_mapper, tls_content_type_mapper, tls_extension_mapper, tls_hs_message_type_mapper, tls_min_type_mapper},
    files::{Endpoint, Frame, Initer, PacketContext, Ref2, TCPPAYLOAD},
};

use super::ProtocolData;

#[derive(Default, Packet)]
pub struct TLS {
    records: Vec<Ref2<TLSRecord>>,
}
impl crate::files::InfoPacket for TLS {
    fn info(&self) -> String {
        let len = self.records.len();
        if len > 0 {
            let one = self.records.get(0).unwrap();
            one.borrow()._type()
        } else {
            String::from("TLS segment")
        }
    }

    fn status(&self) -> String {
        "info".into()
    }
}
#[derive(Default, Clone, Packet)]
pub struct TLSRecord {
    _type: u8,
    min: u8,
    len: u16,
    message: TLSRecorMessage,
}

const TLS_CCS_DESC: &str = "Change Cipher Spec Message";
const TLS_ALERT_DESC: &str = "Encrypted Alert";
const TLS_APPLICATION: &str = "Encrypted Application Data";
const TLS_HEARTBEAT: &str = "Encrypted HEARTBEAT";
impl TLSRecord {
    fn _type(&self) -> String {
        tls_content_type_mapper(self._type)
    }
    fn _type_desc(&self) -> String {
        format!("Content Type: {} ({})", self._type(), self._type)
    }
    fn version(&self) -> String {
        tls_min_type_mapper(self.min)
    }
    fn version_desc(&self) -> String {
        format!("Version: {} (0x03{:02x})", self.version(), self.min)
    }
    fn message(&self) -> String {
        match self.message {
            TLSRecorMessage::CHANGECIPHERSPEC => TLS_CCS_DESC.into(),
            TLSRecorMessage::ALERT => TLS_ALERT_DESC.into(),
            TLSRecorMessage::APPLICAION => TLS_APPLICATION.into(),
            TLSRecorMessage::HEARTBEAT => TLS_HEARTBEAT.into(),
            _ => String::from("Encrypted Message"),
        }
    }
}
impl std::fmt::Display for TLSRecord {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!(
            "{} Record Layer: {} Protocol: {}",
            self.version(),
            self._type(),
            self.message()
        ))
    }
}
impl std::fmt::Display for TLS {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Transport Layer Security")
    }
}
impl TLS {
    pub fn _check(head: &[u8]) -> Result<(bool, usize)> {
        let _type = head[0];
        let major = head[1];
        let min = head[2];
        let len = u16::from_be_bytes(head[3..5].try_into()?);
        let is_tls = _type > 19 && _type < 25 && major == 3 && min < 5;
        return Ok((is_tls, len as usize));
    }
    pub fn check(reader: &Reader) -> Result<(bool, usize)> {
        let left = reader.left()?;
        if left <= 5 {
            return Ok((false, 0));
        }
        let head = reader._slice(5);
        TLS::_check(head)
    }
}

#[derive(Default, Clone, Packet)]
struct CupherSuites{
    size: usize,
    suites: Vec<u16>
}
impl std::fmt::Display for CupherSuites {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Cipher Suites ({} suites)", self.size))
    }
}
impl CupherSuites{
    fn _type (data: u16) -> String {
        format!("Cipher Suite: {} ({:#06x})", tls_cipher_suites_mapper(data), data)
    }
    fn create(reader: &Reader) -> Result<PacketContext<Self>> {
        let packet: PacketContext<CupherSuites> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let len = packet.build_format(reader, Reader::_read16_be, "Cipher Suites Length: {}")?;
        let _len = len/2;
        for _ in 0.._len {
            let code = packet.build_fn(reader, Reader::_read16_be, CupherSuites::_type)?;
            p.suites.push(code);
        }
        drop(p);
        Ok(packet)
    }
}

#[derive(Default, Clone, Packet)]
struct CompressMethod{
    size: usize,
    methods: Vec<u8>
}
impl std::fmt::Display for CompressMethod {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Compression Methods")
    }
}
impl CompressMethod{
    fn create(reader: &Reader) -> Result<PacketContext<Self>> {
        let packet: PacketContext<CompressMethod> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let len = packet.build_format(reader, Reader::_read8, "Compression Methods Length: {}")?;
        for _ in 0..len {
            let code = packet.build_format(reader, Reader::_read8, "Compression Method:({})")?;
            p.methods.push(code);
        }
        drop(p);
        Ok(packet)
    }
}

#[derive(Default, Clone, Packet)]
struct ExtenstionPack{
    size: usize,
    items: Vec<Ref2<Extenstion>>
}
impl std::fmt::Display for ExtenstionPack {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Extenstions")
    }
}
impl ExtenstionPack{
    fn _type (data: u16) -> String {
        format!("Cipher Suite: {} ({:#06x})", tls_cipher_suites_mapper(data), data)
    }
    fn create(reader: &Reader) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let len = packet.build_format(reader, Reader::_read16_be, "Extensions Length: {}")?;
        let finish = reader.cursor() + len as usize;
        loop {
            if reader.cursor() >= finish {
                break;
            }
            let item = packet.build_packet(reader, Extenstion::create, None)?;
            p.items.push(item);
        }
        drop(p);
        Ok(packet)
    }
}
#[derive(Default, Clone, Packet)]
struct Extenstion{
    _type: u16,
    len: u16,
}
impl std::fmt::Display for Extenstion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Extension: {} (len={})", self._type(), self.len))
    }
}
impl Extenstion {
    fn _type(&self) -> String {
        tls_extension_mapper(self._type)        
    }
    fn _type_desc(&self) -> String {
        format!("Type: {} ({})", self._type(), self._type)
    }
    fn create(reader: &Reader) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read16_be, Extenstion::_type_desc)?;
        p.len = packet.build_format(reader, Reader::_read16_be, "Length: {}")?;
        reader.slice(p.len as usize);
        //todo exptext
        drop(p);
        Ok(packet)
    }
}

#[derive(Clone)]
struct HandshakeClientHello {
    random: Vec<u8>,
    session: Vec<u8>,
    ciper_suites: Ref2<CupherSuites>,
    compress_method: Ref2<CompressMethod>,
    extensions: Ref2<ExtenstionPack>,
}
fn hexlize (data: &[u8]) -> String{
    data.iter().map(|f| format!("{:02x}", f)).collect::<String>()
}
// impl HandshakeClientHello {
//     pub fn random(&self) -> String{
//         (&self.random).iter().map(|f| format!("{:02x}", f)).collect::<String>()
//     }
//     pub fn session(&self) -> String{
//         (&self.session).iter().map(|f| format!("{:02x}", f)).collect::<String>()
//     }
// }
#[derive(Clone)]
struct HandshakeServerHello {
    random: Vec<u8>,
    session: Vec<u8>,
    ciper_suite: u16,
    method: u8,
    extensions: Ref2<ExtenstionPack>,
}
#[derive(Default, Clone)]
enum HandshakeType {
    #[default]
    UNKNOWN,
    Encrypted,
    HELLOREQUEST,
    ClientHello(HandshakeClientHello),
    ServerHello(HandshakeServerHello),
    NewSessionTicket,
    EncryptedExtensions,
    Certificate,
    ServerKeyExchange,
    CertificateRequest,
    ServerHelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
}
#[derive(Default, Clone, Packet)]
pub struct HandshakeProtocol {
    _type: u8,
    len: u32,
    msg: HandshakeType
}
impl std::fmt::Display for HandshakeProtocol {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Transport Layer Security")
    }
}
impl HandshakeProtocol {
    fn _type(&self) -> String{
        tls_hs_message_type_mapper(self._type)
    }
}
#[derive(Default, Clone, Packet)]
pub struct TLSHandshake {
    items: Vec<Ref2<HandshakeProtocol>>,
}
impl std::fmt::Display for TLSHandshake {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self.items.first() {
            Some(_head) => {
                fmt.write_fmt(format_args!("Handshake Protocol: {}", _head.as_ref().borrow().to_string()))
            },
            None => {
                fmt.write_str("Handshake Protocol")
            }
        }
    }
}
#[derive(Default, Clone)]
pub enum TLSRecorMessage {
    #[default]
    UNKNOWN,
    CHANGECIPHERSPEC,
    ALERT,
    HANDSHAKE(TLSHandshake),
    APPLICAION,
    HEARTBEAT,
}

pub struct TLSVisitor;

impl TLSVisitor {
    fn read_tls(reader: &Reader) -> Result<PacketContext<TLSRecord>> {
        let packet: PacketContext<TLSRecord> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read8, TLSRecord::_type_desc)?;
        reader.read8()?;
        p.min = packet.build_lazy(reader, Reader::_read8, TLSRecord::version_desc)?;
        let len = packet.build_format(reader, Reader::_read16_be, "Length: {}")?;
        p.len = len;
        let finish = reader.cursor() + p.len as usize;

        let _read = |reader: &Reader| {
            reader.slice(len as usize);
            Ok(())
        };
        match p._type {
            20 => {
                p.message = TLSRecorMessage::CHANGECIPHERSPEC;
                packet.build_lazy(reader, _read, TLSRecord::message)?;
            }
            21 => {
                p.message = TLSRecorMessage::ALERT;
                packet.build_lazy(reader, _read, TLSRecord::message)?;
            }
            22 => {
                let handshake_packet: PacketContext<TLSHandshake> = Frame::create_packet();
                let mut handshake = handshake_packet.get().borrow_mut();
                let fetch_one = |reader: &Reader| -> Result<PacketContext<HandshakeProtocol>> {
                    let protocol_packet: PacketContext<HandshakeProtocol> = Frame::create_packet();
                    let mut p = protocol_packet.get().borrow_mut();
                    let head = reader.read32(true)?;
                    let head_type = (head >> 24 & 0xff) as u8;
                    let head_len = head & 0xffffff;
                    p._type = head_type;
                    p.len = head_len;
                    let _mes = tls_hs_message_type_mapper(head_type);
                    let current = reader.cursor();
                    if _mes == "unknown" || head_len == 0 {
                        p.msg = HandshakeType::Encrypted;
                    } else {
                        if finish >= current + (head_len as usize) {
                        } else {
                            p.msg = HandshakeType::Encrypted;
                            drop(p);
                            return Ok(protocol_packet);
                        }
                        if head_len as usize > reader.left()? {
                            p.msg = HandshakeType::Encrypted;
                            drop(p);
                            return Ok(protocol_packet);
                        }
                        let h_type_desc = format!("Handshake Type: {} ({})",tls_hs_message_type_mapper(head_type),head_type);
                        let h_len_desc = format!("Length: {}",head_len);
                        let _finish = reader.cursor() + head_len as usize;

                        match head_type {
                            1 => {
                                reader.read8()?;
                                let min = reader.read8()?;
                                let version_desc = format!("Version: {} (0x03{})", tls_min_type_mapper(min), min);
                                protocol_packet._build(reader, reader.cursor() - 2, 2, version_desc);
                                let random = reader.slice(0x20);
                                protocol_packet._build(reader, reader.cursor() - 0x20, 0x20, format!("Random: {}", hexlize(random)));
                                let session_len = protocol_packet.build_format(reader, Reader::_read8, "Session ID Length: {}")?;
                                let session = reader.slice(session_len as usize);
                                protocol_packet._build(reader, reader.cursor() - session_len as usize, session_len as usize, format!("Session: {}", hexlize(session)));
                                let ciper_suites: Ref2<CupherSuites> = protocol_packet.build_packet(reader, CupherSuites::create, None)?;
                                let compress_method = protocol_packet.build_packet(reader, CompressMethod::create, None)?;
                                let extensions = protocol_packet.build_packet(reader, ExtenstionPack::create, None)?;
                                p.msg = HandshakeType::ClientHello(HandshakeClientHello{random: random.to_vec(), session: session.to_vec(), ciper_suites,compress_method, extensions});
                            },
                            2 => {
                                reader.read8()?;
                                let min = reader.read8()?;
                                let version_desc = format!("Version: {} (0x03{})", tls_min_type_mapper(min), min);
                                protocol_packet._build(reader, reader.cursor() - 2, 2, version_desc);
                                let random = reader.slice(0x20);
                                protocol_packet._build(reader, reader.cursor() - 0x20, 0x20, format!("Random: {}", hexlize(random)));
                                let session_len = protocol_packet.build_format(reader, Reader::_read8, "Session ID Length: {}")?;
                                let session = reader.slice(session_len as usize);
                                protocol_packet._build(reader, reader.cursor() - session_len as usize, session_len as usize, format!("Session: {}", hexlize(session)));
                                let ciper_suite = protocol_packet.build_fn(reader, Reader::_read16_be, CupherSuites::_type)?;
                                let method = protocol_packet.build_format(reader, Reader::_read8, "Compression Method: ({})")?;
                                let extensions = protocol_packet.build_packet(reader, ExtenstionPack::create, None)?;
                                p.msg = HandshakeType::ServerHello(HandshakeServerHello{random: random.to_vec(), session: session.to_vec(), ciper_suite, method, extensions});
                            },
                            _ => {
                                if len - head_len as u16 > 4 {
                                    info!("type:{}", _mes);
                                    info!(
                                        "handshake {:#10x} {},  {} {}",
                                        head, head_type, len, head_len
                                    );
                                }
                            }
                        }

                        if _finish > reader.cursor() {
                            reader.slice(_finish - reader.cursor());
                        }
                    }
                    drop(p);
                    return Ok(protocol_packet);
                };
                'outer: loop {
                    if reader.cursor() >= finish {
                        break;
                    }
                    let _rs = packet.build_packet(reader, fetch_one, None);
                    match &_rs {
                        Ok(_protocol) => {
                            let item = _protocol.clone();
                            handshake.items.push(item);
                        }
                        Err(_err) => break 'outer,
                    }
                }
                // packet.build(reader, _read, TLS_ALERT_DESC.into());
                // p.message = TLSRecorMessage::ALERT;
                drop(handshake);
            }
            23 => {
                p.message = TLSRecorMessage::APPLICAION;
                packet.build_lazy(reader, _read, TLSRecord::message)?;
            }
            _ => {
                packet.build_lazy(reader, _read, TLSRecord::message)?;
            }
        }
        if finish > reader.cursor() {
            reader.slice(finish - reader.cursor());
        }
        drop(p);
        Ok(packet)
    }
}

fn proc(
    frame: &Frame,
    reader: &Reader,
    packet: &PacketContext<TLS>,
    p: &mut TLS,
    ep: &mut Endpoint,
) -> Result<()> {
    loop {
        let left_size = reader.left()?;
        if left_size == 0 {
            //TODO FLUSH SEGMENT
            break;
        }
        let (is_tls, _len) = TLS::check(reader)?;
        if is_tls {
            if left_size >= _len + 5 {
                let item = packet.build_packet(reader, TLSVisitor::read_tls, None)?;
                p.records.push(item);
                // info!("comple");
            } else {
                let left_data = reader.slice(left_size);
                ep.add_segment(frame, TCPPAYLOAD::TLS, left_data);
                break;
            }
        } else {
            // info!("frame: {} left {}", frame.summary.borrow().index, reader.left()?);
            break;
        }
    }
    Ok(())
}
impl TLSVisitor {
    pub fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<TLS> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();

        let _info = frame.get_tcp_info()?;
        let mut ep = _info.as_ref().borrow_mut();
        let _len = reader.left()?;
        let _reader = reader;

        // info!("tls frame {}", frame.summary.borrow().index);
        // if frame.summary.borrow().index == 95 {
        //     println!("")
        // }
        match ep._seg_type {
            TCPPAYLOAD::TLS => {
                let head = ep.get_segment()?;
                let seg_length = head.len();
                let (_, len) = TLS::_check(&head[0..5])?;
                let data = reader.slice(_len);
                if len + 5 > seg_length + _len {
                    ep.add_segment(frame, TCPPAYLOAD::TLS, data);
                    let content = format!("TLS Segments {} bytes", _len);
                    packet._build(reader, reader.cursor(), _len, content);
                } else {
                    let mut _data = ep.take_segment();
                    _data.extend_from_slice(data);
                    let _reader = Reader::new_raw(Rc::new(_data));
                    proc(frame, &_reader, &packet, p.deref_mut(), ep.deref_mut())?;
                }
                // return None;
            }
            TCPPAYLOAD::NONE => {
                proc(frame, reader, &packet, p.deref_mut(), ep.deref_mut())?;
            }
        }
        let _len = p.records.len();
        drop(ep);
        drop(p);
        // if _len > 0 {
        frame.add_element(ProtocolData::TLS(packet));
        // }
        Ok(())
    }
}
