
use anyhow::Result;
use crate::{cache::intern, common::{io::Reader, FieldElement, Protocol, ProtocolElement }, field_back_format, read_field_format, with_range};

pub struct EthernetVisitor {
    
}

pub fn read_mac(reader: &mut Reader) -> Result<&'static str> {
    let data = reader.slice(6, true)?;
    let str = (data)
            .iter()
            .map(|x| format!("{:02x?}", x))
            .collect::<Vec<String>>()
            .join(":");
    Ok(intern(&str))
}

impl EthernetVisitor {
    pub fn parse(reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::Ethernet);
        
        let mut list = Vec::new();
        let source = read_field_format!(list, reader, read_mac(reader)?, "Source: {}");
        let target = read_field_format!(list, reader, read_mac(reader)?, "Destination: {}");
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
            field_back_format!(list, reader,2, format!("Length: {}", ptype).as_str());
        } else {
            field_back_format!(list, reader,2, format!("Type: {} ({:#06x})", ptype, ptype).as_str());
        }
        fe.element.title = intern(format!("Ethernet II, Src: {}, Dst: {}", source, target).as_str());
        fe.element.children = Some(list);

        Ok(("none", fe))
    }
}