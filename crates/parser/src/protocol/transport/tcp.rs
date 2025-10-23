// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_backstep, add_field_format, add_field_forward, add_field_label_no_range, add_sub_field_with_reader,
    common::{
        concept::Field,
        connection::{TCPStat, TcpFlagField},
        core::Context,
        enum_def::{PacketStatus, Protocol, TCPDetail},
        io::Reader,
        util::{read_bit, read_bits},
        Frame,
    },
    constants::{ip_protocol_type_mapper, tcp_option_kind_mapper},
};
use anyhow::Result;

fn read_tcp_flag(reader: &mut Reader, field: &mut Field) -> Result<TcpFlagField> {
    let flag_bit = reader.read16(true)?;
    let result = TcpFlagField::from(flag_bit);
    // let len = flag_bit >> 12;;
    add_field_label_no_range!(field, read_bits(flag_bit, 0..4, |v| format!("Header Length: {v}")));
    add_field_label_no_range!(field, read_bits(flag_bit, 4..7, |_v| "Reserved".into()));
    add_field_label_no_range!(field, read_bit(flag_bit, 7, "Accurate ECN", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 8, "Congestion Window Reduced", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 9, "ECN-Echo", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 10, "URG", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 11, "ACK", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 12, "PUSH", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 13, "RESET", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 14, "SYN", ("SET", "NOT SET")));
    add_field_label_no_range!(field, read_bit(flag_bit, 15, "FIN", ("SET", "NOT SET")));
    field.summary = result.to_string();
    Ok(result)
}

// Returns false if EOL is reached
fn read_tcp_options(reader: &mut Reader, field: &mut Field) -> Result<bool> {
    let mut count = 0;
    loop {
        let left = reader.left();
        if left == 0 {
            break;
        }
        let r = add_sub_field_with_reader!(field, reader, read_tcp_option)?;
        count += 1;
        if r == 0 {
            break;
        }
    }
    field.summary = format!("TCP Options ({count})");
    Ok(true)
}

fn read_tcp_option(reader: &mut Reader, field: &mut Field) -> Result<u8> {
    let kind = reader.read8()?;
    let sim = tcp_option_kind_mapper(kind);
    add_field_backstep!(field, reader, 1, format!("Kind: {} ({})", sim, kind));
    match kind {
        0 => {
            field.summary = format!("TCP Option - {sim} ({kind})");
        }
        1 => {
            field.summary = format!("TCP Option - {sim} ({kind})");
        }
        2 => {
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len == 4 {
                let mss = add_field_format!(field, reader, reader.read16(true)?, "MSS Value: {}");
                field.summary = format!("TCP Option - Maximum segment size: {mss} bytes");
            } else if len > 2 {
                reader.forward(2);
            }
        }
        3 => {
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len == 3 {
                let sup = add_field_format!(field, reader, reader.read8()?, "Shift: {}");
                field.summary = format!("TCP Option - window scale: {sup}");
            } else if len > 3 {
                reader.forward((len - 2) as usize);
            }
        }
        4 => {
            //SACK Permitted
            add_field_format!(field, reader, reader.read8()?, "Length {}");
            field.summary = "TCP Option - SACK Permitted".into();
        }
        5 => {
            //SACK
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len == 10 {
                add_field_format!(field, reader, reader.read32(true)?, "Left Edge: {}");
                add_field_format!(field, reader, reader.read32(true)?, "Right Edge: {}");
                field.summary = "TCP Option - SACK".into();
            } else if len > 2 {
                reader.forward((len - 2) as usize);
            }
        }
        8 => {
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len == 10 {
                add_field_format!(field, reader, reader.read32(true)?, "Timestamp value: {}");
                add_field_format!(field, reader, reader.read32(true)?, "Timestamp echo reply: {}");
                field.summary = "TCP Option - Timestamp".into();
            } else if len > 2 {
                reader.forward((len - 2) as usize);
            }
        }
        28 => {
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len == 4 {
                let bit = reader.read16(true)?;
                let g = bit >> 15;
                let v = bit & 0x7fff;
                let unit = match g {
                    1 => format!("{v} minus"),
                    _ => format!("{v} second"),
                };
                add_field_backstep!(field, reader, 2, unit.clone());
                field.summary = format!("TCP Option - {unit}");
            } else if len > 2 {
                reader.forward((len - 2) as usize);
            }
        }
        29 | 30 => {
            let len = add_field_format!(field, reader, reader.read8()?, "Length {}");
            if len > 2 {
                reader.forward((len - 2) as usize);
            }
        }
        _ => {
        }
    }
    Ok(kind)
}

