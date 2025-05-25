use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{ProtocolInfoField, Protocol},
        io::Reader,
        Frame,
    },
    field_back_format, field_rest_format, read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub fn icmp_type_mapper(code: u8) -> &'static str {
    match code {
        0 => "Echo Reply",
        3 => "Destination Unreachable",
        4 => "Source Quench",
        5 => "Redirect",
        8 => "Echo Request",
        9 => "Router Advertisement",
        10 => "Router Solicitation",
        11 => "Time Exceeded",
        12 => "Parameter Problem",
        13 => "Timestamp",
        14 => "Timestamp Reply",
        15 => "Information Request",
        16 => "Information Reply",
        17 => "Address Mask Request",
        18 => "Address Mask Reply",
        30 => "Traceroute",
        _ => "Unknown",
    }
}

pub fn icmp_code_mapper(type_code: u8, code: u8) -> &'static str {
    match type_code {
        3 => match code {
            0 => "Net Unreachable",
            1 => "Host Unreachable",
            2 => "Protocol Unreachable",
            3 => "Port Unreachable",
            4 => "Fragmentation Needed and Don't Fragment was Set",
            5 => "Source Route Failed",
            6 => "Destination Network Unknown",
            7 => "Destination Host Unknown",
            8 => "Source Host Isolated",
            9 => "Communication with Destination Network is Administratively Prohibited",
            10 => "Communication with Destination Host is Administratively Prohibited",
            11 => "Destination Network Unreachable for Type of Service",
            12 => "Destination Host Unreachable for Type of Service",
            13 => "Communication Administratively Prohibited",
            14 => "Host Precedence Violation",
            15 => "Precedence cutoff in effect",
            _ => "Unknown",
        },
        5 => match code {
            0 => "Redirect Datagram for the Network",
            1 => "Redirect Datagram for the Host",
            2 => "Redirect Datagram for the Type of Service and Network",
            3 => "Redirect Datagram for the Type of Service and Host",
            _ => "Unknown",
        },
        11 => match code {
            0 => "Time to Live exceeded in Transit",
            1 => "Fragment Reassembly Time Exceeded",
            _ => "Unknown",
        },
        12 => match code {
            0 => "Pointer indicates the error",
            1 => "Missing a Required Option",
            2 => "Bad Length",
            _ => "Unknown",
        },
        13 => "timestamp message",
        14 => "timestamp reply message",
        15 => "Information Request",
        16 => "Information Reply",
        43 => match code {
            0 => "No Error",
            1 => "Malformed Query",
            2 => "No Such Interface",
            3 => "No Such Table Entry",
            4 => "Multiple Interfaces Satisfy Query",
            _ => "Unknown",
        },
        _ => "Unknown",
    }
}

pub fn t_icmp_type(type_code: u8) -> String {
    format!("Type: {} ({})", icmp_type_mapper(type_code), type_code)
}

pub fn t_icmp_code(type_code: u8, code: u8) -> String {
    format!("Code: {} ({})", icmp_code_mapper(type_code, code), code)
}

pub struct Visitor {}

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::Icmp(_type, _code) = &frame.protocol_field {
            let type_str = icmp_type_mapper(*_type);
            let code_str = icmp_code_mapper(*_type, *_code);
            Some(format!("{} ({})", type_str, code_str))
        } else {
            Some("Internet Control Message Protocol".to_string())
        }
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let _type = reader.read8()?;
        let code = reader.read8()?;
        frame.protocol_field = ProtocolInfoField::Icmp(_type, code);
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let mut list = vec![];

        let type_code = read_field_format_fn!(list, reader, reader.read8()?, t_icmp_type);
        let code = read_field_format_fn!(list, reader, reader.read8()?, |c| t_icmp_code(type_code, c));
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x}");

        match type_code {
            0 | 8 => {
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
                field_rest_format!(list, reader, format!("Data: {} bytes", reader.left()));
            }
            3 => {
                // Destination Unreachable
                if code == 4 {
                    reader.read16(true)?; // unused
                    read_field_format!(list, reader, reader.read16(true)?, "Next-hop MTU: {}");
                } else {
                    field_back_format!(list, reader, 4, "Unused".into());
                }
                field_rest_format!(list, reader, format!("Original Datagram: {} bytes", reader.left()));
            }
            5 => {
                // Redirect
                let _gateway = read_field_format!(list, reader, reader.read_ip4()?, "Gateway Address: {}");

                field_rest_format!(list, reader, format!("Original Datagram: {} bytes", reader.left()));
            }
            11 => {
                // Time Exceeded
                field_back_format!(list, reader, 4, "Unused".into());

                field_rest_format!(list, reader, format!("Original Datagram: {} bytes", reader.left()));
            }
            12 => {
                // Parameter Problem
                read_field_format!(list, reader, reader.read8()?, "Pointer: {}");
                field_back_format!(list, reader, 3, "Unused".into());
                field_rest_format!(list, reader, format!("Original Datagram: {} bytes", reader.left()));
            }
            13 | 14 => {
                // Timestamp 或 Timestamp Reply
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
                read_field_format!(list, reader, reader.read32(true)?, "Originate Timestamp: {} ms");
                read_field_format!(list, reader, reader.read32(true)?, "Receive Timestamp: {} ms");
                read_field_format!(list, reader, reader.read32(true)?, "Transmit Timestamp: {} ms");
            }
            15 | 16 => {
                // Information Request 或 Information Reply
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
            }
            17 | 18 => {
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
                read_field_format!(list, reader, reader.read_ip4()?, "Address Mask: {}");
            }
            _ => {
                field_rest_format!(list, reader, format!("Data: {} bytes", reader.left()));
            }
        }

        field.summary = "Internet Control Message Protocol".to_string();
        field.children = Some(list);

        Ok(Protocol::None)
    }
}
