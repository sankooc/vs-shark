use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    constants::{dns_class_mapper, dns_type_mapper},
    field_back_format, read_field_format, read_field_format_fn,
};
use anyhow::Result;

// Helper function to parse DNS name from DNS packet
fn parse_dns_name(_reader: &mut Reader, start_offset: usize) -> Result<String> {
    let mut reader = _reader.clone();
    let mut name = String::new();
    let mut is_first = true;
    let mut finish = 0;
    // let arch = reader.cursor - start_offset;
    loop {
        let len = reader.read8()? as usize;

        // todo!()
        // Check for DNS name compression
        if (len & 0xC0) == 0xC0 {
            // This is a pointer to another location in the packet
            let offset_low = reader.read8()? as usize;
            let offset = ((len & 0x3F) << 8) | offset_low;
            // if let Some(_str) = str_map.get(&offset) {
            //     name.push_str(_str);
            // }

            if finish == 0 {
                finish = reader.cursor;
            }
            reader.set(offset + start_offset);
            continue;
        }

        // End of name
        if len == 0 {
            if finish == 0 {
                finish = reader.cursor;
            }
            break;
        }

        // Add dot between labels
        if !is_first {
            name.push('.');
        } else {
            is_first = false;
        }

        // Read the label
        let label_data = reader.slice(len, true)?;
        name.push_str(&String::from_utf8_lossy(label_data));
    }

    if name.is_empty() {
        name.push('.');
    }

    // str_map.insert(arch, name.clone());
    _reader.set(finish);
    Ok(name)
}

// Helper function to format DNS flags
fn format_dns_flags(flags: u16) -> Vec<String> {
    let mut result = Vec::new();

    // QR flag
    let qr = (flags & 0x8000) != 0;
    result.push(format!("{} = {}", if qr { "1..." } else { "0..." }, if qr { "Response" } else { "Query" }));

    // Opcode
    let opcode = (flags >> 11) & 0xF;
    let opcode_str = match opcode {
        0 => "Standard query",
        1 => "Inverse query",
        2 => "Server status request",
        4 => "Notify",
        5 => "Update",
        _ => "Unknown",
    };
    result.push(format!(".... {:04b} .... .... .... = Opcode: {} ({})", opcode, opcode_str, opcode));

    // Authoritative Answer
    let aa = (flags & 0x0400) != 0;
    result.push(format!(
        ".... .... {}... .... .... = Authoritative: {}",
        if aa { "1" } else { "0" },
        if aa {
            "Server is an authority for domain"
        } else {
            "Server is not an authority for domain"
        }
    ));

    // Truncated
    let tc = (flags & 0x0200) != 0;
    result.push(format!(
        ".... .... .{}.. .... .... = Truncated: {}",
        if tc { "1" } else { "0" },
        if tc { "Message is truncated" } else { "Message is not truncated" }
    ));

    // Recursion Desired
    let rd = (flags & 0x0100) != 0;
    result.push(format!(
        ".... .... ..{}. .... .... = Recursion desired: {}",
        if rd { "1" } else { "0" },
        if rd { "Do query recursively" } else { "Don't query recursively" }
    ));

    // Recursion Available
    let ra = (flags & 0x0080) != 0;
    result.push(format!(
        ".... .... .... {}... .... = Recursion available: {}",
        if ra { "1" } else { "0" },
        if ra { "Server can do recursive queries" } else { "Server can't do recursive queries" }
    ));

    // Z (Reserved)
    let z = (flags & 0x0070) >> 4;
    result.push(format!(".... .... .... .{:03b} .... = Reserved: {}", z, z));

    // Response code
    let rcode = flags & 0x000F;
    let rcode_str = match rcode {
        0 => "No error",
        1 => "Format error",
        2 => "Server failure",
        3 => "Name Error",
        4 => "Not Implemented",
        5 => "Refused",
        _ => "Unknown",
    };
    result.push(format!(".... .... .... .... {:04b} = Reply code: {} ({})", rcode, rcode_str, rcode));

    result
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        match &frame.protocol_field {
            ProtocolInfoField::DnsRESPONSE(transaction_id) => {
                return Some(format!("Domain Name System (response) ID: 0x{:04x}", transaction_id));
            }
            ProtocolInfoField::DnsQUERY(transaction_id) => {
                return Some(format!("Domain Name System (query) ID: 0x{:04x}", transaction_id));
            }
            _ => None,
        }
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // DNS header
        let transaction_id = reader.read16(true)?;
        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;
        if is_response {
            frame.protocol_field = ProtocolInfoField::DnsRESPONSE(transaction_id);
        } else {
            frame.protocol_field = ProtocolInfoField::DnsQUERY(transaction_id);
        }
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        let start_offset = reader.cursor;

        let transaction_id = read_field_format!(list, reader, reader.read16(true)?, "Transaction ID: 0x{:04x}");

        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;

        let mut flags_field = Field::with_children(format!("Flags: 0x{:04x}", flags), reader.cursor - 2, 2);
        let mut flags_list = vec![];

        for flag_str in format_dns_flags(flags) {
            flags_list.push(Field::label(flag_str, reader.cursor - 2, reader.cursor));
        }

        flags_field.children = Some(flags_list);
        list.push(flags_field);

        // Counts
        let query_count = read_field_format!(list, reader, reader.read16(true)?, "Questions: {}");
        let answer_count = read_field_format!(list, reader, reader.read16(true)?, "Answer RRs: {}");
        let authority_count = read_field_format!(list, reader, reader.read16(true)?, "Authority RRs: {}");
        let additional_count = read_field_format!(list, reader, reader.read16(true)?, "Additional RRs: {}");
        // Parse queries
        if query_count > 0 {
            let mut queries_field = Field::with_children(format!("Queries ({})", query_count), reader.cursor, 0);
            let mut queries_list = vec![];

            for _ in 0..query_count {
                if let Ok(query) = read_query_record(start_offset, reader) {
                    queries_list.push(query);
                }
            }
            queries_field.size = reader.cursor - queries_field.start;
            queries_field.children = Some(queries_list);
            list.push(queries_field);
        }

        
        if answer_count > 0 && reader.left() > 0 {
            let mut answers_field = Field::with_children(format!("Answers ({})", answer_count), reader.cursor, 0);
            let mut answers_list = vec![];

            for _ in 0..answer_count {
                if reader.left() < 10 {
                    break;
                }
                if let Ok(field) = read_resource_record(start_offset, reader) {
                    answers_list.push(field);
                }
            }

            answers_field.size = reader.cursor - answers_field.start;
            answers_field.children = Some(answers_list);
            list.push(answers_field);
        }

        // Set summary
        let type_str = if is_response { "response" } else { "query" };
        field.summary = format!("Domain Name System ({}) ID: 0x{:04x}", type_str, transaction_id);
        field.children = Some(list);

        Ok(Protocol::None)
    }
}
fn rr_type(t: u16) -> String {
    format!("Type: {} ({})", dns_type_mapper(t), t)
}
fn rr_class(t: u16) -> String {
    format!("Class: {} ({})", dns_class_mapper(t), t)
}
fn rr_ttl(t: u32) -> String {
    format!("Time to live: {} seconds", t)
}

