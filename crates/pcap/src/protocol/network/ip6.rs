use crate::{
    cache::{intern, intern_ip6},
    common::{concept::Field, enum_def::Protocol, io::Reader, Context, Frame},
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
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.read32(true)?;
        reader.read16(true)?;
        let protocol_type = reader.read8()?;
        reader.read8()?; //hop
        let source = intern_ip6(reader)?.to_string(); // source
        let target = intern_ip6(reader)?.to_string(); // target
        frame.info.source = intern(source);
        frame.info.dest = intern(target);
        frame.info.info = intern(format!("Internet Protocol Version 6, Src: {}, Dst: {}", &frame.info.source, &frame.info.dest));
        Ok(ip4_mapper(protocol_type))
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        reader.read32(true)?; //head
        read_field_format!(list, reader, reader.read16(true)?, "Payload Length: {}");
        let protocol_type = read_field_format_fn!(list, reader, reader.read8()?, t_protocol);
        read_field_format!(list, reader, reader.read8()?, "Hop Limit: {}");
        let source = read_field_format!(list, reader, intern_ip6(reader)?, "Source Address: {}");
        let target = read_field_format!(list, reader, intern_ip6(reader)?, "Destination Address: {}");
        field.summary = intern(format!("Internet Protocol Version 4, Src: {}, Dst: {}", source.str, target.str));
        field.children = Some(list);
        Ok(ip4_mapper(protocol_type))
    }
}
