use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    },
    constants::ip_protocol_type_mapper,
    field_forward_format, read_field_format,
};
use anyhow::Result;

pub struct Visitor {}

pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let Some(ports) = &frame.ports {
            let source_port = ports.0;
            let target_port = ports.1;
            let payload_len = match frame.protocol_field {
                ProtocolInfoField::UDP(udp_len) => {
                    if udp_len < 8 {    
                        0
                    } else {
                        udp_len - 8
                    }
                },
                _ => 0,
            };
            return Some(format!("{} → {} Len={}", source_port, target_port, payload_len));
        }
        None
    }

    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let length = reader.read16(true)?;
        let _checksum = reader.read16(true)?;
        
        frame.ports = Some((source_port, target_port));
        frame.protocol_field = ProtocolInfoField::UDP(length);
        
        let next_protocol = match (source_port, target_port) {
            // (53, _) | (_, 53) => Protocol::DNS,          // DNS
            // (67, _) | (_, 67) | (68, _) | (_, 68) => Protocol::DHCP, // DHCP
            _ => Protocol::None,
        };
        
        Ok(next_protocol)
    }

    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = Vec::new();
        
        let source_port = read_field_format!(list, reader, reader.read16(true)?, "Source Port: {}");
        let target_port = read_field_format!(list, reader, reader.read16(true)?, "Destination Port: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Length: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x} [unverified]");
        
        // let next_protocol = match (source_port, target_port) {
        //     (53, _) | (_, 53) => {
        //         field_forward_format!(list, reader, payload_len, format!("DNS Message ({} bytes)", payload_len));
        //         Protocol::DNS
        //     },
        //     (67, _) | (_, 67) | (68, _) | (_, 68) => {
        //         field_forward_format!(list, reader, payload_len, format!("DHCP Message ({} bytes)", payload_len));
        //         Protocol::DHCP
        //     },
        //     (123, _) | (_, 123) => {
        //         field_forward_format!(list, reader, payload_len, format!("NTP Message ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        //     (161, _) | (_, 161) | (162, _) | (_, 162) => {
        //         field_forward_format!(list, reader, payload_len, format!("SNMP Message ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        //     (514, _) | (_, 514) => {
        //         field_forward_format!(list, reader, payload_len, format!("Syslog Message ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        //     (1900, _) | (_, 1900) => {
        //         field_forward_format!(list, reader, payload_len, format!("SSDP Message ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        //     (5353, _) | (_, 5353) => {
        //         field_forward_format!(list, reader, payload_len, format!("mDNS Message ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        //     _ => {
        //         field_forward_format!(list, reader, payload_len, format!("UDP Payload ({} bytes)", payload_len));
        //         Protocol::None
        //     },
        // };
        
        field.summary = format!("User Datagram Protocol, Src Port: {}, Dst Port: {}", source_port, target_port);
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}
