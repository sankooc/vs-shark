use crate::{add_field_format, add_field_label_no_range};
use crate::common::concept::{Field, HttpHeadContinue};
use crate::common::core::{Context, Segment};
use crate::common::enum_def::{DataError, ProtocolInfoField, SegmentStatus};
use crate::common::io::Reader;
use crate::common::{enum_def::Protocol, Frame};
use crate::common::{hex_num, quick_trim_num, std_string, trim_data};
use anyhow::{bail, Result};

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

pub fn inspect(data: &[u8]) -> (Option<usize>, Option<bool>) {
    let size = data.len();
    let mut length = None;
    let mut chunked = None;
    if size >= 18 {
        match &data[0..18] {
            b"transfer-encoding:" | b"Transfer-Encoding:" | b"TRANSFER-ENCODING:" => {
                let _data = trim_data(&data[18..]);
                if _data.len() == 7 {
                    match _data {
                        b"chunked" | b"Chunked" | b"CHUNKED" => {
                            chunked = Some(true);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if size >= 15 {
        match &data[0..15] {
            b"content-length:" | b"Content-Length:" | b"CONTENT-LENGTH:" => {
                length = Some(quick_trim_num(&data[15..]).unwrap());
            }
            _ => (),
        }
    }
    (length, chunked)
}

pub fn parse_header(_length: usize, _chunked: bool, reader: &mut Reader, tcp_index: usize) -> Result<SegmentStatus> {
    let mut length = _length;
    let mut chunked = _chunked;
    loop {
        let left = reader.left();
        if left == 2 {
            match reader.preview(2)? {
                b"\r\n" => {
                    reader.forward(2);
                    if chunked {
                        return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, 0));
                    } else if length > 0 {
                        return Ok(SegmentStatus::HttpContentContinue(tcp_index, length));
                    } else {
                        return Ok(SegmentStatus::Finish);
                    }
                }
                _ => {}
            }
        }
        if let Some(size) = reader.search_enter(0xffff) {
            if size == 0 {
                if chunked {
                    return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, 0));
                } else if length > 0 {
                    return Ok(SegmentStatus::HttpContentContinue(tcp_index, length));
                } else {
                    return Ok(SegmentStatus::Finish);
                }
            }
            let data = reader.slice(size, true)?;
            // ll.push(String::from_utf8_lossy(&data).to_string());

            let (_length, _chunked) = inspect(data);
            if let Some(true) = _chunked {
                chunked = true;
            }
            if let Some(length_) = _length {
                length = length_;
            }
            reader.forward(2);
        } else {
            //todo
            if reader.left() == 0 {
                return Ok(SegmentStatus::Finish);
            }
            let extra = reader.slice(reader.left(), true)?;

            // fix it http headers segments
            return Ok(SegmentStatus::HttpHeaderContinue(HttpHeadContinue::new(length, chunked, extra.to_vec())));
        }
    }
}
pub fn parse_content(reader: &mut Reader, tcp_index: usize, length: usize) -> Result<SegmentStatus> {
    let left = reader.left();
    if left >= length {
        return Ok(SegmentStatus::Init);
    } else {
        let left = length - left;
        return Ok(SegmentStatus::HttpContentContinue(tcp_index, left));
    }
}
pub fn parse_chunked(reader: &mut Reader, tcp_index: usize, left: usize) -> Result<SegmentStatus> {
    let _left = reader.left();
    if left >= _left {
        return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, left - _left));
    }
    if left > 0 {
        if !reader.forward(left) {
            bail!(DataError::HttpChunkForwardErr)
        }
    }
    loop {
        let ext = reader.left();
        if ext == 0 {
            return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, 0));
        }
        if ext < 3 {
            return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, 3 - ext));
        }
        if let Some(pos) = reader.search_enter(u16::MAX.into()) {
            let data = reader.slice(pos, true)?;
            if let Ok(len) = hex_num(&data) {
                reader.forward(2);
                if len == 0 {
                    return Ok(SegmentStatus::Finish);
                }
                let next_len = len + 2;
                if next_len > reader.left() {
                    return Ok(SegmentStatus::HttpChunkedContinue(tcp_index, next_len - reader.left()));
                }
                reader.forward(next_len);
                continue;
            } else {
                return Ok(SegmentStatus::Error);
            }
        } else {
            return Ok(SegmentStatus::Error);
        }
    }
}

