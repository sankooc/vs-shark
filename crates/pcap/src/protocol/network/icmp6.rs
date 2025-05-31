use crate::{
    common::{concept::Field, core::Context, enum_def::{ProtocolInfoField, Protocol}, io::Reader, Frame},
    field_back_format, field_rest_format, read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub fn icmp6_type_mapper(code: u8) -> &'static str {
    match code {
        1 => "Destination Unreachable",
        2 => "Packet Too Big",
        3 => "Time Exceeded",
        4 => "Parameter Problem",
        128 => "Echo Request",
        129 => "Echo Reply",
        130 => "Multicast Listener Query",
        131 => "Multicast Listener Report",
        132 => "Multicast Listener Done",
        133 => "Router Solicitation",
        134 => "Router Advertisement",
        135 => "Neighbor Solicitation",
        136 => "Neighbor Advertisement",
        137 => "Redirect Message",
        138 => "Router Renumbering",
        139 => "ICMP Node Information Query",
        140 => "ICMP Node Information Response",
        141 => "Inverse Neighbor Discovery Solicitation",
        142 => "Inverse Neighbor Discovery Advertisement",
        143 => "Version 2 Multicast Listener Report",
        144 => "Home Agent Address Discovery Request",
        145 => "Home Agent Address Discovery Reply",
        146 => "Mobile Prefix Solicitation",
        147 => "Mobile Prefix Advertisement",
        148 => "Certification Path Solicitation",
        149 => "Certification Path Advertisement",
        151 => "Multicast Router Advertisement",
        152 => "Multicast Router Solicitation",
        153 => "Multicast Router Termination",
        155 => "RPL Control Message",
        _ => "Unknown",
    }
}

pub fn icmp6_code_mapper(type_code: u8, code: u8) -> &'static str {
    match type_code {
        1 => match code {
            0 => "No Route to Destination",
            1 => "Communication with Destination Administratively Prohibited",
            2 => "Beyond Scope of Source Address",
            3 => "Address Unreachable",
            4 => "Port Unreachable",
            5 => "Source Address Failed Ingress/Egress Policy",
            6 => "Reject Route to Destination",
            7 => "Error in Source Routing Header",
            _ => "Unknown",
        },
        3 => match code {
            0 => "Hop Limit Exceeded in Transit",
            1 => "Fragment Reassembly Time Exceeded",
            _ => "Unknown",
        },
        4 => match code {
            0 => "Erroneous Header Field Encountered",
            1 => "Unrecognized Next Header Type Encountered",
            2 => "Unrecognized IPv6 Option Encountered",
            _ => "Unknown",
        },
        137 => match code {
            0 => "Redirect for Network",
            1 => "Redirect for Host",
            2 => "Redirect for Type of Service and Network",
            3 => "Redirect for Type of Service and Host",
            _ => "Unknown",
        },
        _ => "Unknown",
    }
}

pub fn t_icmp6_type(type_code: u8) -> String {
    format!("Type: {} ({})", icmp6_type_mapper(type_code), type_code)
}

pub fn t_icmp6_code(type_code: u8, code: u8) -> String {
    format!("Code: {} ({})", icmp6_code_mapper(type_code, code), code)
}

