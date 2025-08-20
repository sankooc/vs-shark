use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    field_back_format,
};
use anyhow::Result;

// Helper function to parse SSDP headers
fn parse_ssdp_headers(data: &[u8]) -> (String, String, String) {
    let text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = text.split("\r\n").collect();
    
    let mut method_or_status = String::new();
    let mut location = String::new();
    let mut nt_or_st = String::new(); // Notification Type or Search Target
    
    // Parse first line (Request/Status line)
    if !lines.is_empty() {
        method_or_status = lines[0].to_string();
    }
    
    // Parse headers
    for line in lines.iter().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if let Some(pos) = line.find(':') {
            let header_name = line[..pos].trim().to_lowercase();
            let header_value = line[pos+1..].trim();
            
            match header_name.as_str() {
                "location" => location = header_value.to_string(),
                "nt" | "st" => nt_or_st = header_value.to_string(),
                _ => {}
            }
        }
    }
    
    (method_or_status, location, nt_or_st)
}

// Helper function to determine if an SSDP message is a request or response
fn is_ssdp_request(first_line: &str) -> bool {
    // SSDP request line format: METHOD * HTTP/1.1
    // SSDP response line format: HTTP/1.1 STATUS_CODE REASON
    !first_line.starts_with("HTTP/")
}

// Helper function to extract SSDP method or status code
fn extract_method_or_status(first_line: &str) -> String {
    if is_ssdp_request(first_line) {
        // Extract method from request line
        if let Some(pos) = first_line.find(' ') {
            return first_line[..pos].to_string();
        }
    } else {
        // Extract status code from response line
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            return format!("{} {}", parts[1], parts[2..].join(" "));
        }
    }
    
    String::new()
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::SSDP(method_or_status, location, nt_or_st) = &frame.protocol_field {
            // Format differently based on whether it's a notification, search, or response
            if method_or_status == "NOTIFY" {
                return Some(format!("Simple Service Discovery Protocol (Notify), NT: {}", nt_or_st));
            } else if method_or_status == "M-SEARCH" {
                return Some(format!("Simple Service Discovery Protocol (Search), ST: {}", nt_or_st));
            } else {
                return Some(format!("Simple Service Discovery Protocol (Response), Location: {}", location));
            }
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // SSDP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        
        // Parse SSDP headers
        let (first_line, location, nt_or_st) = parse_ssdp_headers(data);
        let method_or_status = extract_method_or_status(&first_line);
        
        // Store SSDP information in the frame
        frame.protocol_field = ProtocolInfoField::SSDP(method_or_status, location, nt_or_st);
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // SSDP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        let text = String::from_utf8_lossy(data);
        
        // Split the message into lines
        let lines: Vec<&str> = text.split("\r\n").collect();
        
        // Parse first line (Request/Status line)
        if !lines.is_empty() {
            let first_line = lines[0];
            let is_request = is_ssdp_request(first_line);
            
            if is_request {
                field_back_format!(list, reader, first_line.len() + 2, format!("Request Line: {}", first_line));
            } else {
                field_back_format!(list, reader, first_line.len() + 2, format!("Status Line: {}", first_line));
            }
        }
        
        // Parse headers
        let mut current_pos = 0;
        let mut method_or_status = String::new();
        let mut location = String::new();
        let mut nt_or_st = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                // Already processed the first line
                current_pos += line.len() + 2; // +2 for CRLF
                method_or_status = extract_method_or_status(line);
                continue;
            }
            
            let line_len = line.len();
            if line_len == 0 {
                // Empty line indicates the end of headers
                field_back_format!(list, reader, 2, "\r\n".to_string());
                current_pos += 2;
                break;
            }
            
            // Process header line
            if let Some(pos) = line.find(':') {
                let header_name = line[..pos].trim();
                let header_value = line[pos+1..].trim();
                
                field_back_format!(list, reader, line_len + 2, format!("{}: {}", header_name, header_value));
                
                // Store important headers
                let header_lower = header_name.to_lowercase();
                match header_lower.as_str() {
                    "location" => location = header_value.to_string(),
                    "nt" | "st" => nt_or_st = header_value.to_string(),
                    _ => {}
                }
            } else {
                field_back_format!(list, reader, line_len + 2, line.to_string());
            }
            
            current_pos += line_len + 2; // +2 for CRLF
        }
        
        // Set summary based on the type of SSDP message
        if method_or_status == "NOTIFY" {
            field.summary = format!("Simple Service Discovery Protocol (Notify), NT: {}", nt_or_st);
        } else if method_or_status == "M-SEARCH" {
            field.summary = format!("Simple Service Discovery Protocol (Search), ST: {}", nt_or_st);
        } else {
            field.summary = format!("Simple Service Discovery Protocol (Response), Location: {}", location);
        }
        
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}