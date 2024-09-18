use pcap_derive::{Packet2, NINFO};

use crate::files::{PacketOpt, Visitor};
use crate::specs::ProtocolData;
use crate::{
    common::io::Reader,
    files::{Frame, Initer, PacketContext},
};
use crate::common::io::AReader;
use anyhow::{Ok, Result};
use std::fmt::Display;

#[derive(Default, Packet2, NINFO)]
pub struct PPPoESS {
    version: u8,
    _type: u8,
    code: u8,
    session_id: u16,
    payload: u16,
    ptype: u16,
}
impl Display for PPPoESS {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("PPP-over-Ethernet Session")
    }
}

impl PPPoESS {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<usize>) -> Result<()> {
        let head = reader.read8()?;
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.build_lazy(reader, Reader::_read8, PPPoESS::code)?;
        p.session_id = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::session_id)?;
        p.payload = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::payload)?;
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::ptype)?;
        Ok(())
    }
}

impl PPPoESS {
    fn code(&self) -> String {
        format!("Code: Session Data ({:#04x})", self.code)
    }
    fn session_id(&self) -> String {
        format!("Session ID: {:#06x}", self.session_id)
    }
    fn payload(&self) -> String {
        format!("Payload Length: {}", self.payload)
    }
    fn ptype(&self) -> String {
        match self.ptype {
            33 => "Protocol: Internet Protocol version 4 (0x0021)".into(),
            87 => "Protocol: Internet Protocol version 6 (0x0057)".into(),
            _ => format!("Unknown: {}", self.ptype),
        }
    }
}
pub struct PPPoESSVisitor;
impl Visitor for PPPoESSVisitor {
    fn visit(&self, _: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = PPPoESS::create(reader, None)?;
        let p = packet.get();
        let code = p.borrow().code;
        let ptype = p.borrow().ptype;
        if code == 0 {
            return match ptype {
                33 => Ok((ProtocolData::PPPoESS(packet), "ipv4")),
                87 => Ok((ProtocolData::PPPoESS(packet), "ipv6")),
                _ => Ok((ProtocolData::PPPoESS(packet), "none")),
            };
        }
        Ok((ProtocolData::PPPoESS(packet), "none"))
    }
}
