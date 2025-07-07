// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_backstep, add_field_rest_format, add_sub_field_with_reader, common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    }
};
use anyhow::Result;

const SUMMARY: &str = "PPP-over-Ethernet Discovery";

const PADI: u8 = 0x09; // PPPoE Active Discovery Initiation
const PADO: u8 = 0x07; // PPPoE Active Discovery Offer
const PADR: u8 = 0x19; // PPPoE Active Discovery Request
const PADS: u8 = 0x65; // PPPoE Active Discovery Session-confirmation
const PADT: u8 = 0xa7; // PPPoE Active Discovery Terminate

const TAG_END_OF_LIST: u16 = 0x0000;
const TAG_SERVICE_NAME: u16 = 0x0101;
const TAG_AC_NAME: u16 = 0x0102;
const TAG_HOST_UNIQ: u16 = 0x0103;
const TAG_AC_COOKIE: u16 = 0x0104;
const TAG_VENDOR_SPECIFIC: u16 = 0x0105;
const TAG_RELAY_SESSION_ID: u16 = 0x0110;
const TAG_SERVICE_NAME_ERROR: u16 = 0x0201;
const TAG_AC_SYSTEM_ERROR: u16 = 0x0202;
const TAG_GENERIC_ERROR: u16 = 0x0203;

