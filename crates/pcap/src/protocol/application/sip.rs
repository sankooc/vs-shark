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

// Helper function to parse SIP headers
fn parse_sip_headers(data: &[u8]) -> (String, String, String, String) {
    let text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = text.split("\r\n").collect();
    
    let mut method_or_status = String::new();
    let mut call_id = String::new();
    let mut from = String::new();
    let mut to = String::new();
    
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
                "call-id" => call_id = header_value.to_string(),
                "from" => from = header_value.to_string(),
                "to" => to = header_value.to_string(),
                _ => {}
            }
        }
    }
    
    (method_or_status, call_id, from, to)
}

// Helper function to determine if a SIP message is a request or response
fn is_sip_request(first_line: &str) -> bool {
    // SIP request line format: METHOD URI SIP/2.0
    // SIP response line format: SIP/2.0 STATUS_CODE REASON
    !first_line.starts_with("SIP/")
}

// Helper function to extract SIP method or status code
fn extract_method_or_status(first_line: &str) -> String {
    if is_sip_request(first_line) {
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
        if let ProtocolInfoField::SIP(method_or_status, call_id, from, to) = &frame.protocol_field {
            return Some(format!("Session Initiation Protocol ({}) Call-ID: {}",
                method_or_status, call_id));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // SIP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        
        // Parse SIP headers
        let (first_line, call_id, from, to) = parse_sip_headers(data);
        let method_or_status = extract_method_or_status(&first_line);
        
        // Store SIP information in the frame
        frame.protocol_field = ProtocolInfoField::SIP(method_or_status, call_id, from, to);
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // SIP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        let text = String::from_utf8_lossy(data);
        
        // Split the message into lines
        let lines: Vec<&str> = text.split("\r\n").collect();
        
        // Parse first line (Request/Status line)
        if !lines.is_empty() {
            let first_line = lines[0];
            let is_request = is_sip_request(first_line);
            
            if is_request {
                field_back_format!(list, reader, first_line.len() + 2, format!("Request Line: {}", first_line));
            } else {
                field_back_format!(list, reader, first_line.len() + 2, format!("Status Line: {}", first_line));
            }
        }
        
        // Parse headers
        let mut current_pos = 0;
        let mut method_or_status = String::new();
        let mut call_id = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                // Already processed the first line
                current_pos += line.len() + 2; // +2 for CRLF
                method_or_status = extract_method_or_status(line);
                continue;
            }
            
            let line_len = line.len();
            if line_len == 0 && i < lines.len() - 1 {
                // Empty line indicates the end of headers and start of body
                field_back_format!(list, reader, 2, "\r\n".to_string());
                current_pos += 2;
                
                // Create a field for the message body
                let body_start = current_pos;
                let body_text = lines[i+1..].join("\r\n");
                field_back_format!(list, reader, body_text.len(), format!("Message Body ({} bytes)", body_text.len()));
                break;
            }
            
            // Process header line
            if let Some(pos) = line.find(':') {
                let header_name = line[..pos].trim();
                let header_value = line[pos+1..].trim();
                
                field_back_format!(list, reader, line_len + 2, format!("{}: {}", header_name, header_value));
                
                // Store important headers
                let header_lower = header_name.to_lowercase();
                if header_lower == "call-id" {
                    call_id = header_value.to_string();
                }
            } else {
                field_back_format!(list, reader, line_len + 2, line.to_string());
            }
            
            current_pos += line_len + 2; // +2 for CRLF
        }
        
        // Set summary
        field.summary = format!("Session Initiation Protocol ({}) Call-ID: {}", method_or_status, call_id);
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}