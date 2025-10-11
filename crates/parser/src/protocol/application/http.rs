// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::add_field_format;
use crate::common::concept::{Field, FrameIndex, MessageIndex};
use crate::common::core::{Context, HttpMessage, Segment, SegmentData};
use crate::common::enum_def::{ProtocolInfoField, SegmentStatus};
use crate::common::io::Reader;
use crate::common::{enum_def::Protocol, Frame};
use crate::common::{hex_num, quick_trim_num, std_string, trim_data};
use anyhow::Result;

pub fn detect(reader: &Reader) -> (bool, bool) {
    if reader.left() < 8 {
        return (false, false);
    }
    let buffer = reader.preview(8).unwrap();

    if buffer.len() >= 4 {
        match &buffer[0..4] {
            b"GET " | b"POST" | b"PUT " | b"DELE" | b"HEAD" | b"OPTI" | b"PATC" | b"TRAC" | b"CONN" => return (true, true),
            _ => (),
        }
    }
    if &buffer[0..7] == b"HTTP/1." {
        return (true, false);
    }
    (false, false)
}

pub fn detect_length(data: &[u8]) -> Option<usize> {
    let size: usize = data.len();
    if size >= 15 {
        match &data[0..15] {
            b"content-length:" | b"Content-Length:" | b"CONTENT-LENGTH:" => {
                return Some(quick_trim_num(&data[15..]).unwrap());
            }
            _ => (),
        }
    }
    None
}
pub fn detect_chunked(data: &[u8]) -> bool {
    let size = data.len();
    if size >= 18 {
        match &data[0..18] {
            b"transfer-encoding:" | b"Transfer-Encoding:" | b"TRANSFER-ENCODING:" => {
                let _data = trim_data(&data[18..]);
                if _data.len() == 7 {
                    match _data {
                        b"chunked" | b"Chunked" | b"CHUNKED" => {
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    false
}
pub fn detect_type(data: &[u8]) -> Option<String> {
    let size = data.len();
    if size >= 13 {
        match &data[0..13] {
            b"content-type:" | b"Content-Type:" | b"CONTENT-TYPE:" => {
                return Some(String::from_utf8_lossy(&data[14..]).to_string());
            }
            _ => {}
        }
    }
    None
}
pub fn detect_hostname(data: &[u8]) -> Option<String> {
    let size = data.len();
    if size >= 5 {
        match &data[0..5] {
            b"Host:" | b"host:" | b"HOST:" => {
                return Some(String::from_utf8_lossy(trim_data(&data[5..])).to_string());
            }
            _ => {}
        }
    }
    None
}

fn read_line(reader: &mut Reader, len: usize) -> Result<String> {
    let data = reader.slice(len, true)?;
    Ok(std_string(data)?.to_string())
}

pub fn parse_http_header(record: &mut HttpMessage, reader: &mut Reader, message_index: MessageIndex) -> Result<SegmentStatus> {
    loop {
        let left = reader.left();
        if left == 2 && reader.preview(2)? == b"\r\n" {
            reader.forward(2);
            if record.chunked {
                return Ok(SegmentStatus::HttpChunkedContinue(message_index, 0));
            } else if let Some(length) = record.length {
                if length > 0 {
                    return Ok(SegmentStatus::HttpContentContinue(message_index, length));
                }
            }
            return Ok(SegmentStatus::Finish);
        }
        if let Some(size) = reader.search_enter(0xffff) {
            if size == 0 {
                reader.forward(2);
                if record.chunked {
                    return Ok(SegmentStatus::HttpChunkedContinue(message_index, 0));
                } else if let Some(length) = record.length {
                    if length > 0 {
                        return Ok(SegmentStatus::HttpContentContinue(message_index, length));
                    }
                }
                return Ok(SegmentStatus::Finish);
            }
            let data = reader.slice(size, true)?;
            if record.length.is_none() {
                record.length = detect_length(data);
            }
            if !record.chunked {
                record.chunked = detect_chunked(data);
            }
            if record.content_type.is_none() {
                record.content_type = detect_type(data);
            }

            if record.hostname.is_none() {
                record.hostname = detect_hostname(data);
                // if let Some(hn) = &record.hostname {
                //     let name = hn.clone();
                //     ctx.add_http_hostname(message_index, &name);

                // }
            }
            reader.forward(2);
        } else {
            if reader.left() == 0 {
                return Ok(SegmentStatus::Finish);
            }
            let extra = reader.slice(reader.left(), true)?;
            return Ok(SegmentStatus::HttpHeaderContinue(message_index, extra.to_vec()));
        }
    }
}

pub fn parse_http_chunked(record: &mut HttpMessage, index: FrameIndex, reader: &mut Reader, message_index: MessageIndex, left: usize) -> Result<SegmentStatus> {
    let _left = reader.left();
    if left >= _left {
        if left > _left + 2 {
            record.append_body(index, reader.cursor..reader.cursor + _left);
        } else {
            record.append_body(index, reader.cursor..reader.cursor + left - 2);
        }
        return Ok(SegmentStatus::HttpChunkedContinue(message_index, left - _left));
    } else if left > 0 {
        reader.forward(left);
        if left > 2 {
            record.append_body(index, reader.cursor - left..reader.cursor - 2);
        }
    }
    loop {
        let ext = reader.left();
        if ext == 0 {
            return Ok(SegmentStatus::HttpChunkedContinue(message_index, 0));
        }
        if let Some(pos) = reader.search_enter(u16::MAX.into()) {
            let data = reader.slice(pos, true)?;
            if let Ok(len) = hex_num(data) {
                reader.forward(2);
                if len == 0 {
                    return Ok(SegmentStatus::Finish);
                }
                if let Some(ll) = record.length {
                    record.length = Some(ll + len);
                }
                let next_len = len + 2;
                let _left = reader.left();
                if len > _left {
                    record.append_body(index, reader.left_range());
                } else {
                    record.append_body(index, reader.cursor..reader.cursor + len);
                }
                if next_len > _left {
                    return Ok(SegmentStatus::HttpChunkedContinue(message_index, next_len - _left));
                }
                reader.forward(next_len);
                continue;
            } else {
                return Ok(SegmentStatus::Error);
            }
        } else {
            if ext < 10 {
                let extra = reader.slice(ext, true)?;
                return Ok(SegmentStatus::HttpChunkedBroken(message_index, extra.to_vec()));
            }
            return Ok(SegmentStatus::Error);
        }
    }
}

fn segment_append(data: SegmentData, segment: Segment) -> SegmentData {
    match data {
        SegmentData::None => SegmentData::Single(segment),
        SegmentData::Single(_segment) => SegmentData::Multiple(vec![_segment, segment]),
        SegmentData::Multiple(mut segments) => {
            segments.push(segment);
            SegmentData::Multiple(segments)
        }
    }
}

fn parse_http(ctx: &mut Context, reader: &mut Reader, frame_index: FrameIndex, status: SegmentStatus) -> Result<SegmentStatus> {
    match status {
        SegmentStatus::HttpDetected(message_index) => {
            if let Some(record) = ctx.get_http_message(message_index) {
                let cursor = reader.cursor;
                // let connect =
                let status = parse_http_header(record, reader, message_index)?;

                let segment = Segment {
                    index: frame_index,
                    range: cursor..reader.cursor,
                };
                let orgin = std::mem::take(&mut record.headers);

                match &status {
                    SegmentStatus::HttpHeaderContinue(_, _) => {
                        record.headers = segment_append(orgin, segment);
                        if let Some(hn) = &record.hostname {
                            let name = hn.clone();
                            ctx.add_http_hostname(message_index, &name);
                        }
                        Ok(status)
                    }
                    _ => {
                        record.headers = segment_append(orgin, segment);
                        if let Some(hn) = &record.hostname {
                            let name = hn.clone();
                            ctx.add_http_hostname(message_index, &name);
                        }
                        parse_http(ctx, reader, frame_index, status)
                    }
                }
            } else {
                Ok(SegmentStatus::Error)
            }
        }
        SegmentStatus::HttpHeaderContinue(message_index, mut extra) => {
            if let Some(record) = ctx.get_http_message(message_index) {
                if let Some(size) = reader.search_enter(0x0fff) {
                    let segment = Segment {
                        index: frame_index,
                        range: reader.cursor..reader.cursor + size + 2,
                    };
                    let orgin = std::mem::take(&mut record.headers);
                    record.headers = segment_append(orgin, segment);
                    let data = reader.slice(size, true)?;
                    extra.extend_from_slice(data);
                    if record.length.is_none() {
                        record.length = detect_length(&extra);
                    }
                    if !record.chunked {
                        record.chunked = detect_chunked(&extra);
                    }
                    if record.content_type.is_none() {
                        record.content_type = detect_type(&extra);
                    }
                    if record.hostname.is_none() {
                        record.hostname = detect_hostname(&extra);
                        if let Some(hn) = &record.hostname {
                            let name = hn.clone();
                            ctx.add_http_hostname(message_index, &name);
                        }
                    }
                    reader.forward(2);
                    parse_http(ctx, reader, frame_index, SegmentStatus::HttpDetected(message_index))
                } else {
                    let segment = Segment {
                        index: frame_index,
                        range: reader.left_range(),
                    };
                    let orgin = std::mem::take(&mut record.headers);
                    record.headers = segment_append(orgin, segment);
                    let data = reader.slice(reader.left(), true)?;
                    extra.extend_from_slice(data);
                    Ok(SegmentStatus::HttpHeaderContinue(message_index, extra))
                }
            } else {
                Ok(SegmentStatus::Error)
            }
        }
        SegmentStatus::HttpContentContinue(message_index, left) => {
            if let Some(record) = ctx.get_http_message(message_index) {
                let _left = reader.left();
                let size = std::cmp::min(left, _left);
                let segment = Segment {
                    index: frame_index,
                    range: reader.cursor..reader.cursor + size,
                };
                let orgin = std::mem::take(&mut record.content);
                record.content = segment_append(orgin, segment);

                if _left >= left {
                    return Ok(SegmentStatus::Init);
                } else {
                    return Ok(SegmentStatus::HttpContentContinue(message_index, left - _left));
                }
            }
            Ok(SegmentStatus::Error)
        }
        SegmentStatus::HttpChunkedContinue(message_index, left) => {
            if let Some(record) = ctx.get_http_message(message_index) {
                if record.length.is_none() {
                    record.length = Some(0);
                }
                parse_http_chunked(record, frame_index, reader, message_index, left)
            } else {
                Ok(SegmentStatus::Error)
            }
        }
        SegmentStatus::HttpChunkedBroken(message_index, mut extra) => {
            if let Some(record) = ctx.get_http_message(message_index) {
                let last = extra[extra.len() - 1];
                if last == b'\r' {
                    reader.forward(1);
                    if let Ok(len) = hex_num(&extra[..extra.len() - 2]) {
                        if len == 0 {
                            return Ok(SegmentStatus::Finish);
                        }
                        if let Some(ll) = record.length {
                            record.length = Some(ll + len);
                        }
                        return parse_http_chunked(record, frame_index, reader, message_index, len + 2);
                    }
                } else if let Some(size) = reader.search_enter(0x000f) {
                    let data = reader.slice(size, true)?;
                    extra.extend_from_slice(data);
                    reader.forward(2);
                    if let Ok(len) = hex_num(&extra) {
                        if len == 0 {
                            return Ok(SegmentStatus::Finish);
                        }
                        if let Some(ll) = record.length {
                            record.length = Some(ll + len);
                        }
                        return parse_http_chunked(record, frame_index, reader, message_index, len + 2);
                    }
                } else {
                    return Ok(SegmentStatus::Error);
                }
                // TODO
            }
            Ok(SegmentStatus::Finish)
        }
        _ => Ok(status),
    }
}
pub struct Visitor;
impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::Http(data, _mi) = &frame.protocol_field {
            Some(data.clone())
        } else {
            frame.tcp_description()
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
        if let Some((conversation_key, endpoint)) = ctx.connection(frame) {
            let frame_index = frame.info.index;
            let ts = frame.info.time;
            let mut _status = std::mem::replace(&mut endpoint.segment_status, SegmentStatus::Init);
            match _status {
                SegmentStatus::Init | SegmentStatus::Error | SegmentStatus::Finish => {
                    let rs = detect(&reader);
                    if rs.0 {
                        if let Some(pos) = reader.search_enter(0xffff) {
                            let data = reader.slice(pos, true)?.to_vec();
                            let line = String::from_utf8_lossy(&data).to_string();
                            // let mut mi = None;
                            reader.forward(2);
                            // let hostname = re
                            let message_index = ctx.init_segment_message(frame_index, line.clone(), rs.1, conversation_key, ts);
                            next_status = parse_http(ctx, &mut reader, frame_index, SegmentStatus::HttpDetected(message_index))?;
                            frame.protocol_field = ProtocolInfoField::Http(line, message_index);
                        }
                    }
                }
                _ => {
                    next_status = parse_http(ctx, &mut reader, frame_index, _status)?;
                }
            };
        }

        if let Some((_, endpoint)) = ctx.connection(frame) {
            endpoint.segment_status = next_status;
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        match &frame.protocol_field {
            ProtocolInfoField::Http(_, _mi) => {
                loop {
                    let left = reader.left();
                    if left >= 2 && reader.preview(2)? == b"\r\n" {
                        reader.forward(2);
                        //todo header parse finish
                        break;
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
            // ProtocolInfoField::HttpSegment(mi) => {

            // let left = reader.left();
            // field.summary = format!("Http Segment Frame {}", left);
            // if let Some(sm) = ctx.segment_messages.get(*mi) {
            //     for segment in sm.segments.iter() {
            //         add_field_label_no_range!(field, format!("Frame {} [{}]", segment.index + 1, segment.range.end - segment.range.start));
            //     }
            // }
            // reader.forward(left);
            // }
            _ => {}
        }
        Ok(Protocol::None)
    }
}
