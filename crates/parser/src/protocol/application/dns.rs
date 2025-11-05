// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    add_field_backstep, add_field_format, add_field_format_fn, add_sub_field_with_reader,
    common::{
        Frame, concept::{DNSRecord, Field, NameService}, core::Context, enum_def::{Protocol, ProtocolInfoField}, io::Reader
    },
    constants::{dns_class_mapper, dns_type_mapper},
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
    result.push(format!(".... {opcode:04b} .... .... .... = Opcode: {opcode_str} ({opcode})"));

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
    result.push(format!(".... .... .... .{z:03b} .... = Reserved: {z}"));

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
    result.push(format!(".... .... .... .... {rcode:04b} = Reply code: {rcode_str} ({rcode})"));

    result
}

pub fn name_service_parse(frame: &mut Frame, reader: &mut Reader, ns_type: NameService) -> Result<Protocol> {
    let start_offset = reader.cursor;
    let transaction_id = reader.read16(true)?;
    let flags = reader.read16(true)?;
    let is_response = (flags & 0x8000) != 0;
    if is_response {
        frame.protocol_field = ProtocolInfoField::DNSRESPONSE(ns_type, transaction_id, start_offset);
    } else {
        frame.protocol_field = ProtocolInfoField::DNSQUERY(ns_type, transaction_id, start_offset);
    }
    frame.add_proto(crate::common::ProtoMask::DNS);
    Ok(Protocol::None)
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        match &frame.protocol_field {
            ProtocolInfoField::DNSRESPONSE(_, transaction_id, _) => Some(format!("Domain Name System (response) ID: 0x{transaction_id:04x}")),
            ProtocolInfoField::DNSQUERY(_, transaction_id, _) => Some(format!("Domain Name System (query) ID: 0x{transaction_id:04x}")),
            _ => None,
        }
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        name_service_parse(frame, reader, NameService::DNS)
    }
    // pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
    //     Visitor::_detail(field, reader)
    // }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let start_offset = reader.cursor;

        let transaction_id = add_field_format!(field, reader, reader.read16(true)?, "Transaction ID: 0x{:04x}");

        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;

        let mut flags_field = Field::with_children(format!("Flags: 0x{flags:04x}"), reader.cursor - 2, 2);

        for flag_str in format_dns_flags(flags) {
            add_field_backstep!(flags_field, reader, 2, flag_str);
        }
        field.children.as_mut().unwrap().push(flags_field);

        // Counts
        let query_count = add_field_format!(field, reader, reader.read16(true)?, "Questions: {}");
        let answer_count = add_field_format!(field, reader, reader.read16(true)?, "Answer RRs: {}");
        let authority_count = add_field_format!(field, reader, reader.read16(true)?, "Authority RRs: {}");
        let additional_count = add_field_format!(field, reader, reader.read16(true)?, "Additional RRs: {}");
        // Parse queries
        if query_count > 0 {
            for _ in 0..query_count {
                add_sub_field_with_reader!(field, reader, |reader2, field2| parse_query_field(reader2, field2, start_offset))?;
            }
        }

        if answer_count > 0 && reader.left() > 0 {
            for _ in 0..answer_count {
                if reader.left() < 10 {
                    break;
                }
                add_sub_field_with_reader!(field, reader, |reader2, field2| parese_resource_record_field(reader2, field2, start_offset))?;
            }
        }

        if authority_count > 0 && reader.left() > 0 {
            for _ in 0..authority_count {
                if reader.left() < 10 {
                    break;
                }
                add_sub_field_with_reader!(field, reader, |reader2, field2| parese_resource_record_field(reader2, field2, start_offset))?;
            }
        }

        if additional_count > 0 && reader.left() > 0 {
            for _ in 0..additional_count {
                if reader.left() < 10 {
                    break;
                }
                add_sub_field_with_reader!(field, reader, |reader2, field2| parese_resource_record_field(reader2, field2, start_offset))?;
            }
        }
        let type_str = if is_response { "response" } else { "query" };
        field.summary = format!("Domain Name System ({type_str}) ID: 0x{transaction_id:04x}");
        Ok(Protocol::None)
    }

    pub fn answers(reader: &mut Reader) -> Result<Vec<DNSRecord>> {
        let start_offset = reader.cursor;
        let _tid = reader.read16(true)?;
        reader.read16(true)?; // flag
        let query_count = reader.read16(true)?;
        let answer_count = reader.read16(true)?;
        reader.forward(4);

        if query_count > 0 {
            for _ in 0..query_count {
                parse_dns_name(reader, start_offset)?;
                reader.forward(4);
            }
        }
        let mut rs = vec![];

        if answer_count > 0 && reader.left() > 0 {
            for _ in 0..answer_count {
                if reader.left() < 10 {
                    break;
                }
                let mut item = DNSRecord::default();
                let host = parse_dns_name(reader, start_offset)?;
                let record_type = reader.read16(true)?;
                item.host = host;
                item.rtype = dns_type_mapper(record_type).to_string();
                let mut message = None;
                match record_type {
                    41 => {
                        // "Type: OPT (41)".into()
                    }
                    3 | 4 | 20 => {
                        // "Deprecated"
                    }
                    249 | 250 => {
                        // field.summary = "Type: DDNS".into();
                    }
                    255 => {
                        // field.summary = "Type: ANY".into();
                    }
                    46..=48 => {
                        // field.summary = "Type: DNSSEC".into();
                    }
                    _ => {
                        let record_class = reader.read16(true)?;
                        item.class = dns_class_mapper(record_class).to_string();
                        let _ttl = reader.read32(true)?;
                        let data_len =  reader.read16(true)? as usize;
                        let finish = reader.cursor + data_len;
                        let _record_data = match record_type {
                            1 => {
                                // A record
                                if data_len == 4 {
                                    let _cotent = reader.slice(data_len, true)?;
                                    let ip = Ipv4Addr::from(<[u8; 4]>::try_from(_cotent).unwrap());
                                    message = Some(ip.to_string());
                                }
                            }
                            28 => {
                                // AAAA record
                                if data_len == 16 {
                                    let _cotent = reader.slice(data_len, true)?;
                                    let ip_data = Ipv6Addr::from(<[u8; 16]>::try_from(_cotent).unwrap());
                                    message = Some(ip_data.to_string());
                                }
                            }
                            5 => {
                                if let Ok(cname) = parse_dns_name(reader, start_offset) {
                                    message = Some(cname);
                                }
                            },
                            2 => {
                                // NS record
                                if let Ok(ns) = parse_dns_name(reader, start_offset) {
                                    message = Some(ns)
                                }
                            },
                            16 => {
                                // TXT record
                                let mut txt_data = String::new();
                                let mut remaining = data_len;
                                let _data_start = reader.cursor;

                                while remaining > 0 && reader.left() > 0 {
                                    if reader.left() < 1 {
                                        break;
                                    }

                                    let str_len = reader.read8()? as usize;
                                    if str_len > remaining - 1 || str_len > reader.left() {
                                        break;
                                    }

                                    let txt_bytes = reader.slice(str_len, true)?;
                                    if let Ok(txt) = std::str::from_utf8(txt_bytes) {
                                        if !txt_data.is_empty() {
                                            txt_data.push_str(", ");
                                        }
                                        txt_data.push_str(&format!("\"{txt}\""));
                                    }

                                    remaining -= str_len + 1; // +1 for length byte
                                }
                                if !txt_data.is_empty() {
                                    message = Some(txt_data)
                                }
                            }
                            _ => {}
                        };
                        reader.set(finish);
                    }
                }
                item.info = message;
                rs.push(item);
            }
        }
        Ok(rs)
    }
}
fn rr_type(t: u16) -> String {
    format!("Type: {} ({})", dns_type_mapper(t), t)
}
fn rr_class(t: u16) -> String {
    format!("Class: {} ({})", dns_class_mapper(t), t)
}
fn rr_ttl(t: u32) -> String {
    format!("Time to live: {t} seconds")
}