pub struct Visitor;
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}
impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        frame.tcp_description()
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let index = frame.info.index;
        let start = reader.left();
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let sequence = reader.read32(true)?;
        let ack = reader.read32(true)?;
        let flag_bit = reader.read16(true)?;
        let state = TcpFlagField::from(flag_bit);
        let _window = reader.read16(true)?;
        let crc = reader.read16(true)?;
        let _urgent = reader.read16(true)?;
        let len = state.head_len();
        if len > 5 {
            let skip = (len - 5) * 4;
            reader.forward(skip as usize);
        }
        let mut left_size = reader.left();
        let iplen = frame.iplen as usize;
        if iplen > 0 && start > iplen && iplen + left_size >= start {
            left_size = iplen + left_size - start;
        }
        let ds = reader.ds();
        let range = reader.cursor..reader.cursor + left_size;
        let tcp_state = TCPStat::new(index, sequence, ack, crc, state, left_size as u16);

        frame.add_proto(crate::common::ProtoMask::TCP);
        if let Ok(mut tcp_info) = ctx.get_connect(frame, source_port, target_port, tcp_state, ds, range) {
            tcp_info.flag_bit = flag_bit;
            frame.info.status = match &tcp_info.status {
                TCPDetail::NEXT | TCPDetail::KEEPALIVE => PacketStatus::NORNAL,
                _ => PacketStatus::ERROR,
            };
            frame.ports = Some((source_port, target_port));
            let mut next = tcp_info.next_protocol;

            if tcp_info.len == 0 {
                next = Protocol::None;
            }
            frame.tcp_info = Some(tcp_info);

            return Ok(next);
        }
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        // let mut list = Vec::new();

        let source_port = add_field_format!(field, reader, reader.read16(true)?, "Source Port: {}");
        let target_port = add_field_format!(field, reader, reader.read16(true)?, "Destination Port: {}");

        let info = frame.tcp_info.as_ref().unwrap();

        let sequence = reader.read32(true)?;
        // add_field_format!(field, reader, 4)
        add_field_backstep!(field, reader, 4, format!("Sequence Number: {}    (relative sequence number)", info.seq));
        add_field_backstep!(field, reader, 4, format!("Sequence Number (raw): {}", sequence));
        // list.push(Field::label(format!("[Next Sequence Number: {}    (relative sequence number)]", info.next), 0, 0));
        let ack = reader.read32(true)?;
        add_field_backstep!(field, reader, 4, format!("Acknowledgment Number: {}    (relative ack number)", info.ack));
        add_field_backstep!(field, reader, 4, format!("Acknowledgment Number (raw): {}", ack));
        // let _state = TcpFlagField::from(reader.read16(true)?);
        let state = add_sub_field_with_reader!(field, reader, read_tcp_flag)?;
        // let state = add_field_format!(field, reader, TcpFlagField::from(reader.read16(true)?), "{}");
        add_field_format!(field, reader, reader.read16(true)?, "Window: {}");
        add_field_format!(field, reader, reader.read16(true)?, "Checksum: {:#06x} [unverified]");
        add_field_format!(field, reader, reader.read16(true)?, "Urgent Pointer: {}");
        let len = state.head_len();
        if len > 5 {
            let skip = (len - 5) * 4;
            let mut _reader = reader.slice_as_reader(skip as usize)?;
            add_sub_field_with_reader!(field, &mut _reader, read_tcp_options)?;
            // reader.forward(skip as usize);
        }
        let payload_len = info.len as usize;

        add_field_forward!(field, reader, payload_len, format!("TCP payload ({} bytes)", payload_len));

        field.summary = format!("Transmission Control Protocol, Src Port: {}, Dst Port: {}, Len: {}", source_port, target_port, info.len);
        // field.children = Some(list);
        Ok(info.next_protocol)
    }
}
