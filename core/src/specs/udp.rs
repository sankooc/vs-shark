use std::fmt::Display;

use pcap_derive::{Packet, NINFO};
use anyhow::Result;

use crate::common::{Description, PlayloadPacket, PortPacket};
use crate::files::Visitor;
use crate::{
    common::Reader,
    files::{Frame, Initer, PacketContext},
};

fn execute(source: u16, target: u16, frame: &Frame, reader: &Reader)  -> Result<()>{
    match source {
        53 => return super::dns::DNSVisitor.visit(frame, reader),
        _ => (),
    }
    match target {
        53 => return super::dns::DNSVisitor.visit(frame, reader),
        _ => (),
    }
    Ok(())
}

#[derive(Default, Packet, NINFO)]
pub struct UDP {
    source_port: u16,
    target_port: u16,
    len: u16,
    crc: u16,
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
        f.write_str(
            format!(
                "User Datagram Protocol, Src Port: {}, Dst Port: {}",
                self.source_port, self.target_port
            )
            .as_str(),
        )?;
        Ok(())
    }
}
pub struct UDPVisitor;

impl Visitor for UDPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<UDP> = Frame::create_packet();
        let source = packet.build_lazy(reader, Reader::_read16_be, Description::source_port)?;
        let target = packet.build_lazy(reader, Reader::_read16_be, Description::target_port)?;
        let len = packet.build_lazy(reader, Reader::_read16_be, Description::packet_length)?;
        let crc = reader.read16(false)?;
        let playload_size = len - 8;
        packet._build(
            reader,
            reader.cursor(),
            playload_size as usize,
            format!("UDP payload ({} bytes)", playload_size)
        );
        let mut p = packet.get().borrow_mut();
        p.source_port = source;
        p.target_port = target;
        p.len = len;
        p.crc = crc;
        drop(p);
        frame.add_element(super::ProtocolData::UDP(packet));
        execute(source, target, frame, reader)
    }
}
