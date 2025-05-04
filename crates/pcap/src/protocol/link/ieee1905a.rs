use anyhow::Result;
use crate::{common::{enum_def::Protocol, io::Reader, Frame, ProtocolElement}, read_field_format};



pub struct Visitor {

}    
impl Visitor {
    
    pub fn parse(_: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::SSL);
        let mut list = vec![];
        read_field_format!(list, reader, reader.read8()?, "Message version: {}");
        reader.forward(1);
        read_field_format!(list, reader, reader.read16(true)?, "Message type: ({})");
        read_field_format!(list, reader, reader.read16(true)?, "Message id: ({})");
        read_field_format!(list, reader, reader.read8()?, "Fragment id: ({})");
        fe.element.children = Some(list);
        Ok(("none", fe))
    }
}