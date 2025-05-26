use std::net::Ipv4Addr;

use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    constants::nbns_type_mapper,
    field_back_format,
    read_field_format,
};
use anyhow::Result;

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
            
            if c1 >= b'A' && c1 <= b'P' && c2 >= b'A' && c2 <= b'P' {
                let high = c1 - b'A';
                let low = c2 - b'A';
                let ascii = (high << 4) | low;
                
                // Only append printable ASCII characters
                if ascii >= 32 && ascii < 127 {
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
    
    // NBNS names are typically 32 bytes (16 characters) encoded in a special way
    Ok(decode_netbios_name(encoded_name))
}

// Helper function to format NBNS flags
fn format_nbns_flags(flags: u16) -> Vec<String> {
    let mut result = Vec::new();
    
    // Response flag
    let response = (flags & 0x8000) != 0;
    result.push(format!("{} = {}", 
        if response { "1..." } else { "0..." },
        if response { "Response" } else { "Query" }));
    
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
    result.push(format!(".... .... {}... .... .... = Authoritative: {}", 
        if aa { "1" } else { "0" },
        if aa { "Yes" } else { "No" }));
    
    // Truncated
    let tc = (flags & 0x0200) != 0;
    result.push(format!(".... .... .{}.. .... .... = Truncated: {}", 
        if tc { "1" } else { "0" },
        if tc { "Yes" } else { "No" }));
    
    // Recursion Desired
    let rd = (flags & 0x0100) != 0;
    result.push(format!(".... .... ..{}. .... .... = Recursion desired: {}", 
        if rd { "1" } else { "0" },
        if rd { "Yes" } else { "No" }));
    
    // Recursion Available
    let ra = (flags & 0x0080) != 0;
    result.push(format!(".... .... .... {}... .... = Recursion available: {}", 
        if ra { "1" } else { "0" },
        if ra { "Yes" } else { "No" }));
    
    // Broadcast flag
    let b = (flags & 0x0010) != 0;
    result.push(format!(".... .... .... ...{}. .... = Broadcast: {}", 
        if b { "1" } else { "0" },
        if b { "Yes" } else { "No" }));
    
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
            return Some(format!("NetBIOS Name Service ({}) ID: 0x{:04x}, Name: {}",
                type_str, transaction_id, name));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // NBNS header is similar to DNS
        let transaction_id = reader.read16(true)?;
        let flags = reader.read16(true)?;
        let is_response = (flags & 0x8000) != 0;
        
        let query_count = reader.read16(true)?;
        let answer_count = reader.read16(true)?;
        let authority_count = reader.read16(true)?;
        let additional_count = reader.read16(true)?;
        
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
        let authority_count = read_field_format!(list, reader, reader.read16(true)?, "Authority RRs: {}");
        let additional_count = read_field_format!(list, reader, reader.read16(true)?, "Additional RRs: {}");
        
        // Parse queries
        if query_count > 0 {
            let mut queries_field = Field::with_children(format!("Queries ({})", query_count), reader.cursor, 0);
            let mut queries_list = vec![];
            
            for i in 0..query_count {
                let query_start = reader.cursor;
                
                // Parse query name
                let name = parse_nbns_name(reader)?;
                
                // Query type and class
                let query_type = reader.read16(true)?;
                let query_class = reader.read16(true)?;
                
                let mut query_field = Field::with_children(
                    format!("Query: {}, type {}", name, nbns_type_mapper(query_type)),
                    query_start,
                    reader.cursor - query_start
                );
                
                let mut query_details = vec![];
                query_details.push(Field::label(format!("Name: {}", name), query_start, reader.cursor - 4));
                query_details.push(Field::label(
                    format!("Type: {} ({})", nbns_type_mapper(query_type), query_type),
                    reader.cursor - 4,
                    reader.cursor - 2
                ));
                query_details.push(Field::label(
                    format!("Class: IN ({})", query_class),
                    reader.cursor - 2,
                    reader.cursor
                ));
                
                query_field.children = Some(query_details);
                queries_list.push(query_field);
            }
            
            queries_field.size = reader.cursor - queries_field.start;
            queries_field.children = Some(queries_list);
            list.push(queries_field);
        }
        
        // Parse answers (simplified)
        if answer_count > 0 && reader.left() > 0 {
            let mut answers_field = Field::with_children(format!("Answers ({})", answer_count), reader.cursor, 0);
            let mut answers_list = vec![];
            
            for i in 0..answer_count {
                if reader.left() < 10 { // Minimum size for an answer record
                    break;
                }
                
                let answer_start = reader.cursor;
                
                // Parse name
                let name = parse_nbns_name(reader)?;
                
                // Record type, class, TTL, and data length
                let record_type = reader.read16(true)?;
                let record_class = reader.read16(true)?;
                let ttl = reader.read32(true)?;
                let data_len = reader.read16(true)? as usize;
                
                // Parse record data based on type
                let record_data = if reader.left() >= data_len {
                    let data = match record_type {
                        1 => { // A record (IP address)
                            if data_len >= 6 { // NetBIOS encodes IP with 2 extra bytes
                                // Skip flags
                                reader.read16(true)?;
                                // Read IP
                                let ip = reader.read_ip4()?;
                                format!("IPv4 address: {}", ip)
                            } else {
                                reader.slice(data_len, true)?;
                                format!("Data (length: {})", data_len)
                            }
                        },
                        _ => {
                            reader.slice(data_len, true)?;
                            format!("Data (length: {})", data_len)
                        }
                    };
                    data
                } else {
                    "<insufficient data>".to_string()
                };
                
                let mut answer_field = Field::with_children(
                    format!("Answer: {}, type {}, {}", 
                        name, nbns_type_mapper(record_type), record_data),
                    answer_start,
                    reader.cursor - answer_start
                );
                
                let mut answer_details = vec![];
                answer_details.push(Field::label(format!("Name: {}", name), answer_start, answer_start + name.len()));
                answer_details.push(Field::label(
                    format!("Type: {} ({})", nbns_type_mapper(record_type), record_type),
                    answer_start + name.len(),
                    answer_start + name.len() + 2
                ));
                answer_details.push(Field::label(
                    format!("Class: IN ({})", record_class),
                    answer_start + name.len() + 2,
                    answer_start + name.len() + 4
                ));
                answer_details.push(Field::label(
                    format!("Time to live: {} seconds", ttl),
                    answer_start + name.len() + 4,
                    answer_start + name.len() + 8
                ));
                answer_details.push(Field::label(
                    format!("Data length: {}", data_len),
                    answer_start + name.len() + 8,
                    answer_start + name.len() + 10
                ));
                answer_details.push(Field::label(
                    record_data,
                    answer_start + name.len() + 10,
                    reader.cursor
                ));
                
                answer_field.children = Some(answer_details);
                answers_list.push(answer_field);
            }
            
            answers_field.size = reader.cursor - answers_field.start;
            answers_field.children = Some(answers_list);
            list.push(answers_field);
        }
        
        // Set summary
        let type_str = if is_response { "response" } else { "query" };
        field.summary = format!("NetBIOS Name Service ({}) ID: 0x{:04x}", type_str, transaction_id);
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}