fn read_query_record(start_offset: usize, reader: &mut Reader) -> Result<Field> {
    let start = reader.cursor;
    let mut list = vec![];
    let name = read_field_format!(list, reader, parse_dns_name(reader, start_offset)?, "Name: {}");
    let record_type = read_field_format_fn!(list, reader, reader.read16(true)?, rr_type);
    let record_class = read_field_format_fn!(list, reader, reader.read16(true)?, rr_class);
    let end = reader.cursor;
    let title = format!(
        "{}, type {}, class {}",
        name,
        dns_type_mapper(record_type),
        dns_class_mapper(record_class)
    );
    let field = Field::new(title, start, end, list);
    Ok(field)
}


fn read_resource_record(start_offset: usize, reader: &mut Reader) -> Result<Field> {
    let start = reader.cursor;
    let mut list = vec![];
    let name = read_field_format!(list, reader, parse_dns_name(reader, start_offset)?, "Name: {}");
    let record_type = read_field_format_fn!(list, reader, reader.read16(true)?, rr_type);
    let record_class = read_field_format_fn!(list, reader, reader.read16(true)?, rr_class);
    read_field_format_fn!(list, reader, reader.read32(true)?, rr_ttl);
    let data_len = read_field_format!(list, reader, reader.read16(true)? as usize, "Data length: {} bytes");
    let finish = reader.cursor + data_len;
    let record_data = match record_type {
        1 => {
            // A record
            if data_len == 4 {
                let _cotent = reader.slice(data_len, true)?;
                let ip = Ipv4Addr::from(<[u8; 4]>::try_from(_cotent).unwrap());
                format!("IPv4 address: {}", ip)
            } else {
                format!("Data (length: {})", data_len)
            }
        }
        28 => {
            if data_len == 16 {
                let _cotent = reader.slice(data_len, true)?;
                let ip_data = Ipv6Addr::from(<[u8; 16]>::try_from(_cotent).unwrap());
                format!("IPv6 address: {}", ip_data)
            } else {
                format!("Data (length: {})", data_len)
            }
        }
        5 => {
            if let Ok(cname) = parse_dns_name(reader, start_offset) {
                format!("CNAME: {}", cname)
            } else {
                format!("Data (length: {})", data_len)
            }
        }
        _ => {
            format!("Data (length: {})", data_len)
        }
    };
    reader.set(finish);
    field_back_format!(list, reader, data_len, record_data.clone());

    let end = reader.cursor;
    let title = format!(
        "{}, type {}, class {}, {}",
        name,
        dns_type_mapper(record_type),
        dns_class_mapper(record_class),
        record_data
    );
    let field = Field::new(title, start, end, list);
    Ok(field)
}
