use crate::common::{concept::Field, enum_def::Protocol, io::Reader, Context, Frame};

use anyhow::Result;

pub struct DefaultParser {}

impl DefaultParser {
    pub fn parse(_: &mut Frame, _: &mut Reader) -> Result<Protocol> {
        // let mut fe = ProtocolElement::new(Protocol::None);
        // fe.element.title = intern("Unkown data packet".to_string());
        // fe.element.position = Some(range64(reader.range.clone()));
        // let fe = FieldElement::create(intern("Unkown data packet".to_string()), None);
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, _: &mut Reader) -> Result<Protocol> {
        field.summary = "parse failed";
        // TODO 
        Ok(Protocol::None)
    }
}
