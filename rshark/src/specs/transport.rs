use std::fmt::Display;

use crate::common::{Description, PlayloadPacket, PortablePacket};
use crate::files::{Field, Visitor};
use crate::{
    common::{Protocol, Reader},
    files::{Frame, Initer, PacketContext},
};

#[derive(Default)]
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
impl Initer<UDP> for UDP {
    fn new() -> UDP {
        UDP {
            protocol: Protocol::UDP,
            ..Default::default()
        }
    }

    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    fn info(&self) -> String {
        self.to_string().clone()
    }
}

pub struct UDPVisitor;

impl Visitor for UDPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let mut packet: PacketContext<UDP> = Frame::create_packet();
        let source = packet.read(reader, Reader::_read16_be, Some(Description::source_port));
        let target = packet.read(reader, Reader::_read16_be, Some(Description::target_port));
        let len = packet.read(reader, Reader::_read16_be, Some(Description::packet_length));
        let crc = packet.read(reader, Reader::_read16_be, None);//checksum
        let playload_size = len - 8;
        packet.read_empty(reader, playload_size as usize, Some(|start, size, val| Field::new(start, size, format!("UDP payload ({} bytes)", val.len - 8))));
        
        let p = &mut packet.val;
        p.source_port = source;
        p.target_port = target;
        p.len = len;
        p.crc = crc;
        frame.add_element(Box::new(packet));
    }
}
