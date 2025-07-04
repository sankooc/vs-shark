// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::concept::{Field, FrameIndex};
use crate::common::connection::{TCPSegment, TLSSegment, TlsData};
use crate::common::core::Context;
use crate::common::enum_def::{ProtocolInfoField, SegmentStatus};
use crate::common::io::DataSource;
use crate::common::{enum_def::Protocol, io::Reader, Frame};
use crate::{add_field_format, add_field_format_fn};
use anyhow::Result;
use record::parse_record_detail;
pub mod record;
mod extension;
mod decode;

#[derive(Default)]
pub struct TLSList {
    pub list: Vec<TlsData>,
}

impl TLSList {
    pub fn push(&mut self, data: TlsData) {
        self.list.push(data);
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<&TlsData> {
        self.list.get(index)
    }
}

fn stamp(index: FrameIndex, reader: &Reader) -> TCPSegment {
    let start = reader.range.start;
    let end = reader.cursor;
    TCPSegment { index, range: start..end }
}

fn tls_type(content_type: u8) -> String {
    match content_type {
        20 => "ChangeCipherSpec".into(),
        21 => "Alert".into(),
        22 => "Handshake".into(),
        23 => "ApplicationData".into(),
        24 => "Heartbeat".into(),
        _ => "".into(),
    }
}
pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        match &frame.protocol_field {
            ProtocolInfoField::TLS(list) => {
                let str = tls_type(list.get(0).unwrap().content_type);
                Some(str)
            },
            ProtocolInfoField::TLSSegment => frame.tcp_descripion(),
            _ => None,
        }
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut left = _reader.left();
        if left == 0 {
            return Ok(Protocol::None);
        }
        if let Some(tcp) = &frame.tcp_info {
            if left != tcp.len as usize {
                //TODO check
            }
            left = tcp.len as usize;
        }
        if left == 0 {
            return Ok(Protocol::None);
        }
        let index = frame.info.index;
        let mut reader = _reader.slice_as_reader(left)?;
        let mut list = TLSList::default();

        if let Some((_, endpoint)) = ctx.connection(frame) {
            // let endpoint = conn.source_endpoint();
            let mut _status = std::mem::replace(&mut endpoint.segment_status, SegmentStatus::Init);
            match _status {
                SegmentStatus::Init => endpoint.segment_status = recycle(index, &mut reader, &mut list)?,
                SegmentStatus::TlsHead(segment, data) => {
                    let _len = data.len();
                    if _len < 5 {
                        let _data = reader.slice(5 - _len, true)?;
                        let mut content = Vec::with_capacity(5);
                        content.extend(data);
                        content.extend(_data);
                        let content_type = content[0];
                        let len = u16::from_be_bytes(content[3..5].try_into()?);
                        if reader.left() >= len as usize {
                            reader.forward(len as usize);
                            let current = stamp(index, &reader);
                            let mut tlsdata = TlsData::new(content_type);
                            tlsdata.append(segment);
                            tlsdata.append(current);
                            list.push(tlsdata);
                            endpoint.segment_status = recycle(index, &mut reader, &mut list)?;
                        } else {
                            reader.forward(reader.left());
                            let mut _seg = TLSSegment::new(content_type, len + 5);
                            _seg.append(segment)?;
                            let current = stamp(index, &reader);
                            _seg.append(current)?;
                            endpoint.segment_status = SegmentStatus::TlsSegment(_seg);
                        }
                    } else {
                        endpoint.segment_status = SegmentStatus::Init;
                    }
                }
                SegmentStatus::TlsSegment(mut _seg) => {
                    let _len = _seg.len;
                    if _len as usize > reader.left() {
                        reader.forward(reader.left());
                        let current = stamp(index, &reader);
                        _seg.append(current)?;
                        endpoint.segment_status = SegmentStatus::TlsSegment(_seg);
                    } else {
                        reader.forward(_len as usize);
                        let current = stamp(index, &reader);
                        _seg.append(current)?;
                        list.push(_seg.into());

                        let _left = reader.left();
                        if _left == 0 {
                            endpoint.segment_status = SegmentStatus::Init;
                            return Ok(Protocol::None);
                        }
                        reader = reader.slice_as_reader(_left)?;
                        endpoint.segment_status = recycle(index, &mut reader, &mut list)?;
                    }
                }
                _ => {
                    endpoint.segment_status = SegmentStatus::Init;
                }
            }
        }

