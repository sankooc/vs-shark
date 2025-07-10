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
    field_back_format,
};
use anyhow::Result;

// IPSec Protocol Types
const IPSEC_AH: u8 = 51;  // Authentication Header
const IPSEC_ESP: u8 = 50; // Encapsulating Security Payload

// Helper function to get protocol name
fn get_protocol_name(protocol: u8) -> &'static str {
    match protocol {
        IPSEC_AH => "Authentication Header (AH)",
        IPSEC_ESP => "Encapsulating Security Payload (ESP)",
        _ => "Unknown",
    }
}

// Helper function to parse AH header
fn parse_ah_header(reader: &mut Reader) -> Result<(u32, u32, u8)> {
    let next_header = reader.read_u8()?;
    let payload_len = reader.read_u8()?;
    let _reserved = reader.read_u16()?;
    let spi = reader.read_u32()?;
    let seq_num = reader.read_u32()?;
    
    // Skip the ICV (Integrity Check Value) - variable length
    // The payload_len field indicates the length of the AH header in 32-bit words, minus 2
    // So the total length in bytes is (payload_len + 2) * 4
    let total_ah_length = (payload_len as usize + 2) * 4;
    let icv_length = total_ah_length - 12; // 12 bytes already read
    reader.skip(icv_length)?;
    
    Ok((spi, seq_num, next_header))
}

// Helper function to parse ESP header
fn parse_esp_header(reader: &mut Reader) -> Result<(u32, u32)> {
    let spi = reader.read_u32()?;
    let seq_num = reader.read_u32()?;
    
    // The rest of the ESP packet is encrypted, so we can't parse it further
    // Just skip the remaining data
    reader.skip(reader.remaining())?;
    
    Ok((spi, seq_num))
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::IPSec(protocol, spi, seq_num) = &frame.protocol_field {
            let protocol_name = get_protocol_name(*protocol);
            return Some(format!("IPSec {}, SPI: 0x{:08x}, Seq: {}", protocol_name, spi, seq_num));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // Get the protocol type from the IP header's protocol field
        // This should be passed in via the context, but for now we'll assume it's in the frame
        let protocol = if let ProtocolInfoField::IP(_, _, _, _, protocol, _, _) = frame.protocol_field {
            protocol
        } else {
            return Ok(Protocol::None);
        };
        
        // Parse based on protocol type
        match protocol {
            IPSEC_AH => {
                // Parse Authentication Header
                let (spi, seq_num, next_header) = parse_ah_header(reader)?;
                
                // Store IPSec information in the frame
                frame.protocol_field = ProtocolInfoField::IPSec(IPSEC_AH, spi, seq_num);
                
                // Return the next protocol
                match next_header {
                    6 => Ok(Protocol::TCP),
                    17 => Ok(Protocol::UDP),
                    _ => Ok(Protocol::None),
                }
            },
            IPSEC_ESP => {
                // Parse Encapsulating Security Payload
                let (spi, seq_num) = parse_esp_header(reader)?;
                
                // Store IPSec information in the frame
                frame.protocol_field = ProtocolInfoField::IPSec(IPSEC_ESP, spi, seq_num);
                
                // ESP encrypts the payload, so we can't determine the next protocol
                Ok(Protocol::None)
            },
            _ => Ok(Protocol::None),
        }
    }

    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // Get the protocol type from the IP header's protocol field
        let protocol = if let ProtocolInfoField::IP(_, _, _, _, protocol, _, _) = frame.protocol_field {
            protocol
        } else {
            return Ok(Protocol::None);
        };
        
        match protocol {
            IPSEC_AH => {
                // Authentication Header
                let start_pos = reader.cursor;
                
                let next_header = reader.read_u8()?;
                let payload_len = reader.read_u8()?;
                let reserved = reader.read_u16()?;
                let spi = reader.read_u32()?;
                let seq_num = reader.read_u32()?;
                
                field_back_format!(list, reader, 1, format!("Next Header: {} ({})", next_header, get_next_header_name(next_header)));
                field_back_format!(list, reader, 1, format!("Payload Length: {} ({} bytes)", payload_len, (payload_len as u16 + 2) * 4));
                field_back_format!(list, reader, 2, format!("Reserved: 0x{:04x}", reserved));
                field_back_format!(list, reader, 4, format!("Security Parameters Index (SPI): 0x{:08x}", spi));
                field_back_format!(list, reader, 4, format!("Sequence Number: {}", seq_num));
                
                // Calculate ICV length and add a field for it
                let total_ah_length = (payload_len as usize + 2) * 4;
                let icv_length = total_ah_length - 12; // 12 bytes already read
                
                if icv_length > 0 && reader.remaining() >= icv_length {
                    field_back_format!(list, reader, icv_length, format!("Integrity Check Value (ICV): {} bytes", icv_length));
                }
                
                field.summary = format!("IPSec Authentication Header, SPI: 0x{:08x}, Seq: {}", spi, seq_num);
            },
            IPSEC_ESP => {
                // Encapsulating Security Payload
                let start_pos = reader.cursor;
                
                let spi = reader.read_u32()?;
                let seq_num = reader.read_u32()?;
                
                field_back_format!(list, reader, 4, format!("Security Parameters Index (SPI): 0x{:08x}", spi));
                field_back_format!(list, reader, 4, format!("Sequence Number: {}", seq_num));
                
                // The rest is encrypted payload, padding, pad length, next header, and optional ICV
                let encrypted_data_len = reader.remaining();
                if encrypted_data_len > 0 {
                    field_back_format!(list, reader, encrypted_data_len, format!("Encrypted Data: {} bytes", encrypted_data_len));
                }
                
                field.summary = format!("IPSec Encapsulating Security Payload, SPI: 0x{:08x}, Seq: {}", spi, seq_num);
            },
            _ => {
                field.summary = "Unknown IPSec Protocol".to_string();
            }
        }
        
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}

// Helper function to get next header protocol name
fn get_next_header_name(protocol: u8) -> &'static str {
    match protocol {
        1 => "ICMP",
        6 => "TCP",
        17 => "UDP",
        50 => "ESP",
        51 => "AH",
        58 => "ICMPv6",
        _ => "Unknown",
    }
}
