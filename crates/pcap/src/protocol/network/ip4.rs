// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::net::Ipv4Addr;

use crate::{
    add_field_backstep, add_field_backstep_fn, add_field_format, add_field_format_fn, add_sub_field, common::{concept::Field, core::Context, enum_def::{AddressField, DataError, Protocol}, io::Reader, Frame}, constants::ip_protocol_type_mapper, protocol::ip4_mapper
};
use anyhow::{bail, Result};

pub fn head_lenstr(head_len: u8) -> String {
    format!(".... {:04b} = Header Length: {} bytes ({})", head_len, head_len * 4, head_len)
}
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

pub fn t_flags(flags_word: u16, main_field: &mut Field) {
    let flags = (flags_word >> 13) & 0x07;
    let fragment_offset = flags_word & 0x1FFF;

    let reserved_bit = (flags >> 2) & 0x01;
    let dont_fragment = (flags >> 1) & 0x01;
    let more_fragments = flags & 0x01;
    
    main_field.summary = format!("Flags: 0x{:01x} ({}{}{}), Fragment offset: {}", 
        flags,
        if reserved_bit == 1 { "R" } else { "." },
        if dont_fragment == 1 { "D" } else { "." },
        if more_fragments == 1 { "M" } else { "." },
        fragment_offset);
    
    if let Some(children) = &mut main_field.children {
        // Reserved bit
        children.push(Field::label(
            format!("{:1}... .... .... .... = Reserved bit: {}", 
                   reserved_bit,
                   if reserved_bit == 1 { "Set" } else { "Not set" }),
            0, 1
        ));
        
        // Don't fragment flag
        children.push(Field::label(
            format!(".{}.. .... .... .... = Don't fragment: {}", 
                   dont_fragment, 
                   if dont_fragment == 1 { "Set" } else { "Not set" }),
            0, 1
        ));
        
        // More fragments flag
        children.push(Field::label(
            format!("..{}. .... .... .... = More fragments: {}", 
                   more_fragments, 
                   if more_fragments == 1 { "Set" } else { "Not set" }),
            0, 1
        ));
        
        let offset_binary = format!("{:013b}", fragment_offset);
        children.push(Field::label(
            format!("...{} {} {} {} = Fragment offset: {}", 
                &offset_binary[0..1],
                &offset_binary[1..5],
                &offset_binary[5..9],
                &offset_binary[9..13],
                fragment_offset),
            0, 1
        ));
    }
}


pub fn t_tos(tos: u8, main_field: &mut Field) {
    let precedence = (tos >> 5) & 0x07;
    let dscp = tos >> 2;
    let ecn = tos & 0x03;
    
    let precedence_str = match precedence {
        0 => "Routine (0)",
        1 => "Priority (1)",
        2 => "Immediate (2)",
        3 => "Flash (3)",
        4 => "Flash Override (4)",
        5 => "Critical (5)",
        6 => "Internetwork Control (6)",
        7 => "Network Control (7)",
        _ => "Unknown"
    };
    
    let ecn_str = if ((tos >> 1) & 0x01) == 1 && (tos & 0x01) == 1 {
        "CE (3)"
    } else if ((tos >> 1) & 0x01) == 1 {
        "ECT(1) (1)"
    } else if (tos & 0x01) == 1 {
        "ECT(0) (2)"
    } else {
        "Not-ECT (0)"
    };
    
    
    main_field.summary = format!("Differentiated Services Field: 0x{:02x} (DSCP: 0x{:02x}, ECN: 0x{:02x})", tos, dscp, ecn);
    
    if let Some(children) = &mut main_field.children {
        children.push(Field::label(
            format!("{:03b}. .... = Differentiated Services Codepoint: {}", precedence, precedence_str),
            0, 1
        ));
        
        children.push(Field::label(
            format!("...{} .... = ECN-Capable Transport (ECT): {}", 
                if ((tos >> 1) & 0x01) == 1 { "1" } else { "0" },
                if ((tos >> 1) & 0x01) == 1 { "1" } else { "0" }),
            0, 1
        ));
        
        children.push(Field::label(
            format!(".... .{}{}{} = Explicit Congestion Notification: {}", 
                if (tos & 0x01) == 1 { "1" } else { "0" }, 
                if ((tos >> 1) & 0x01) == 1 { "1" } else { "0" }, 
                if (tos & 0x01) == 1 { "1" } else { "0" },
                ecn_str),
            0, 1
        ));
    }
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let AddressField::IPv4(s, t) = &frame.address_field {
            return Some(format!("Internet Protocol Version 4, Src: {}, Dst: {}", s, t));
        }
        None
    }
    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        reader.read8()?; // tos
        let total_len = reader.read16(true)?;
        reader.read16(true)?; //
        reader.read16(true)?; // flag TODO
        reader.read8()?; // ttl
        let protocol_type = reader.read8()?; // protocol
        reader.read16(true)?; // checksum
        let _data = reader.slice(8, true)?;
        let source = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[..4])?);
        let target = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[4..])?);
        frame.address_field = AddressField::IPv4(source, target);
        if head_len < 5 {
            bail!(DataError::Ipv4HeadLengthInvalid)
        }
        let ext = head_len - 5;
        if ext > 0 {
            reader.slice((ext * 4) as usize, true)?;
        }
        let _stop = reader.left();
        if total_len == 0 {
            //  payload_len is None;
        } else {
            if total_len < (_start - _stop) as u16 {
                return Ok(Protocol::None);
            }
            frame.iplen = total_len - (_start - _stop) as u16;
        }

        Ok(ip4_mapper(protocol_type))
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        // let mut list = field.children.as_mut().unwrap();
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        add_field_backstep!(field, reader, 1, "0100 .... = Version: 4".into());
        add_field_backstep_fn!(field, reader, 1, head_lenstr(head_len));
        // reader.read8()?; // tos
        add_sub_field!(field, reader, reader.read8()?, t_tos);
        let total_len = add_field_format!(field, reader, reader.read16(true)?, "Total Length: {}");
        add_field_format!(field, reader, reader.read16(true)?, "Identification: {:#06x}");
        add_sub_field!(field, reader, reader.read16(true)?, t_flags);
        add_field_format!(field, reader, reader.read8()?, "Time To Live: {}");
        let protocol_type = add_field_format_fn!(field, reader, reader.read8()?, t_protocol);
        add_field_format!(field, reader, reader.read16(true)?, "Header Checksum: {}");

        let source = add_field_format!(field, reader, reader.read_ip4()?, "Source Address: {}");
        let target = add_field_format!(field, reader, reader.read_ip4()?, "Destination Address: {}");
        let ext = head_len - 5;
        if ext > 0 {
            reader.forward((ext * 4) as usize);
        }
        let _stop = reader.left();
        if total_len == 0 {
            //  payload_len is None;
        } else if total_len < (_start - _stop) as u16 {
            bail!(DataError::Ipv4TotalLengthInvalid);
        }

        field.summary = format!("Internet Protocol Version 4, Src: {}, Dst: {}", source, target);
        Ok(ip4_mapper(protocol_type))
    }
}
