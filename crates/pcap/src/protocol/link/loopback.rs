use crate::{common::{concept::Field, enum_def::Protocol, io::Reader, core::Context, Frame}, read_field_format};
use anyhow::Result;


const SUMMARY: &'static str = "Null/Loopback";
pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, _: &Frame) -> Option<String>{
        Some(SUMMARY.to_string())
    }
    pub fn parse(_: &mut Context, _: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.read32(false)?;
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
        field.summary = SUMMARY.to_string();
        field.children = Some(list);
        if _next == 0x45 {
            Ok(Protocol::IP4)
        } else {
            Ok(Protocol::None)
        }
    }
}
