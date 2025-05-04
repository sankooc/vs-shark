use crate::constants::ssl_type_mapper;
use crate::{
    cache::intern,
    common::{
        enum_def::Protocol,
        io::{read_mac, Reader},
        Frame, ProtocolElement,
    },
    constants::etype_mapper,
    protocol::enthernet_protocol_mapper,
    read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub struct SSLVisitor;

fn typedesc(_type: u16) -> String {
    format!("Packet Type: {}", ssl_type_mapper(_type))
}

fn ptype_str(ptype: u16) -> String {
    format!("Protocol: {} ({:#06x})", etype_mapper(ptype), ptype)
}

impl SSLVisitor {
    pub fn parse(frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::SSL);
        let mut list = vec![];
        let _type = read_field_format_fn!(list, reader, reader.read16(true)?, typedesc);
        read_field_format!(list, reader, reader.read16(true)?, "Link-layer address type: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Length: {}");
        let source = read_field_format!(list, reader, read_mac(reader)?, "Source MAC: {}");
        reader.forward(2);
        let ptype = read_field_format_fn!(list, reader, reader.read16(true)?, ptype_str);

        let info = intern(format!("Ethernet II, Src: {}", source));
        fe.element.title = info;
        frame.info.info = info;
        frame.info.source = source;
        fe.element.children = Some(list);
        Ok((enthernet_protocol_mapper(ptype), fe))
    }
}
