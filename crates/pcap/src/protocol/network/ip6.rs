use anyhow::Result;
use crate::{cache::{intern, intern_ip6}, common::{enum_def::Protocol, io::Reader, Frame, ProtocolElement}, constants::ip_protocol_type_mapper, protocol::ip4_mapper, read_field_format, read_field_format_fn};



pub struct Visitor {

}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    
    pub fn parse(frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::IP6);
        let mut list = vec![];
        reader.read32(true)?; //head
        read_field_format!(list, reader, reader.read16(true)?, "Payload Length: {}");
        let protocol_type = read_field_format_fn!(list, reader, reader.read8()?, t_protocol);
        read_field_format!(list, reader, reader.read8()?, "Hop Limit: {}");
        let source = read_field_format!(list, reader, intern_ip6(reader)?, "Source Address: {}");
        let target = read_field_format!(list, reader, intern_ip6(reader)?, "Destination Address: {}");
        
        frame.info.source = source.str;
        frame.info.dest = target.str;
        fe.element.title = intern(format!("Internet Protocol Version 4, Src: {}, Dst: {}", source.str, target.str));       
        fe.element.children = Some(list);
        Ok((ip4_mapper(protocol_type), fe))
    }
}