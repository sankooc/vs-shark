use crate::{common::{concept::Field, enum_def::Protocol, io::Reader, core::Context, Frame}, read_field_format};
use anyhow::Result;

pub struct Visitor {}

impl Visitor {
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.read32(false)?;
        // frame.info.info = "Null/Loopback";
        let _next = reader.next()?;
        if _next == 0x45 {
            Ok(Protocol::IP4)
        } else {
            Ok(Protocol::None)
        }
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        read_field_format!(list, reader, reader.read32(false)?, "Family: {}");
        let _next = reader.next()?;
        field.summary = "Null/Loopback";
        field.children = Some(list);
        if _next == 0x45 {
            Ok(Protocol::IP4)
        } else {
            Ok(Protocol::None)
        }
    }
}
