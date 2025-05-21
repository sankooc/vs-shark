use crate::common::io::Reader;

macro_rules! chc {
    ($reader:expr, $byte_slice:expr) => {
        {
            if let Ok(stream) = $reader.preview($byte_slice.len()) {
                stream == $byte_slice
            } else {
                false
            }
        }
    };
}

pub fn detect<'a>(reader: &'a Reader) -> bool {
    if reader.left() < 8 {
        return false;
    }
    let buffer = reader.preview(8).unwrap();

    if buffer.len() >= 4 {
        match &buffer[0..4] {
            b"GET " | b"POST" | b"PUT " | b"DELE" | b"HEAD" | b"OPTI" 
            | b"PATC" | b"TRAC" | b"CONN" => return true,
            _ => ()
        }
    }
    if buffer.len() >= 7 && &buffer[0..7] == b"HTTP/1." {
        return true;
    }
    false
    // chc!(reader, b"GET ") || chc!(reader, b"POST ")
    //     || chc!(reader, b"HTTP/1.1 ") || chc!(reader, b"HTTP/1.0 ")
    //     || chc!(reader, b"PUT ") || chc!(reader, b"DELETE ")
    //     || chc!(reader, b"HEAD ") || chc!(reader, b"CONNECT ") || chc!(reader, b"OPTIONS ") || chc!(reader, b"TRACE ") || chc!(reader, b"PATCH ")
}
