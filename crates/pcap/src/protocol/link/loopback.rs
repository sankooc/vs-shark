use anyhow::Result;
use crate::{cache::intern, common::{enum_def::Protocol, io::Reader, Frame, ProtocolElement}, read_field_format};



pub struct Visitor {

}

impl Visitor {
    
    pub fn parse(_: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::Loopback);
        let mut list = vec![];
        read_field_format!(list, reader, reader.read32(false)?, "Family: {}");
        
        let mut next = "none";
        intern(next.to_string());
        let head = reader.next()?;
        if head == 0x45 {
            next = "ipv4";
        }
        fe.element.children = Some(list);
        Ok((next, fe))
    }
}