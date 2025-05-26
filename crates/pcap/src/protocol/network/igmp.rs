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
use std::net::Ipv4Addr;

// IGMP Message Types
const IGMP_MEMBERSHIP_QUERY: u8 = 0x11;
const IGMP_MEMBERSHIP_REPORT_V1: u8 = 0x12;
const IGMP_MEMBERSHIP_REPORT_V2: u8 = 0x16;
const IGMP_LEAVE_GROUP: u8 = 0x17;
const IGMP_MEMBERSHIP_REPORT_V3: u8 = 0x22;

// Helper function to get IGMP message type as string
fn igmp_type_to_string(msg_type: u8) -> &'static str {
    match msg_type {
        IGMP_MEMBERSHIP_QUERY => "Membership Query",
        IGMP_MEMBERSHIP_REPORT_V1 => "Membership Report (v1)",
        IGMP_MEMBERSHIP_REPORT_V2 => "Membership Report (v2)",
        IGMP_LEAVE_GROUP => "Leave Group",
        IGMP_MEMBERSHIP_REPORT_V3 => "Membership Report (v3)",
        _ => "Unknown",
    }
}

// Helper function to determine IGMP version based on message type and length
fn determine_igmp_version(msg_type: u8, data_len: usize) -> u8 {
    match msg_type {
        IGMP_MEMBERSHIP_REPORT_V1 => 1,
        IGMP_MEMBERSHIP_REPORT_V2 | IGMP_LEAVE_GROUP => 2,
        IGMP_MEMBERSHIP_REPORT_V3 => 3,
        IGMP_MEMBERSHIP_QUERY => {
            // For queries, we need to check the length to determine version
            if data_len == 8 {
                1 // IGMPv1 query
            } else if data_len == 12 {
                3 // IGMPv3 query
            } else {
                2 // IGMPv2 query (or assume v2 if we can't determine)
            }
        }
        _ => 0, // Unknown version
    }
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::IGMP(version, msg_type, group_addr) = &frame.protocol_field {
            let type_str = igmp_type_to_string(*msg_type);
            let addr_str = format!("{}", group_addr);
            
            return Some(format!("Internet Group Management Protocol v{}, Type: {}, Group: {}", 
                               version, type_str, addr_str));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // IGMP header is at least 8 bytes
        if reader.remaining() < 8 {
            return Ok(Protocol::None);
        }
        
        // Read IGMP header fields
        let msg_type = reader.read_u8()?;
        let max_resp_time = reader.read_u8()?;
        let _checksum = reader.read_u16()?;
        
        // Read group address (4 bytes for IPv4)
        let group_addr_bytes = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        let group_addr = Ipv4Addr::new(
            group_addr_bytes[0],
            group_addr_bytes[1],
            group_addr_bytes[2],
            group_addr_bytes[3],
        );
        
        // Determine IGMP version based on message type and data length
        let version = determine_igmp_version(msg_type, reader.remaining() + 8);
        
        // Store IGMP information in the frame
        frame.protocol_field = ProtocolInfoField::IGMP(version, msg_type, group_addr);
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        // Save the initial cursor position
        let start_pos = reader.cursor;
        
        // Read IGMP header fields
        let msg_type = reader.read_u8()?;
        let max_resp_time = reader.read_u8()?;
        let checksum = reader.read_u16()?;
        
        // Read group address
        let group_addr_bytes = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        let group_addr = Ipv4Addr::new(
            group_addr_bytes[0],
            group_addr_bytes[1],
            group_addr_bytes[2],
            group_addr_bytes[3],
        );
        
        // Determine IGMP version
        let version = determine_igmp_version(msg_type, reader.remaining() + 8);
        
        // Format the header fields
        field_back_format!(list, reader, 1, format!("Type: {} (0x{:02x})", igmp_type_to_string(msg_type), msg_type));
        
        // Max Response Time interpretation depends on version
        if version == 1 {
            field_back_format!(list, reader, 1, format!("Max Response Time: Reserved (0x{:02x})", max_resp_time));
        } else {
            // For IGMPv2/v3, max_resp_time is in 1/10 seconds
            let resp_time_sec = if max_resp_time < 128 {
                max_resp_time as f32 / 10.0
            } else {
                // For values >= 128, there's a special encoding in IGMPv3
                let mant = max_resp_time & 0x0F;
                let exp = (max_resp_time & 0x70) >> 4;
                (mant | 0x10) as f32 * (1 << exp) as f32 / 10.0
            };
            field_back_format!(list, reader, 1, format!("Max Response Time: {:.1} sec (0x{:02x})", resp_time_sec, max_resp_time));
        }
        
        field_back_format!(list, reader, 2, format!("Checksum: 0x{:04x}", checksum));
        field_back_format!(list, reader, 4, format!("Group Address: {}", group_addr));
        
        // Parse IGMPv3-specific fields if present
        if version == 3 && msg_type == IGMP_MEMBERSHIP_QUERY && reader.remaining() >= 4 {
            let resv_s_qrv = reader.read_u8()?;
            let qqic = reader.read_u8()?;
            let num_sources = reader.read_u16()?;
            
            let s_flag = (resv_s_qrv & 0x08) != 0;
            let qrv = resv_s_qrv & 0x07;
            
            field_back_format!(list, reader, 1, format!("Reserved/S/QRV: 0x{:02x} (S={}, QRV={})", resv_s_qrv, s_flag, qrv));
            field_back_format!(list, reader, 1, format!("QQIC: {} sec", qqic));
            field_back_format!(list, reader, 2, format!("Number of Sources: {}", num_sources));
            
            // Parse source addresses if present
            for i in 0..num_sources {
                if reader.remaining() >= 4 {
                    let src_addr_bytes = [
                        reader.read_u8()?,
                        reader.read_u8()?,
                        reader.read_u8()?,
                        reader.read_u8()?,
                    ];
                    let src_addr = Ipv4Addr::new(
                        src_addr_bytes[0],
                        src_addr_bytes[1],
                        src_addr_bytes[2],
                        src_addr_bytes[3],
                    );
                    
                    field_back_format!(list, reader, 4, format!("Source Address {}: {}", i+1, src_addr));
                }
            }
        }
        
        // Set the summary
        let type_str = igmp_type_to_string(msg_type);
        field.summary = format!("Internet Group Management Protocol v{}, Type: {}, Group: {}", 
                              version, type_str, group_addr);
        
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}