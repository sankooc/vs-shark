use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::base::PacketOpt;
use crate::constants::{ppp_lcp_option_type_mapper, ppp_lcp_type_mapper, ppp_type_mapper};
use crate::specs::ProtocolData;
use crate::{
    common::io::Reader,
    common::base::{Frame, PacketBuilder, PacketContext},
};
use crate::common::io::AReader;
use anyhow::{Ok, Result};
use std::fmt::Display;
use crate::common::FIELDSTATUS;



// #[derive(Default, Packet2, NINFO)]
// struct InternetControlProtocol {
//     _type: u8,
//     identifier: u8,
//     len: u16,
// }
// impl Display for InternetControlProtocol {
//     fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         _f.write_str("Protocol: Internet Protocol Control Protocol (0x8021)")
//     }
// }
// impl InternetControlProtocol {
//     fn _type(&self) -> String {
//         ppp_lcp_type_mapper(self._type)
//     }
//     fn _type_desc(&self) -> String {
//         format!("Code: {} ({})", self._type(), self._type)
//     }
//     fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<usize>) -> Result<()> {
//         p._type = packet.build_lazy(reader, Reader::_read8, Some("ppp.lcp.type"), InternetControlProtocol::_type_desc)?;
//         p.identifier = packet.build_format(reader, Reader::_read8, Some("ppp.lcp.identifier"), "Identifier: {}")?;
//         p.len = packet.build_format(reader, Reader::_read16_be, None,"Length: {}")?;
//         let left = p.len - 4;
//         reader._move(left as usize);
//         Ok(())
//     }
// }



#[derive(Default, Packet2, NINFO)]
struct LinkControlProtocol {
    _type: u8,
    identifier: u8,
    len: u16,
}
impl Display for LinkControlProtocol {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("")
    }
}
impl LinkControlProtocol {
    fn _type(&self) -> String {
        ppp_lcp_type_mapper(self._type)
    }
    fn _type_desc(&self) -> String {
        format!("Code: {} ({})", self._type(), self._type)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, ptype: Option<usize>) -> Result<()> {
        let option_type = ptype.unwrap();
        p._type = packet.build_lazy(reader, Reader::_read8, Some("ppp.lcp.type"), LinkControlProtocol::_type_desc)?;
        p.identifier = packet.build_format(reader, Reader::_read8, Some("ppp.lcp.identifier"), "Identifier: {}")?;
        p.len = packet.build_format(reader, Reader::_read16_be, None,"Length: {}")?;
        let left = p.len - 4;
        if option_type != 0xc021 {
            reader._move(left as usize);
        } else {
            if left == 4 {
                packet.build_format(reader, Reader::_read32_be, None,"Magic Number: {}")?;
            } else if left < 4 {
                reader._move(left as usize);
            } else {
                loop {
                    packet.build_fn(reader, Reader::_read8, Some("ppp.lcp.option.type"), ppp_lcp_option_type_mapper)?;
                    let option_size = reader.read8()?;
                    if option_size < 2 {
                        break;
                    }
                    if !reader._move((option_size - 2) as usize) {
                        break;
                    }
                    if reader.left() < 4 {
                        break;
                    }
                    
                }
            }
        }
        Ok(())
    }
}

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
        p.code = packet.build_lazy(reader, Reader::_read8, Some("ppp.code"), PPPoESS::code)?;
        p.session_id = packet.build_lazy(reader, Reader::_read16_be,Some("ppp.session.id"), PPPoESS::session_id)?;
        p.payload = packet.build_lazy(reader, Reader::_read16_be, Some("ppp.pload"),PPPoESS::payload)?;
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, Some("ppp.ptype"),PPPoESS::ptype)?;
        let type_desc = p.ptype();
        let decs = format!("Protocol: {} ({:#06x})", type_desc, p.ptype);
        match &p.ptype {
            33 | 87 => {},//ip
            0xc021 | 0x8021 | 0xc023 | 0x8057 => {
                packet.build_packet(reader, LinkControlProtocol::create, Some(p.ptype as usize), Some(decs))?;
            },
            _ => {}
        }
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
        ppp_type_mapper(self.ptype).into()
    }
}
#[derive(Visitor3)]
pub struct PPPoESSVisitor;
impl PPPoESSVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = PPPoESS::create(reader, None)?;
        let p = packet.get();
        let code = p.borrow().code;
        let ptype = p.borrow().ptype;
        if code == 0 {
            return match ptype {
                33 => Ok((ProtocolData::PPPoES(packet), "ipv4")),
                87 => Ok((ProtocolData::PPPoES(packet), "ipv6")),
                _ => Ok((ProtocolData::PPPoES(packet), "none")),
            };
        }
        Ok((ProtocolData::PPPoES(packet), "none"))
    }
}



#[derive(Default, Packet2, NINFO)]
pub struct PPPoED {
    version: u8,
    _type: u8,
    code: u8,
    session_id: u16,
    payload: u16,
}
impl Display for PPPoED {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("PPP-over-Ethernet Discovery")
    }
}

impl PPPoED {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<usize>) -> Result<()> {
        let head = reader.read8()?;
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.build_lazy(reader, Reader::_read8, Some("ppp.code"), PPPoED::code)?;
        p.session_id = packet.build_lazy(reader, Reader::_read16_be,Some("ppp.session.id"), PPPoED::session_id)?;
        p.payload = packet.build_lazy(reader, Reader::_read16_be, Some("ppp.pload"),PPPoED::payload)?;
        Ok(())
    }
}

impl PPPoED {
    fn code(&self) -> String {
        format!("Code: Session Data ({:#04x})", self.code)
    }
    fn session_id(&self) -> String {
        format!("Session ID: {:#06x}", self.session_id)
    }
    fn payload(&self) -> String {
        format!("Payload Length: {}", self.payload)
    }
}

#[derive(Visitor3)]
pub struct PPPoEDVisitor;
impl PPPoEDVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = PPPoED::create(reader, None)?;
        Ok((ProtocolData::PPPoED(packet), "none"))
    }
}