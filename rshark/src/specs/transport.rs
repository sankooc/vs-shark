use std::fmt::Display;

use pcap_derive::Packet;

use crate::common::{ContainProtocol, Description, PlayloadPacket, PortablePacket};
use crate::files::Visitor;
use crate::{
    common::{Protocol, Reader},
    files::{Frame, Initer, PacketContext},
};

fn execute(source: u16, target: u16, frame: &Frame, reader: &Reader) {
    match source {
        53 => return super::application::DNSVisitor.visit(frame, reader),
        _ => (),
    }
    match target {
        53 => return super::application::DNSVisitor.visit(frame, reader),
        _ => (),
    }
}

#[derive(Default, Packet)]
pub struct UDP {
    protocol: Protocol,
    source_port: u16,
    target_port: u16,
    len: u16,
    crc: u16,
}

impl PortablePacket for UDP {
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
impl UDP {
    fn _info(&self) -> String {
        return self.to_string();
    }
    fn _summary(&self) -> String {
        return self.to_string();
    }
}
pub struct UDPVisitor;

impl Visitor for UDPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<UDP> = Frame::create_packet(Protocol::UDP);
        let source = packet.read_with_string(reader, Reader::_read16_be, Description::source_port);
        let target = packet.read_with_string(reader, Reader::_read16_be, Description::target_port);
        let len = packet.read_with_string(reader, Reader::_read16_be, Description::packet_length);
        let crc = reader.read16(false);
        let playload_size = len - 8;
        packet.append_string(format!("UDP payload ({} bytes)", playload_size), reader.get_raw());
        let mut p = packet.get().borrow_mut();
        p.source_port = source;
        p.target_port = target;
        p.len = len;
        p.crc = crc;
        drop(p);
        frame.add_element(Box::new(packet));
        execute(source, target, frame, reader);
    }
}
