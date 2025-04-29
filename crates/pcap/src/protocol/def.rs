
use anyhow::Result;
use crate::{cache::intern, common::{io::Reader, range64, Protocol, ProtocolElement}};
pub struct DefaultParser {
    
}

impl DefaultParser {
    pub fn parse(reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::None);
        fe.element.title = intern("Unkown data packet");
        fe.element.position = Some(range64(reader.range.clone()));
        Ok(("none", fe))
    }
}