pub fn parse(reader: &mut Reader, segment_status: SegmentStatus) -> Result<SegmentStatus> {
    match segment_status {
        SegmentStatus::HttpDetected(tcp_index) => {
            let status = parse_header(0, false, reader, tcp_index)?;
            return parse(reader, status);
        }
        SegmentStatus::HttpContentContinue(tcp_index, left) => {
            return parse_content(reader, tcp_index, left);
        }
        SegmentStatus::HttpChunkedContinue(tcp_index, left) => {
            return parse_chunked(reader, tcp_index, left);
        }
        _ => {
            return Ok(segment_status);
        }
    }
}

fn read_line(reader: &mut Reader, len: usize) -> Result<String> {
    let data = reader.slice(len, true)?;
    Ok(std_string(&data)?.to_string())
}
pub struct Visitor;
impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::Http(data, _mi) = &frame.protocol_field {
            return Some(String::from_utf8_lossy(data).to_string());
        } else {
            return frame.tcp_descripion();
        }
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut left = _reader.left();
        if let Some(tcp) = &frame.tcp_info {
            if left != tcp.len as usize {
                //TODO check
            }
            left = tcp.len as usize;
        }
        let mut reader = _reader.slice_as_reader(left)?;
        let mut next_status = SegmentStatus::Init;
        if let Some((tcp_index, endpoint)) = ctx.connection(frame) {
            // let endpoint = conn2.source_endpoint();

            let mut _status = std::mem::replace(&mut endpoint.segment_status, SegmentStatus::Init);
            match _status {
                SegmentStatus::Init | SegmentStatus::Error | SegmentStatus::Finish => {
                    if detect(&reader) {
                        if let Some(pos) = reader.search_enter(0xffff) {
                            let data = reader.slice(pos, true)?.to_vec();
                            let mut mi = None;
                            reader.forward(2);
                            next_status = parse(&mut reader, SegmentStatus::HttpDetected(0))?;
                            match next_status {
                                SegmentStatus::HttpContentContinue(_, size) => {
                                    let segment = Segment {
                                        index: frame.info.index,
                                        range: reader.range.clone(),
                                    };
                                    let message_index = ctx.create_segment_message(Protocol::HTTP, tcp_index, segment);
                                    mi = Some(message_index);
                                    next_status = SegmentStatus::HttpContentContinue(message_index, size);
                                }
                                SegmentStatus::HttpChunkedContinue(_, size) => {
                                    let segment = Segment {
                                        index: frame.info.index,
                                        range: reader.range.clone(),
                                    };
                                    let message_index = ctx.create_segment_message(Protocol::HTTP, tcp_index, segment);
                                    mi = Some(message_index);
                                    next_status = SegmentStatus::HttpChunkedContinue(message_index, size)
                                }
                                SegmentStatus::HttpHeaderContinue(mut hhc) => {
                                    let segment = Segment {
                                        index: frame.info.index,
                                        range: reader.range.clone(),
                                    };
                                    let message_index = ctx.create_segment_message(Protocol::HTTP, tcp_index, segment);
                                    mi = Some(message_index);
                                    hhc.message_index = message_index;
                                    hhc.frame_index = frame.info.index;
                                    next_status = SegmentStatus::HttpHeaderContinue(hhc)
                                }
                                _ => {}
                            }
                            frame.protocol_field = ProtocolInfoField::Http(data, mi);
                        }
                    }
                }
                SegmentStatus::HttpContentContinue(message_index, _) => {
                    let segment = Segment {
                        index: frame.info.index,
                        range: reader.range.clone(),
                    };
                    ctx.add_segment_message(message_index, segment);
                    next_status = parse(&mut reader, _status.clone())?;
                }
                SegmentStatus::HttpChunkedContinue(message_index, _) => {
                    let segment = Segment {
                        index: frame.info.index,
                        range: reader.range.clone(),
                    };
                    ctx.add_segment_message(message_index, segment);
                    next_status = parse(&mut reader, _status.clone())?;
                }
                SegmentStatus::HttpHeaderContinue(mut hhc) => {
                    let segment = Segment {
                        index: frame.info.index,
                        range: reader.range.clone(),
                    };
                    ctx.add_segment_message(hhc.message_index, segment);
                    frame.protocol_field = ProtocolInfoField::HttpSegment(hhc.message_index);
                    let mut pre_data = hhc.extra;
                    if let Some(size) = reader.search_enter(0x0fff) {
                        let mut length = hhc.length;
                        let mut chunked = hhc.chunked;
                        let data = reader.slice(size, true)?;
                        pre_data.extend_from_slice(data);
                        let (_length, _chunked) = inspect(&pre_data);
                        if let Some(true) = _chunked {
                            chunked = true;
                        }
                        if let Some(length_) = _length {
                            length = length_;
                        }
                        reader.forward(2);
                        let _next_status = parse_header(length, chunked, &mut reader, hhc.message_index)?;
                        next_status = parse(&mut reader, _next_status)?;
                        
                    } else {
                        let data = reader.slice(reader.left(), true)?;
                        pre_data.extend_from_slice(data);
                        hhc.frame_index = frame.info.index;
                        hhc.extra = pre_data;
                        next_status = SegmentStatus::HttpHeaderContinue(hhc)
                    }
                }
                _ => {}
            };
        }

        if let Some((_, endpoint)) = ctx.connection(frame) {
            // let endpoint = conn.source_endpoint();
            endpoint.segment_status = next_status;
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, ctx: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        match &frame.protocol_field {
            ProtocolInfoField::Http(_, _mi) => {
                loop {
                    let left = reader.left();
                    if left >= 2 {
                        match reader.preview(2)? {
                            b"\r\n" => {
                                reader.forward(2);
                                //todo header parse finish
                                break;
                            }
                            _ => {}
                        }
                    }
                    if let Some(pos) = reader.search_enter(0xffff) {
                        add_field_format!(field, reader, read_line(reader, pos)?, "{}");
                        reader.forward(2);
                    } else {
                        break;
                    }
                }
                field.summary = "Hypertext Transfer Protocol".to_string();
            }
            ProtocolInfoField::HttpSegment(mi) => {
                
                let left = reader.left();
                field.summary = format!("Http Segment Frame {}", left);
                if let Some(sm) = ctx.segment_messages.get(*mi) {
                    for segment in sm.segments.iter() {
                        add_field_label_no_range!(field, format!("Frame {} [{}]", segment.index + 1, segment.range.end - segment.range.start));
                    }
                }
                reader.forward(left);
                
            }
            _ => {}
        }
        // if let ProtocolInfoField::Http(_, _mi) = &frame.protocol_field {
        //     // let mut list = Vec::new();
        //     loop {
        //         let left = reader.left();
        //         if left >= 2 {
        //             match reader.preview(2)? {
        //                 b"\r\n" => {
        //                     reader.forward(2);
        //                     //todo header parse finish
        //                     break;
        //                 }
        //                 _ => {}
        //             }
        //         }
        //         if let Some(pos) = reader.search_enter(0xffff) {
        //             add_field_format!(field, reader, read_line(reader, pos)?, "{}");
        //             reader.forward(2);
        //         } else {
        //             break;
        //         }
        //     }
        //     field.summary = "Hypertext Transfer Protocol".to_string();
        // } else {
        //     let left = reader.left();
        //     reader.forward(left);
            

        //     field.summary = format!("Http Segment Frame {}", left);
        // }
        Ok(Protocol::None)
    }
}