        if list.len() > 0 {
            frame.protocol_field = ProtocolInfoField::TLS(list);
        } else {
            frame.protocol_field = ProtocolInfoField::TLSSegment;
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, _reader: &mut Reader) -> Result<(Protocol, Option<Vec<u8>>)> {
        // let index = frame.info.index;
        field.children = Some(vec![]);
        let mut extra_data = None;
        let list = field.children.as_mut().unwrap();
        if let ProtocolInfoField::TLS(tls_list) = &frame.protocol_field {
            for item in &tls_list.list {
                if item.segments.len() == 1 {
                    let range = item.segments.first().unwrap().range.clone();
                    let ds = _reader.ds();
                    let reader = Reader::new_sub(ds, range)?;
                    list.push(parse_segment(reader, 0)?);
                } else {
                    let data = item.combind(_reader.ds());
                    let mut ds = DataSource::create(data, 0..0);
                    let reader = Reader::new(&ds);
                    list.push(parse_segment(reader, 1)?);
                    let _data = std::mem::take(&mut ds.data);
                    extra_data = Some(_data);
                }
            }
        }
        field.summary = "Transport Layer Security".to_string();
        Ok((Protocol::None, extra_data))
    }
}

fn field_tls_type(content_type: u8) -> String {
    format!("Content Type: {} ({})", tls_type(content_type), content_type)
}

pub fn field_tls_version(val: u16) -> String {
    format!("Version: {} ({:#06x})", tls_version(val), val)
}
fn tls_version(val: u16) -> String {
    match val & 0x00ff {
        0x00 => "SSLv3".to_string(),
        0x01 => "TLSv1.0".to_string(),
        0x02 => "TLSv1.1".to_string(),
        0x03 => "TLSv1.2".to_string(),
        0x04 => "TLSv1.3".to_string(),
        _ => "".to_string(),
    }
}

fn parse_segment(mut reader: Reader, source: u8) -> Result<Field> {
    let start = reader.cursor;
    let mut field = Field::with_children("".to_string(), start, 0);
    field.source = source;
    let content_type = add_field_format_fn!(field, reader, reader.read8()?, field_tls_type);
    let version = add_field_format_fn!(field, reader, reader.read16(true)?, field_tls_version);
    let _len = add_field_format!(field, reader, reader.read16(true)?, "Length:{}");
    field.size = _len as usize + 5;

    // let _reader_record = |reader: &mut Reader, field: &mut Field| parse_record(content_type, version, reader, field);
    let mut record_reader = reader.slice_as_reader(_len as usize)?;
    let mut record_field = Field::with_children("".to_string(), reader.cursor, _len as usize);
    record_field.source = field.source;
    parse_record_detail(content_type, version, &mut record_reader, &mut record_field)?;
    field.children.as_mut().unwrap().push(record_field);

    field.summary = format!("{} Record Layer: {}", tls_version(version), tls_type(content_type));
    Ok(field)
}

pub fn detect(reader: &Reader) -> bool {
    let left = reader.left();
    if left <= 5 {
        false
    } else {
        let data = reader.preview(5).unwrap();
        let content_type = data[0];
        let major = data[1];
        let minor = data[2];
        content_type > 19 && content_type < 25 && major == 3 && minor < 5
    }
}

fn recycle(index: FrameIndex, _reader: &mut Reader, list: &mut TLSList) -> Result<SegmentStatus> {
    let _left = _reader.left();
    if _left == 0 {
        return Ok(SegmentStatus::Init);
    }
    let mut reader = _reader.slice_as_reader(_left)?;
    loop {
        if reader.left() == 0 {
            return Ok(SegmentStatus::Init);
        }
        let extra = reader.left();
        if extra < 5 {
            let data = reader.slice(extra, true)?.to_vec();
            let content_type = data[0];
            if content_type > 19 && content_type < 25 {
                let segment = stamp(index, &reader);
                return Ok(SegmentStatus::TlsHead(segment, data));
            } else {
                // todo
                return Ok(SegmentStatus::Init);
            }
        }
        if detect(&reader) {
            let _start = reader.cursor;
            let content_type = reader.read8()?;
            reader.forward(2);
            let len = reader.read16(true)? as usize;
            if reader.left() >= len {
                reader.forward(len);
                let segment = stamp(index, &reader);
                list.push(TlsData::single(content_type, segment));
                let _left = reader.left();
                if _left == 0 {
                    continue;
                }
                reader = reader.slice_as_reader(_left)?;
            } else {
                // let extra = len - reader.left();
                reader.forward(reader.left());
                let mut _seg = TLSSegment::new(content_type, (len + 5) as u16);
                let segment = stamp(index, &reader);
                _seg.append(segment)?;
                return Ok(SegmentStatus::TlsSegment(_seg));
            }
        } else {
            return Ok(SegmentStatus::Init);
        }
    }
}