fn parse_query_field(reader: &mut Reader, field: &mut Field, start_offset: usize) -> Result<()> {
    let name = add_field_format!(field, reader, parse_dns_name(reader, start_offset)?, "Name: {}");
    let record_type = add_field_format_fn!(field, reader, reader.read16(true)?, rr_type);
    let record_class = add_field_format_fn!(field, reader, reader.read16(true)?, rr_class);
    field.summary = format!("{}, type {}, class {}", name, dns_type_mapper(record_type), dns_class_mapper(record_class));
    Ok(())
}

fn parese_resource_record_field(reader: &mut Reader, field: &mut Field, start_offset: usize) -> Result<()> {
    let name = add_field_format!(field, reader, parse_dns_name(reader, start_offset)?, "Name: {}");
    let record_type = add_field_format_fn!(field, reader, reader.read16(true)?, rr_type);
    match record_type {
        41 => {
            add_field_format!(field, reader, reader.read16(true)?, "UDP payload size: {} bytes");
            field.summary = "Type: OPT (41)".into();
        }
        3 | 4 | 20 => {
            // dep
            field.summary = "Deprecated".into();
        }
        249 | 250 => {
            field.summary = "Type: DDNS".into();
        }
        255 => {
            field.summary = "Type: ANY".into();
        }
        46..=48 => {
            field.summary = "Type: DNSSEC".into();
        }
        _ => {
            let record_class = add_field_format_fn!(field, reader, reader.read16(true)?, rr_class);
            add_field_format_fn!(field, reader, reader.read32(true)?, rr_ttl);
            let data_len = add_field_format!(field, reader, reader.read16(true)? as usize, "Data length: {} bytes");
            let finish = reader.cursor + data_len;
            let record_data = match record_type {
                1 => {
                    // A record
                    if data_len == 4 {
                        let _cotent = reader.slice(data_len, true)?;
                        let ip = Ipv4Addr::from(<[u8; 4]>::try_from(_cotent).unwrap());
                        format!("IPv4 address: {ip}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                28 => {
                    // AAAA record
                    if data_len == 16 {
                        let _cotent = reader.slice(data_len, true)?;
                        let ip_data = Ipv6Addr::from(<[u8; 16]>::try_from(_cotent).unwrap());
                        format!("IPv6 address: {ip_data}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                5 => {
                    // CNAME record
                    if let Ok(cname) = parse_dns_name(reader, start_offset) {
                        format!("CNAME: {cname}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                2 => {
                    // NS record
                    if let Ok(ns) = parse_dns_name(reader, start_offset) {
                        format!("Name Server: {ns}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                6 => {
                    // SOA record
                    if reader.left() >= 22 {
                        // Minimum SOA record size
                        // let mut soa_data = String::new();
                        add_field_format!(field, reader, parse_dns_name(reader, start_offset)?, "Primary NS: {}");
                        add_field_format!(field, reader, parse_dns_name(reader, start_offset)?, "Responsible: {}");
                        add_field_format!(field, reader, reader.read32(true)?, "Serial: {}");
                        add_field_format!(field, reader, reader.read32(true)?, "Refresh: {} seconds");
                        add_field_format!(field, reader, reader.read32(true)?, "Retry: {} seconds");
                        add_field_format!(field, reader, reader.read32(true)?, "Expire: {} seconds");
                        add_field_format!(field, reader, reader.read32(true)?, "Minimum: {} seconds");
                        "".into()
                    } else {
                        "".into()
                    }
                }
                12 => {
                    // PTR record
                    if let Ok(ptr) = parse_dns_name(reader, start_offset) {
                        format!("Domain name pointer: {ptr}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                15 => {
                    // MX record
                    if reader.left() >= 2 {
                        let preference = reader.read16(true)?;
                        if let Ok(exchange) = parse_dns_name(reader, start_offset) {
                            format!("Mail exchange: {exchange} (preference: {preference})")
                        } else {
                            format!("Preference: {}, Data (length: {})", preference, data_len - 2)
                        }
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                16 => {
                    // TXT record
                    let mut txt_data = String::new();
                    let mut remaining = data_len;
                    let _data_start = reader.cursor;

                    while remaining > 0 && reader.left() > 0 {
                        if reader.left() < 1 {
                            break;
                        }

                        let str_len = reader.read8()? as usize;
                        if str_len > remaining - 1 || str_len > reader.left() {
                            break;
                        }

                        let txt_bytes = reader.slice(str_len, true)?;
                        if let Ok(txt) = std::str::from_utf8(txt_bytes) {
                            if !txt_data.is_empty() {
                                txt_data.push_str(", ");
                            }
                            txt_data.push_str(&format!("\"{txt}\""));
                        }

                        remaining -= str_len + 1; // +1 for length byte
                    }

                    if !txt_data.is_empty() {
                        format!("Text: {txt_data}")
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                33 => {
                    // SRV record
                    if reader.left() >= 6 {
                        // Priority, weight, port
                        let priority = reader.read16(true)?;
                        let weight = reader.read16(true)?;
                        let port = reader.read16(true)?;
                        if let Ok(target) = parse_dns_name(reader, start_offset) {
                            format!("Service: {target}:{port} (priority: {priority}, weight: {weight})")
                        } else {
                            format!("Priority: {}, Weight: {}, Port: {}, Data (length: {})", priority, weight, port, data_len - 6)
                        }
                    } else {
                        format!("Data (length: {data_len})")
                    }
                }
                _ => {
                    format!("Data (length: {data_len})")
                }
            };
            reader.set(finish);
            if !record_data.is_empty() {
                add_field_backstep!(field, reader, data_len, record_data.clone());
            }
            field.summary = format!("{}, type {}, class {}, {}", name, dns_type_mapper(record_type), dns_class_mapper(record_class), record_data);
        }
    }
    Ok(())
}
