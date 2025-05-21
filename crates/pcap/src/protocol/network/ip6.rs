use std::net::Ipv6Addr;

use crate::{
    common::{concept::Field, core::Context, enum_def::Protocol, io::Reader, quick_hash, Frame},
    constants::ip_protocol_type_mapper,
    protocol::ip4_mapper,
    read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub struct Visitor {}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        if let Some(key) = frame.ptr {
            if let Some((_, source, target)) = ctx.ipv6map.get(&key) {
                return Some(format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target))
            }
        }
        None
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut reader = _reader.slice_as_reader(40)?;
        let data = reader.refer()?;
        let key = quick_hash(data);
        frame.ipv6 = Some(key);
        frame.ptr = Some(key);
        
        if let Some(enty) = ctx.ipv6map.get(&key) {
            Ok(ip4_mapper(enty.0))
        } else {
            reader.read32(true)?;
            reader.read16(true)?;
            let protocol_type = reader.read8()?;
            reader.read8()?; //hop
            let source =  Ipv6Addr::from(<[u8; 16]>::try_from(reader.slice(16, true)?)?);
            let target =  Ipv6Addr::from(<[u8; 16]>::try_from(reader.slice(16, true)?)?);
            ctx.ipv6map.insert(key, (protocol_type, source, target));
            Ok(ip4_mapper(protocol_type))
        }
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        reader.read32(true)?; //head
        read_field_format!(list, reader, reader.read16(true)?, "Payload Length: {}");
        let protocol_type = read_field_format_fn!(list, reader, reader.read8()?, t_protocol);
        read_field_format!(list, reader, reader.read8()?, "Hop Limit: {}");
        // let source = read_field_format!(list, reader, intern_ip6(reader)?, "Source Address: {}");
        // let target = read_field_format!(list, reader, intern_ip6(reader)?, "Destination Address: {}");
        // field.summary = intern(format!("Internet Protocol Version 4, Src: {}, Dst: {}", source.str, target.str));
        field.children = Some(list);
        Ok(ip4_mapper(protocol_type))
    }
}
