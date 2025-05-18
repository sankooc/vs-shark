use crate::{
    common::{concept::Field, enum_def::Protocol, io::Reader, Context, Frame},
    read_field_format,
};
use anyhow::Result;

pub struct Visitor {}
impl Visitor {
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.forward(7);
        frame.info.info = "IEEE 1905.1a";
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        read_field_format!(list, reader, reader.read8()?, "Message version: {}");
        reader.forward(1);
        read_field_format!(list, reader, reader.read16(true)?, "Message type: ({})");
        read_field_format!(list, reader, reader.read16(true)?, "Message id: ({})");
        read_field_format!(list, reader, reader.read8()?, "Fragment id: ({})");
        field.summary = "IEEE 1905.1a";
        field.children = Some(list);
        // TODO 
        Ok(Protocol::None)
    }
}
