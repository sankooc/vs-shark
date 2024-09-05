use std::{cell::RefMut, fmt::Formatter, ops::DerefMut, rc::Rc};

use anyhow::Result;
use log::info;
use pcap_derive::{Packet, NINFO};

use crate::{
    common::Reader,
    constants::{tls_content_type_mapper, tls_min_type_mapper},
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
}
impl std::fmt::Display for TLSRecord {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Transport Layer Security")
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

enum HandshakeTYPE {
    HELLOCIENT,
}

#[derive(Default, Clone)]
pub struct TLSHandshake {
    head: u32,
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

const TLS_CCS_DESC: &str = "Change Cipher Spec Message";
const TLS_ALERT_DESC: &str = "Encrypted Alert";
const TLS_APPLICATION: &str = "Encrypted Application Data";

pub struct TLSVisitor;

impl TLSVisitor {
    fn read_tls(reader: &Reader) -> Result<PacketContext<TLSRecord>> {
        let packet: PacketContext<TLSRecord> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read8, TLSRecord::_type_desc)?;
        reader.read8()?;
        p.min = packet.build_lazy(reader, Reader::_read8, TLSRecord::version_desc)?;
        p.len = packet.build_format(reader, Reader::_read16_be, "Length: {}")?;
        let finish = reader.cursor() + p.len as usize;

        let _read = |reader: &Reader| {
            reader.slice(p.len as usize);
        };
        match p._type {
            20 => {
                packet.build(reader, _read, TLS_CCS_DESC.into());
                p.message = TLSRecorMessage::CHANGECIPHERSPEC;
            }
            21 => {
                packet.build(reader, _read, TLS_ALERT_DESC.into());
                p.message = TLSRecorMessage::ALERT;
            }
            22 => {
                // packet.build(reader, _read, TLS_ALERT_DESC.into());
                // p.message = TLSRecorMessage::ALERT;
            }
            23 => {
                packet.build(reader, _read, TLS_APPLICATION.into());
                p.message = TLSRecorMessage::ALERT;
            }
            _ => {
                packet.build(reader, _read, "Encrypted Message".into());
            }
        }
        if finish > reader.cursor() {
            reader.slice(finish - reader.cursor());
        }
        drop(p);
        Ok(packet)
    }
}

fn proc(frame: &Frame, reader: &Reader, packet: &PacketContext<TLS>, p: &mut TLS, ep: &mut Endpoint)  -> Result<()> {
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
    };
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
            },
        }

        // if frame.summary.borrow().index == 562 {
        //     println!("")
        // }
        // loop {
        //     let left_size = reader.left()?;
        //     if left_size == 0 {
        //         //TODO FLUSH SEGMENT
        //         // info!("complete");
        //         break;
        //     }
        //     let (is_tls, _len) = TLS::check(reader)?;
        //     if is_tls {
        //         if left_size >= _len + 5 {
        //             let item = packet.build_packet(reader, TLSVisitor::read_tls, None)?;
        //             p.records.push(item);
        //             // info!("comple");
        //         } else {
        //             let left_data = reader.slice(left_size);
        //             ep.add_segment(frame, TCPPAYLOAD::TLS, left_data);
        //             break;
        //         }
        //     } else {
        //         // info!("frame: {} left {}", frame.summary.borrow().index, reader.left()?);
        //         break;
        //     }
        // }
        let _len = p.records.len();
        drop(ep);
        drop(p);
        // if _len > 0 {
        frame.add_element(ProtocolData::TLS(packet));
        // }
        Ok(())
    }
}
