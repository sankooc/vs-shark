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
    constants::ip_protocol_type_mapper,
    read_field_format,
};
use anyhow::Result;

pub struct Visitor;

pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

fn detect_protocol(source_port: u16, target_port: u16) -> Protocol {
    match (source_port, target_port) {
        (53, _) | (_, 53) => Protocol::DNS,
        (5353, _) | (_, 5353) => Protocol::MDNS,
        (137, _) | (_, 137) => Protocol::NBNS,
        (67, _) | (_, 67) | (68, _) | (_, 68) => Protocol::DHCP,
        // (123, _) | (_, 123) => Protocol::None,
        // (161, _) | (_, 161) | (162, _) | (_, 162) => Protocol::None,
        // (514, _) | (_, 514) => Protocol::None,
        // (1900, _) | (_, 1900) => Protocol::None,
        // (5353, _) | (_, 5353) => Protocol::None,
        _ => Protocol::None,
    }
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
                }
                _ => 0,
            };
            return Some(format!("{} → {} Len={}", source_port, target_port, payload_len));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let source_port = reader.read16(true)?;
        let target_port = reader.read16(true)?;
        let length = reader.read16(true)?;
        let _checksum = reader.read16(true)?;

        frame.ports = Some((source_port, target_port));
        frame.protocol_field = ProtocolInfoField::UDP(length);

        
        let next_protocol = detect_protocol(source_port, target_port);
        Ok(next_protocol)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = Vec::new();

        let source_port = read_field_format!(list, reader, reader.read16(true)?, "Source Port: {}");
        let target_port = read_field_format!(list, reader, reader.read16(true)?, "Destination Port: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Length: {}");
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x} [unverified]");

        let next_protocol = detect_protocol(source_port, target_port);

        field.summary = format!("User Datagram Protocol, Src Port: {}, Dst Port: {}", source_port, target_port);
        field.children = Some(list);

        Ok(next_protocol)
    }
}