fn parse_icmpv6_options(list: &mut Vec<Field>, reader: &mut Reader) -> Result<()> {
    if reader.left() == 0 {
        return Ok(());
    }
    
    let options_start = reader.cursor;
    let mut options_list = Vec::new();
    
    while reader.left() > 0 {
        let option_start = reader.cursor;
        let option_type = reader.read8()?;
        let option_len = reader.read8()? as usize;
        
        if option_len == 0 {
            
            break;
        }
        
        let total_len = option_len * 8;
        let data_len = total_len - 2; 
        
        if reader.left() < data_len {
            
            break;
        }
        
        let mut option_field = Field::with_children(format!("Option: Type {} (Length: {} bytes)", option_type_to_string(option_type), total_len), option_start, total_len); 
        match option_type {
            1 => {
                if let Some(children) = &mut option_field.children {
                    children.push(Field::label(format!("Source Link-layer Address: {}", format_mac_address(reader, data_len)?), reader.cursor, reader.cursor + data_len));
                }
                reader.forward(data_len);
            },
            2 => {
                if let Some(children) = &mut option_field.children {
                    children.push(Field::label(format!("Target Link-layer Address: {}", format_mac_address(reader, data_len)?), reader.cursor, reader.cursor + data_len));
                }
                reader.forward(data_len);
            },
            3 => {
                if let Some(children) = &mut option_field.children {
                    let prefix_len = reader.read8()?;
                    let flags = reader.read8()?;
                    let l_flag = (flags >> 7) & 0x01;
                    let a_flag = (flags >> 6) & 0x01;
                    
                    children.push(Field::label(format!("Prefix Length: {}", prefix_len), reader.cursor - 2, reader.cursor));
                    
                    children.push(Field::label(format!("Flags: {:#04x} (L:{}, A:{})", flags, l_flag, a_flag), reader.cursor - 1, reader.cursor));
                    
                    
                    let valid_lifetime = reader.read32(true)?;
                    children.push(Field::label(format!("Valid Lifetime: {} seconds", valid_lifetime), reader.cursor - 4, reader.cursor));
                    
                    
                    let preferred_lifetime = reader.read32(true)?;
                    children.push(Field::label(format!("Preferred Lifetime: {} seconds", preferred_lifetime), reader.cursor - 4, reader.cursor));

                    reader.forward(4); 
                    
                    let prefix = reader.read_ip6()?;
                    children.push(Field::label(format!("Prefix: {}", prefix), reader.cursor - 16, reader.cursor));
                }
            },
            4 => {
                reader.forward(6);
                if let Some(children) = &mut option_field.children {
                    children.push(Field::label(format!("Redirected Header: {} bytes", data_len - 6), reader.cursor, reader.cursor + data_len - 6));
                }
                reader.forward(data_len - 6);
            },
            5 => {
                reader.forward(2); 
                let mtu = reader.read32(true)?;
                if let Some(children) = &mut option_field.children {
                    children.push(Field::label(format!("MTU: {}", mtu), reader.cursor - 4, reader.cursor));
                }
            },
            _ => {
                
                reader.forward(data_len);
            }
        }
        
        options_list.push(option_field);
    }
    
    if !options_list.is_empty() {
        let options_field = Field::new(format!("Options: {} bytes", reader.cursor - options_start), options_start, reader.cursor, options_list);
        list.push(options_field);
    }
    
    Ok(())
}

fn option_type_to_string(option_type: u8) -> &'static str {
    match option_type {
        1 => "Source Link-layer Address",
        2 => "Target Link-layer Address",
        3 => "Prefix Information",
        4 => "Redirected Header",
        5 => "MTU",
        6 => "NBMA Shortcut Limit",
        7 => "Advertisement Interval",
        8 => "Home Agent Information",
        9 => "Source Address List",
        10 => "Target Address List",
        11 => "CGA",
        12 => "RSA Signature",
        13 => "Timestamp",
        14 => "Nonce",
        15 => "Trust Anchor",
        16 => "Certificate",
        17 => "IP Address/Prefix",
        18 => "New Router Prefix Information",
        19 => "Link-layer Address",
        20 => "Neighbor Advertisement Acknowledgment",
        24 => "MAP",
        25 => "Route Information",
        26 => "RDNSS",
        31 => "DNSSL",
        _ => "Unknown",
    }
}