fn parse_tags(reader: &mut Reader, field: &mut Field) -> Result<()> {
    field.summary = "PPPoE Tags".to_string();
    
    while reader.left() >= 4 {
        let tag_type = reader.read16(true)?;
        let tag_length = reader.read16(true)?;
        
        let tag_name = match tag_type {
            TAG_END_OF_LIST => "End of List",
            TAG_SERVICE_NAME => "Service-Name",
            TAG_AC_NAME => "AC-Name",
            TAG_HOST_UNIQ => "Host-Uniq",
            TAG_AC_COOKIE => "AC-Cookie",
            TAG_VENDOR_SPECIFIC => "Vendor-Specific",
            TAG_RELAY_SESSION_ID => "Relay-Session-Id",
            TAG_SERVICE_NAME_ERROR => "Service-Name-Error",
            TAG_AC_SYSTEM_ERROR => "AC-System-Error",
            TAG_GENERIC_ERROR => "Generic-Error",
            _ => "Unknown",
        };
        
        if tag_length > 0 && reader.left() >= tag_length as usize {
            match tag_type {
                TAG_SERVICE_NAME | TAG_AC_NAME => {
                    if let Ok(tag_value) = reader.read_string(tag_length as usize) {
                        add_field_backstep!(field, reader, tag_length as usize, format!("{}: {}", tag_name, tag_value));
                    } else {
                        let tag_data = reader.slice(tag_length as usize, true)?;
                        let hex_str = tag_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                        add_field_backstep!(field, reader, tag_length as usize, format!("{}: 0x{}", tag_name, hex_str));
                    }
                },
                TAG_HOST_UNIQ | TAG_AC_COOKIE | TAG_RELAY_SESSION_ID => {
                    // 显示为十六进制
                    let tag_data = reader.slice(tag_length as usize, true)?;
                    let hex_str = tag_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                    add_field_backstep!(field, reader, tag_length as usize, format!("{}: 0x{}", tag_name, hex_str));
                },
                TAG_VENDOR_SPECIFIC => {
                    if tag_length >= 4 {
                        let vendor_id = reader.read32(true)?;
                        add_field_backstep!(field, reader, 4, format!("{}: Vendor ID: {:#010x}", tag_name, vendor_id));
                        
                        if tag_length > 4 {
                            let vendor_data = reader.slice((tag_length - 4) as usize, true)?;
                            let hex_str = vendor_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                            add_field_backstep!(field, reader, (tag_length - 4) as usize, format!("Vendor Data: 0x{}", hex_str));
                        }
                    } else {
                        let tag_data = reader.slice(tag_length as usize, true)?;
                        let hex_str = tag_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                        add_field_backstep!(field, reader, tag_length as usize, format!("{}: 0x{}", tag_name, hex_str));
                    }
                },
                TAG_SERVICE_NAME_ERROR | TAG_AC_SYSTEM_ERROR | TAG_GENERIC_ERROR => {
                    if let Ok(error_msg) = reader.read_string(tag_length as usize) {
                        add_field_backstep!(field, reader, tag_length as usize, format!("{}: {}", tag_name, error_msg));
                    } else {
                        let tag_data = reader.slice(tag_length as usize, true)?;
                        let hex_str = tag_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                        add_field_backstep!(field, reader, tag_length as usize, format!("{}: 0x{}", tag_name, hex_str));
                    }
                },
                _ => {
                    let tag_data = reader.slice(tag_length as usize, true)?;
                    let hex_str = tag_data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ");
                    add_field_backstep!(field, reader, tag_length as usize, format!("{} (0x{:04x}): 0x{}", tag_name, tag_type, hex_str));
                }
            }
        } else if tag_length == 0 {
            add_field_backstep!(field, reader, 0, format!("{}", tag_name));
        } else {
            add_field_rest_format!(field, reader, format!("Incomplete {} Tag", tag_name));
            break;
        }
    }
    
    if reader.left() > 0 {
        add_field_rest_format!(field, reader, format!("Trailing Data: {} bytes", reader.left()));
    }
    
    Ok(())
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        match frame.protocol_field {
            ProtocolInfoField::PPPoES(Some(code)) => {
                let msg = match code {
                    PADI => "Active Discovery Initiation (PADI)",
                    PADO => "Active Discovery Offer (PADO)",
                    PADR => "Active Discovery Request (PADR)",
                    PADS => "Active Discovery Session-confirmation (PADS)",
                    PADT => "Active Discovery Terminate (PADT)",
                    _ => SUMMARY,
                };
                Some(msg.to_string())
            },
            _ => Some(SUMMARY.to_string()),
        }
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _version_type = reader.read8()?;
        let code = reader.read8()?;
        reader.forward(4);
        
        frame.protocol_field = ProtocolInfoField::PPPoES(Some(code));
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        
        let version_type = reader.read8()?;
        let version = (version_type >> 4) & 0x0F;
        let type_val = version_type & 0x0F;
        add_field_backstep!(field, reader, 1, format!("Version: {}, Type: {}", version, type_val));
        
        let code = reader.read8()?;
        let code_name = match code {
            PADI => "PADI (PPPoE Active Discovery Initiation)",
            PADO => "PADO (PPPoE Active Discovery Offer)",
            PADR => "PADR (PPPoE Active Discovery Request)",
            PADS => "PADS (PPPoE Active Discovery Session-confirmation)",
            PADT => "PADT (PPPoE Active Discovery Terminate)",
            _ => "Unknown",
        };
        add_field_backstep!(field, reader, 1, format!("Code: {} (0x{:02x})", code_name, code));
        
        let session_id = reader.read16(true)?;
        add_field_backstep!(field, reader, 2, format!("Session ID: {:#06x}", session_id));
        
        let payload_length = reader.read16(true)?;
        add_field_backstep!(field, reader, 2, format!("Payload Length: {} bytes", payload_length));
        
        if reader.left() > 0 {
            add_sub_field_with_reader!(field, reader, parse_tags)?;
            // let tags = parse_tags(reader)?;
            // if !tags.is_empty() {
            //     let tags_field = Field{ summary: "PPPoE Tags".to_string(), children: Some(tags), ..Default::default() };
            //     list.push(tags_field);
            // }
        }
        
        field.summary = match code {
            PADI => "Active Discovery Initiation (PADI)",
            PADO => "Active Discovery Offer (PADO)",
            PADR => "Active Discovery Request (PADR)",
            PADS => "Active Discovery Session-confirmation (PADS)",
            PADT => "Active Discovery Terminate (PADT)",
            _ => SUMMARY,
        }.to_string();
        Ok(Protocol::None)
    }
}
