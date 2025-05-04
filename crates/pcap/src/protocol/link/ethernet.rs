
use anyhow::Result;
use crate::{cache::intern, common::{enum_def::Protocol, io::{read_mac, Reader}, Frame, ProtocolElement }, constants::etype_mapper, field_back_format, protocol::enthernet_protocol_mapper, read_field_format};

pub struct EthernetVisitor {
    
}

impl EthernetVisitor {
    pub fn parse(frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::Ethernet);
        
        let mut list = Vec::new();
        let source = read_field_format!(list, reader, read_mac(reader)?, "Source: {}");
        let target = read_field_format!(list, reader, read_mac(reader)?, "Destination: {}");
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
            //"IEEE 802.3 Ethernet"
            field_back_format!(list, reader,2, format!("Length: {}", ptype));
        } else {
            field_back_format!(list, reader,2, format!("Type: {} ({:#06x})", etype_mapper(ptype), ptype));
        }
        let info = intern(format!("Ethernet II, Src: {}, Dst: {}", source, target));
        fe.element.title = info;
        fe.element.children = Some(list);
        frame.info.info = info;
        frame.info.source = source;
        frame.info.dest = target;
        Ok((enthernet_protocol_mapper(ptype), fe))
    }
}