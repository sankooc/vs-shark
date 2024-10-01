pub mod ber;
pub mod extension;
pub mod handshake;
use std::{fmt::Formatter, ops::DerefMut, rc::Rc};

use crate::common::{base::Context, io::AReader, FIELDSTATUS};
use anyhow::Result;
use handshake::{HandshakeProtocol, HandshakeType};
use pcap_derive::{Packet, Packet2};

use super::ProtocolData;
use crate::{
    common::io::Reader,
    constants::{tls_content_type_mapper, tls_min_type_mapper},
    common::base::{Endpoint, Frame, PacketBuilder, PacketContext, PacketOpt, Visitor, TCPPAYLOAD},
};

#[derive(Default, Packet)]
pub struct TLS {
    records: Vec<TLSRecord>,
}
impl crate::common::base::InfoPacket for TLS {
    fn info(&self) -> String {
        let len = self.records.len();
        if len > 0 {
            let one = self.records.get(0).unwrap();
            one._type()
        } else {
            String::from("TLS segment")
        }
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
#[derive(Default, Packet2)]
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
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
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
                let pk = packet.build_packet(reader, TLSHandshake::create, Some(finish), None)?;
                p.message = TLSRecorMessage::HANDSHAKE(Rc::new(pk.take()));
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
        Ok(())
    }
}
impl std::fmt::Display for TLSRecord {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{} Record Layer: {} Protocol: {}", self.version(), self._type(), self.message()))
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

#[derive(Default, Packet2)]
pub struct TLSHandshake {
    items: Vec<HandshakeProtocol>,
}
impl TLSHandshake {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, opt: Option<PacketOpt>) -> Result<()> {
        let finish = opt.unwrap();
        'outer: loop {
            if reader.cursor() >= finish {
                break;
            }
            let _rs = packet.build_packet(reader, HandshakeProtocol::create, Some(finish), None);
            match &_rs {
                Ok(_protocol) => {
                    let item = _protocol.clone();
                    let reff = item.as_ref().borrow();
                    let _msg = &reff.msg;
                    match _msg {
                        HandshakeType::Encrypted => {
                            drop(reff);
                            break 'outer;
                        }
                        _ => {
                            drop(reff);
                            p.items.push(item.take());
                        }
                    }
                }
                Err(_err) => break 'outer,
            }
        }
        Ok(())
    }
}
impl std::fmt::Display for TLSHandshake {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self.items.first() {
            Some(_head) => fmt.write_fmt(format_args!("Handshake Protocol: {}", _head.to_string())),
            None => fmt.write_str("Handshake Protocol"),
        }
    }
}
#[derive(Default)]
pub enum TLSRecorMessage {
    #[default]
    UNKNOWN,
    CHANGECIPHERSPEC,
    ALERT,
    HANDSHAKE(Rc<TLSHandshake>),
    APPLICAION,
    HEARTBEAT,
}

pub struct TLSVisitor;

fn proc(frame: &Frame, reader: &Reader, packet: &PacketContext<TLS>, p: &mut TLS, ep: &mut Endpoint) -> Result<()> {
    loop {
        let left_size = reader.left()?;
        if left_size == 0 {
            //TODO FLUSH SEGMENT
            break;
        }
        let (is_tls, _len) = TLS::check(reader)?;
        if is_tls {
            if left_size >= _len + 5 {
                let item = packet.build_packet(reader, TLSRecord::create, None, None)?;
                let record = item.take();
                match &record.message {
                    TLSRecorMessage::HANDSHAKE(hs) => {
                        ep.handshake.push(hs.clone());
                    },
                    _ => {}
                };
                p.records.push(record);
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
impl Visitor for TLSVisitor {
    fn visit(&self, frame: &mut Frame, ctx: &mut Context, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<TLS> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();

        let _info = frame.get_tcp_info(true, ctx)?;
        let mut ep = _info.as_ref().borrow_mut();
        let _len = reader.left()?;
        let _reader = reader;
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
        Ok((ProtocolData::TLS(packet), "none"))
    }
}
