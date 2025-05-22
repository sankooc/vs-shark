use crate::common::concept::Field;
use crate::common::core::Context;
use crate::common::enum_def::InfoField;
use crate::common::{io::Reader};
use crate::{
    common::{
        enum_def::Protocol,
        Frame,
    },
};
use anyhow::Result;
// macro_rules! chc {
//     ($reader:expr, $byte_slice:expr) => {
//         {
//             if let Ok(stream) = $reader.preview($byte_slice.len()) {
//                 stream == $byte_slice
//             } else {
//                 false
//             }
//         }
//     };
// }

pub fn detect<'a>(reader: &'a Reader) -> bool {
    if reader.left() < 8 {
        return false;
    }
    let buffer = reader.preview(8).unwrap();

    if buffer.len() >= 4 {
        match &buffer[0..4] {
            b"GET " | b"POST" | b"PUT " | b"DELE" | b"HEAD" | b"OPTI" | b"PATC" | b"TRAC" | b"CONN" => return true,
            _ => (),
        }
    }
    match &buffer[0..7] {
        b"HTTP/1." => return true,
        _ => (),
    }
    // if &buffer[0..7] == b"HTTP/1." {
    //     return true;
    // }
    false
    // chc!(reader, b"GET ") || chc!(reader, b"POST ")
    //     || chc!(reader, b"HTTP/1.1 ") || chc!(reader, b"HTTP/1.0 ")
    //     || chc!(reader, b"PUT ") || chc!(reader, b"DELETE ")
    //     || chc!(reader, b"HEAD ") || chc!(reader, b"CONNECT ") || chc!(reader, b"OPTIONS ") || chc!(reader, b"TRACE ") || chc!(reader, b"PATCH ")
}

pub struct Visitor;
impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let InfoField::Http(data) = &frame.info_field {
            return Some(String::from_utf8_lossy(data).to_string());
        }
        None
    }
    pub fn parse(_: &Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        frame.tail = Protocol::HTTP;
        if let Some(pos) = reader.search_enter(300) {
            let data = reader.preview(pos)?.to_vec();
            frame.info_field = InfoField::Http(data);
            // frame._str = unsafe { String::from_utf8_unchecked(data) };
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        todo!()
    }
}
