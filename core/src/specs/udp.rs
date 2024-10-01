use std::fmt::Display;

use anyhow::Result;
use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::io::{AReader, Reader};
use crate::common::{Description, PlayloadPacket, PortPacket};
use crate::common::base::{Frame, PacketContext, PacketOpt, PacketBuilder};
use super::ProtocolData;

use crate::common::FIELDSTATUS;

fn execute(source: u16, target: u16) -> &'static str {
    let pp = match source {
        53 => "dns",
        5353 => "mdns",
        137 => "nbns",
        1900 => "ssdp",
        _ => "none",
    };
    if pp == "none" {
        return match target {
            53 => "dns",
            5353 => "mdns",
            137 => "nbns",
            1900 => "ssdp",
            _ => "none",
        };
    }
    pp
}

#[derive(Default, Packet2, NINFO)]
pub struct UDP {
    source_port: u16,
    target_port: u16,
    len: u16,
    crc: u16,
}

impl UDP {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        let source = packet.build_lazy(reader, Reader::_read16_be, Description::source_port)?;
        let target = packet.build_lazy(reader, Reader::_read16_be, Description::target_port)?;
        let len = packet.build_lazy(reader, Reader::_read16_be, Description::packet_length)?;
        let crc = reader.read16(false)?;
        let playload_size = len - 8;
        packet._build(reader, reader.cursor(), playload_size as usize, format!("UDP payload ({} bytes)", playload_size));
        p.source_port = source;
        p.target_port = target;
        p.len = len;
        p.crc = crc;
        Ok(())
    }
}

impl PortPacket for UDP {
    fn source_port(&self) -> u16 {
        self.source_port
    }

    fn target_port(&self) -> u16 {
        self.target_port
    }
}

impl PlayloadPacket for UDP {
    fn len(&self) -> u16 {
        self.len
    }
}

impl Display for UDP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("User Datagram Protocol, Src Port: {}, Dst Port: {}", self.source_port, self.target_port).as_str())?;
        Ok(())
    }
}
#[derive(Visitor3)]
pub struct UDPVisitor;

impl UDPVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<UDP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let source = packet.build_lazy(reader, Reader::_read16_be, Description::source_port)?;
        let target = packet.build_lazy(reader, Reader::_read16_be, Description::target_port)?;
        let len = packet.build_lazy(reader, Reader::_read16_be, Description::packet_length)?;
        let crc = reader.read16(false)?;
        let playload_size = len - 8;
        packet._build(reader, reader.cursor(), playload_size as usize, format!("UDP payload ({} bytes)", playload_size));
        p.source_port = source;
        p.target_port = target;
        p.len = len;
        p.crc = crc;
        drop(p);
        Ok((super::ProtocolData::UDP(packet), execute(source, target)))
    }
}
