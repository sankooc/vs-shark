// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    constants::{dns_class_mapper, nbns_type_mapper},
    field_back_format, read_field_format, read_field_format_fn,
};
use anyhow::{bail, Result};

// NetBIOS name encoding/decoding
fn decode_netbios_name(encoded_name: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < encoded_name.len() && i < 32 {
        // NetBIOS names are encoded by taking each character's ASCII value,
        // dividing by 16, and adding the result to ASCII 'A' (65).
        // The remainder is added to ASCII 'A' (65) to form the second character.
        if i + 1 < encoded_name.len() {
            let c1 = encoded_name[i];
            let c2 = encoded_name[i + 1];

            if (b'A'..=b'P').contains(&c1) && (b'A'..=b'P').contains(&c2) {
                let high = c1 - b'A';
                let low = c2 - b'A';
                let ascii = (high << 4) | low;

                // Only append printable ASCII characters
                if (32..127).contains(&ascii) {
                    result.push(ascii as char);
                }
            }
        }
        i += 2;
    }

    // Trim trailing spaces which are common in NetBIOS names
    result.trim_end().to_string()
}

// Helper function to parse NBNS name
fn parse_nbns_name(reader: &mut Reader) -> Result<String> {
    // First byte is the length
    let name_length = reader.read8()? as usize;

    if name_length == 0 {
        return Ok("<empty>".to_string());
    }

    // Read the encoded name
    let encoded_name = reader.slice(name_length, true)?;
    let content = decode_netbios_name(encoded_name);
    reader.forward(1);
    Ok(content)
}

// Helper function to format NBNS flags
fn format_nbns_flags(flags: u16) -> Vec<String> {
    let mut result = Vec::new();

    // Response flag
    let response = (flags & 0x8000) != 0;
    result.push(format!("{} = {}", if response { "1..." } else { "0..." }, if response { "Response" } else { "Query" }));

    // Opcode
    let opcode = (flags >> 11) & 0xF;
    let opcode_str = match opcode {
        0 => "Name query",
        5 => "Registration",
        6 => "Release",
        7 => "WACK",
        8 => "Refresh",
        _ => "Unknown",
    };
    result.push(format!(".... {:04b} .... .... .... = Opcode: {} ({})", opcode, opcode_str, opcode));

    // Authoritative Answer
    let aa = (flags & 0x0400) != 0;
    result.push(format!(
        ".... .... {}... .... .... = Authoritative: {}",
        if aa { "1" } else { "0" },
        if aa { "Yes" } else { "No" }
    ));

    // Truncated
    let tc = (flags & 0x0200) != 0;
    result.push(format!(
        ".... .... .{}.. .... .... = Truncated: {}",
        if tc { "1" } else { "0" },
        if tc { "Yes" } else { "No" }
    ));

    // Recursion Desired
    let rd = (flags & 0x0100) != 0;
    result.push(format!(
        ".... .... ..{}. .... .... = Recursion desired: {}",
        if rd { "1" } else { "0" },
        if rd { "Yes" } else { "No" }
    ));

    // Recursion Available
    let ra = (flags & 0x0080) != 0;
    result.push(format!(
        ".... .... .... {}... .... = Recursion available: {}",
        if ra { "1" } else { "0" },
        if ra { "Yes" } else { "No" }
    ));

    // Broadcast flag
    let b = (flags & 0x0010) != 0;
    result.push(format!(
        ".... .... .... ...{}. .... = Broadcast: {}",
        if b { "1" } else { "0" },
        if b { "Yes" } else { "No" }
    ));

    // Response code
    let rcode = flags & 0x000F;
    let rcode_str = match rcode {
        0 => "No error",
        1 => "Format error",
        2 => "Server failure",
        3 => "Name error",
        4 => "Not implemented",
        5 => "Refused",
        6 => "Active",
        7 => "Conflict",
        _ => "Unknown",
    };
    result.push(format!(".... .... .... .... {:04b} = Reply code: {} ({})", rcode, rcode_str, rcode));

    result
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::NBNS(transaction_id, is_response, name) = &frame.protocol_field {
            let type_str = if *is_response { "response" } else { "query" };
            return Some(format!("NetBIOS Name Service ({}) ID: 0x{:04x}, Name: {}", type_str, transaction_id, name));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // NBNS header is similar to DNS
        let transaction_id = reader.read16(true)?;
        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;

        let query_count = reader.read16(true)?;
        reader.forward(6);
        // let answer_count = reader.read16(true)?;
        // let authority_count = reader.read16(true)?;
        // let additional_count = reader.read16(true)?;

        // Parse the first query name if present
        let mut name = String::from("<unknown>");
        if query_count > 0 {
            name = parse_nbns_name(reader)?;

            // Skip the query type and class
            reader.read16(true)?; // type
            reader.read16(true)?; // class
        }

        // Store NBNS information in the frame
        frame.protocol_field = ProtocolInfoField::NBNS(transaction_id, is_response, name);

        // Skip the rest of the packet

        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];

        // Parse NBNS header
        let transaction_id = reader.read16(true)?;
        field_back_format!(list, reader, 2, format!("Transaction ID: 0x{:04x}", transaction_id));

        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;

        // Format flags
        let mut flags_field = Field::with_children(format!("Flags: 0x{:04x}", flags), reader.cursor - 2, 2);
        let mut flags_list = vec![];

        for flag_str in format_nbns_flags(flags) {
            flags_list.push(Field::label(flag_str, reader.cursor - 2, reader.cursor));
        }

        flags_field.children = Some(flags_list);
        list.push(flags_field);

        // Counts
        let query_count = read_field_format!(list, reader, reader.read16(true)?, "Questions: {}");
        let answer_count = read_field_format!(list, reader, reader.read16(true)?, "Answer RRs: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Authority RRs: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Additional RRs: {}");

        // Parse queries
        if query_count > 0 {
            let mut queries_field = Field::with_children(format!("Queries ({})", query_count), reader.cursor, 0);
            let mut queries_list = vec![];

            for _ in 0..query_count {
                if let Ok(field) = read_query_record(reader) {
                    queries_list.push(field);
                }
            }

            queries_field.size = reader.cursor - queries_field.start;
            queries_field.children = Some(queries_list);
            list.push(queries_field);
        }

        // Parse answers (simplified)
        if answer_count > 0 && reader.left() > 0 {
            let mut answers_field = Field::new(format!("Answers ({})", answer_count), reader.cursor, reader.cursor, vec![]);
            let answers_list = answers_field.children.as_mut().unwrap();

            for _ in 0..answer_count {
                if reader.left() < 10 {
                    // Minimum size for an answer record
                    break;
                }

                if let Ok(rr) = read_resource_record(reader) {
                    answers_list.push(rr);
                }
            }

            answers_field.size = reader.cursor - answers_field.start;
            list.push(answers_field);
        }

        // Set summary
        let type_str = if is_response { "response" } else { "query" };
        field.summary = format!("NetBIOS Name Service ({}) ID: 0x{:04x}", type_str, transaction_id);
        field.children = Some(list);

        Ok(Protocol::None)
    }
}

