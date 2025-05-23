use crate::{
    common::{concept::Field, core::Context, enum_def::{AddressField, Protocol}, io::Reader, quick_hash, Frame},
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
        if let AddressField::IPv6(key) = &frame.ip_field {
            if let Some((_, source, target)) = ctx.ipv6map.get(key) {
                return Some(format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target));
            }
        }
        None
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut reader = _reader.slice_as_reader(40)?;
        let data = reader.refer()?;
        let key = quick_hash(data);
        frame.ip_field = AddressField::IPv6(key);

        if let Some(enty) = ctx.ipv6map.get(&key) {
            Ok(ip4_mapper(enty.0))
        } else {
            reader.read32(true)?;
            frame.iplen = reader.read16(true)?;
            let protocol_type = reader.read8()?;
            reader.read8()?; //hop
            let source = reader.read_ip6()?;
            let target = reader.read_ip6()?; 
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
        let source = read_field_format!(list, reader, reader.read_ip6()?, "Source Address: {}");
        let target = read_field_format!(list, reader, reader.read_ip6()?, "Destination Address: {}");
        field.summary = format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target);
        field.children = Some(list);
        Ok(ip4_mapper(protocol_type))
    }
}
