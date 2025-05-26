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

// Helper function to parse FTP command or response
fn parse_ftp_message(data: &[u8]) -> (String, bool) {
    let text = String::from_utf8_lossy(data);
    let trimmed = text.trim_end(); // Remove trailing whitespace/CRLF
    
    // Check if this is a command or response
    let is_response = trimmed.chars().next().map_or(false, |c| c.is_ascii_digit());
    
    (trimmed.to_string(), is_response)
}

// Helper function to get FTP command name
fn get_ftp_command(message: &str) -> String {
    if let Some(space_pos) = message.find(' ') {
        message[..space_pos].to_uppercase()
    } else {
        message.to_uppercase()
    }
}

// Helper function to get FTP response code
fn get_ftp_response_code(message: &str) -> String {
    if message.len() >= 3 && message.chars().take(3).all(|c| c.is_ascii_digit()) {
        message[..3].to_string()
    } else {
        String::new()
    }
}

// Helper function to get a description for FTP response codes
fn get_ftp_response_description(code: &str) -> &'static str {
    match code {
        "110" => "Restart marker reply",
        "120" => "Service ready in nnn minutes",
        "125" => "Data connection already open; transfer starting",
        "150" => "File status okay; about to open data connection",
        "200" => "Command okay",
        "202" => "Command not implemented",
        "211" => "System status, or system help reply",
        "212" => "Directory status",
        "213" => "File status",
        "214" => "Help message",
        "215" => "NAME system type",
        "220" => "Service ready for new user",
        "221" => "Service closing control connection",
        "225" => "Data connection open; no transfer in progress",
        "226" => "Closing data connection",
        "227" => "Entering Passive Mode",
        "228" => "Entering Long Passive Mode",
        "229" => "Entering Extended Passive Mode",
        "230" => "User logged in, proceed",
        "231" => "User logged out; service terminated",
        "232" => "Logout command noted, will complete when transfer done",
        "234" => "Specifies that the server accepts the authentication mechanism specified by the client",
        "250" => "Requested file action okay, completed",
        "257" => "PATHNAME created",
        "331" => "User name okay, need password",
        "332" => "Need account for login",
        "350" => "Requested file action pending further information",
        "421" => "Service not available, closing control connection",
        "425" => "Can't open data connection",
        "426" => "Connection closed; transfer aborted",
        "430" => "Invalid username or password",
        "434" => "Requested host unavailable",
        "450" => "Requested file action not taken",
        "451" => "Requested action aborted: local error in processing",
        "452" => "Requested action not taken. Insufficient storage space",
        "500" => "Syntax error, command unrecognized",
        "501" => "Syntax error in parameters or arguments",
        "502" => "Command not implemented",
        "503" => "Bad sequence of commands",
        "504" => "Command not implemented for that parameter",
        "530" => "Not logged in",
        "532" => "Need account for storing files",
        "550" => "Requested action not taken. File unavailable",
        "551" => "Requested action aborted. Page type unknown",
        "552" => "Requested file action aborted. Exceeded storage allocation",
        "553" => "Requested action not taken. File name not allowed",
        "631" => "Integrity protected reply",
        "632" => "Confidentiality and integrity protected reply",
        "633" => "Confidentiality protected reply",
        _ => "Unknown response code",
    }
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::FTP(message, is_response) = &frame.protocol_field {
            if *is_response {
                // Response
                let code = get_ftp_response_code(message);
                let description = get_ftp_response_description(&code);
                return Some(format!("File Transfer Protocol (Response), Code: {} ({})", code, description));
            } else {
                // Command
                let command = get_ftp_command(message);
                return Some(format!("File Transfer Protocol (Command), Command: {}", command));
            }
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // FTP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        
        // Parse FTP message
        let (message, is_response) = parse_ftp_message(data);
        
        // Store FTP information in the frame
        frame.protocol_field = ProtocolInfoField::FTP(message, is_response);
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // FTP is a text-based protocol, so we read all available data
        let data = reader.refer()?;
        let (message, is_response) = parse_ftp_message(data);
        
        if is_response {
            // Response
            let code = get_ftp_response_code(&message);
            let description = get_ftp_response_description(&code);
            
            field_back_format!(list, reader, message.len(), format!("Response: {}", message));
            
            // Add a field for the response code
            if !code.is_empty() {
                let code_field = Field::label(
                    format!("Response Code: {} ({})", code, description),
                    reader.cursor,
                    reader.cursor + 3
                );
                list.push(code_field);
            }
            
            field.summary = format!("File Transfer Protocol (Response), Code: {} ({})", code, description);
        } else {
            // Command
            let command = get_ftp_command(&message);
            
            field_back_format!(list, reader, message.len(), format!("Command: {}", message));
            
            // Add a field for the command
            if !command.is_empty() {
                let command_field = Field::label(
                    format!("Command: {}", command),
                    reader.cursor,
                    reader.cursor + command.len()
                );
                list.push(command_field);
            }
            
            field.summary = format!("File Transfer Protocol (Command), Command: {}", command);
        }
        
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}