fn rr_type(t: u16) -> String {
    format!("type {}", nbns_type_mapper(t))
}
fn rr_class(t: u16) -> String {
    format!("Class: {} ({})", dns_class_mapper(t), t)
}
fn read_query_record(reader: &mut Reader) -> Result<Field> {
    let start = reader.cursor;
    let mut field = Field::new("".to_string(), start, start, vec![]);
    let list = field.children.as_mut().unwrap();

    let name = read_field_format!(list, reader, parse_nbns_name(reader)?, "Name: {}");
    read_field_format_fn!(list, reader, reader.read16(true)?, rr_type);
    read_field_format_fn!(list, reader, reader.read16(true)?, rr_class);
    field.summary = format!("Query: {}", name);
    field.size = reader.cursor - start;
    Ok(field)
}

fn read_resource_record(reader: &mut Reader) -> Result<Field> {
    let start = reader.cursor;
    let mut field = Field::new("".to_string(), start, start, vec![]);
    let list = field.children.as_mut().unwrap();

    let name = read_field_format!(list, reader, parse_nbns_name(reader)?, "Name: {}");
    let record_type = read_field_format_fn!(list, reader, reader.read16(true)?, rr_type);
    let record_class = read_field_format_fn!(list, reader, reader.read16(true)?, rr_class);

    read_field_format!(list, reader, reader.read32(true)?, "Time to live: {} seconds");
    let data_len = read_field_format!(list, reader, reader.read16(true)?, "Data length: {} bytes");
    if reader.left() < data_len as usize {
        bail!("invalid data length")
    } else {
        reader.forward(data_len as usize);
    }
    field.summary = format!("Resource Record: {} type {} class {}", name, record_type, record_class);
    field.size = reader.cursor - start;
    Ok(field)
}