fn format_mac_address(reader: &mut Reader, len: usize) -> Result<String> {
    if len < 6 {
        return Ok("Invalid MAC Address".to_string());
    }
    
    let b1 = reader.read8()?;
    let b2 = reader.read8()?;
    let b3 = reader.read8()?;
    let b4 = reader.read8()?;
    let b5 = reader.read8()?;
    let b6 = reader.read8()?;
    
    if len > 6 {
        reader.forward(len - 6);
    }
    
    Ok(format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", b1, b2, b3, b4, b5, b6))
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, _: &Frame) -> Option<String> {
        // if let InfoField::Icmp6(_type, _code) = &frame.info_field {
        //     let type_str = icmp6_type_mapper(*_type);
        //     let code_str = icmp6_code_mapper(*_type, *_code);
        //     Some(format!("ICMPv6 {} ({})", type_str, code_str))
        // } else {
        //     Some("Internet Control Message Protocol v6".to_string())
        // }
        Some("Internet Control Message Protocol v6".to_string())
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let _type = reader.read8()?;
        let code = reader.read8()?;
        frame.protocol_field = ProtocolInfoField::Icmp6(_type, code);
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _start = reader.left();
        let mut list = vec![];

        let type_code = read_field_format_fn!(list, reader, reader.read8()?, t_icmp6_type);
        read_field_format_fn!(list, reader, reader.read8()?, |c| t_icmp6_code(type_code, c));
        read_field_format!(list, reader, reader.read16(true)?, "Checksum: {:#06x}");

        match type_code {
            // Echo Request/Reply
            128 | 129 => {
                read_field_format!(list, reader, reader.read16(true)?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Sequence Number: {}");
                field_rest_format!(list, reader, format!("Data: {} bytes", reader.left()));
            },
            // Packet Too Big
            2 => {
                read_field_format!(list, reader, reader.read32(true)?, "MTU: {}");
                field_rest_format!(list, reader, format!("Original Packet: {} bytes", reader.left()));
            },
            // Destination Unreachable
            1 => {
                field_back_format!(list, reader, 4, "Unused".into());
                field_rest_format!(list, reader, format!("Original Packet: {} bytes", reader.left()));
            },
            // Time Exceeded
            3 => {
                field_back_format!(list, reader, 4, "Unused".into());
                field_rest_format!(list, reader, format!("Original Packet: {} bytes", reader.left()));
            },
            // Parameter Problem
            4 => {
                read_field_format!(list, reader, reader.read32(true)?, "Pointer: {}");
                field_rest_format!(list, reader, format!("Original Packet: {} bytes", reader.left()));
            },
            // Router Solicitation
            133 => {
                field_back_format!(list, reader, 4, "Reserved".into());
                parse_icmpv6_options(&mut list, reader)?;
            },
            // Router Advertisement
            134 => {
                read_field_format!(list, reader, reader.read8()?, "Cur Hop Limit: {}");
                let flags = reader.read8()?;
                let m_flag = (flags >> 7) & 0x01;
                let o_flag = (flags >> 6) & 0x01;
                let h_flag = (flags >> 5) & 0x01;
                let prf = (flags >> 3) & 0x03;
                let p_flag = (flags >> 2) & 0x01;
                field_back_format!(list, reader, 1, format!("Flags: {:#04x} (M:{}, O:{}, H:{}, Prf:{}, P:{})", flags, m_flag, o_flag, h_flag, prf, p_flag));

                read_field_format!(list, reader, reader.read16(true)?, "Router Lifetime: {} seconds");
                read_field_format!(list, reader, reader.read32(true)?, "Reachable Time: {} milliseconds");
                read_field_format!(list, reader, reader.read32(true)?, "Retrans Timer: {} milliseconds");
                parse_icmpv6_options(&mut list, reader)?;
            },
            // Neighbor Solicitation
            135 => {
                field_back_format!(list, reader, 4, "Reserved".into());
                read_field_format!(list, reader, reader.read_ip6()?, "Target Address: {}");
                parse_icmpv6_options(&mut list, reader)?;
            },
            // Neighbor Advertisement
            136 => {
                let flags = reader.read32(true)?;
                let r_flag = (flags >> 31) & 0x01;
                let s_flag = (flags >> 30) & 0x01;
                let o_flag = (flags >> 29) & 0x01;
                field_back_format!(list, reader, 4, format!("Flags: {:#010x} (R:{}, S:{}, O:{})", flags, r_flag, s_flag, o_flag));
                read_field_format!(list, reader, reader.read_ip6()?, "Target Address: {}");
                parse_icmpv6_options(&mut list, reader)?;
            },
            // Redirect Message
            137 => {
                field_back_format!(list, reader, 4, "Reserved".into());
                read_field_format!(list, reader, reader.read_ip6()?, "Target Address: {}");
                read_field_format!(list, reader, reader.read_ip6()?, "Destination Address: {}");
                parse_icmpv6_options(&mut list, reader)?;
            },
            // Multicast Listener Query/Report/Done
            130 | 131 | 132 => {
                if type_code == 130 {
                    read_field_format!(list, reader, reader.read16(true)?, "Maximum Response Delay: {} milliseconds");
                    field_back_format!(list, reader, 2, "Reserved".into());
                } else {
                    field_back_format!(list, reader, 4, "Reserved".into());
                }
                read_field_format!(list, reader, reader.read_ip6()?, "Multicast Address: {}");
            },
            // 其他ICMPv6消息类型
            _ => {
                field_rest_format!(list, reader, format!("Data: {} bytes", reader.left()));
            }
        }

        field.summary = "Internet Control Message Protocol v6".to_string();
        field.children = Some(list);

        Ok(Protocol::None)
    }
}