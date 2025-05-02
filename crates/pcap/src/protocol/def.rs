use crate::common::{enum_def::Protocol, Frame};

use anyhow::Result;
use crate::{cache::intern, common::{io::Reader, range64, ProtocolElement}};
pub struct DefaultParser {
    
}

impl DefaultParser {
    pub fn parse(_: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::None);
        fe.element.title = intern("Unkown data packet".to_string());
        fe.element.position = Some(range64(reader.range.clone()));
        Ok(("none", fe))
    }
}