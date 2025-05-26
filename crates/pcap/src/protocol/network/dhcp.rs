use std::net::Ipv4Addr;

use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::{MacAddress, Reader},
        Frame,
    },
    constants::{dhcp_option_type_mapper, dhcp_type_mapper},
    field_back_format, field_back_format_with_list, read_field_format,
};
use anyhow::Result;

// Helper function to format DHCP message type
pub fn message_type_str(msg_type: u8) -> String {
    format!("Message type: {} ({})", dhcp_type_mapper(msg_type), msg_type)
}

// Helper function to format DHCP option
pub fn option_type_str(option_type: u8) -> String {
    format!("Option: {} ({})", dhcp_option_type_mapper(option_type), option_type)
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::DHCP(msg_type) = &frame.protocol_field {
            return Some(format!("DHCP ({})", dhcp_type_mapper(*msg_type)));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // Skip to the DHCP part if we're coming from UDP
        // DHCP header starts with message type (1 byte)
        let _op = reader.read8()?; // Message type (1=request, 2=reply)
        let _htype = reader.read8()?; // Hardware address type (1=Ethernet)
        let hlen = reader.read8()?; // Hardware address length (6 for Ethernet)
        let _hops = reader.read8()?; // Hops

        let _xid = reader.read32(true)?; // Transaction ID
        let _secs = reader.read16(true)?; // Seconds elapsed
        let _flags = reader.read16(true)?; // Flags

        // IP addresses
        let _client_ip = reader.read_ip4()?; // Client IP address (ciaddr)
        let _your_ip = reader.read_ip4()?; // Your IP address (yiaddr)
        let _server_ip = reader.read_ip4()?; // Server IP address (siaddr)
        let _gateway_ip = reader.read_ip4()?; // Gateway IP address (giaddr)

        // frame.address_field = AddressField::IPv4(client_ip, server_ip);
        // Client hardware address
        reader.slice(hlen as usize, true)?;
        // let client_mac = MacAddress::from(<[u8; 6]>::try_from(client_mac_data)?);

        // Skip the rest of the chaddr field (10 bytes)
        reader.slice(10, true)?;

        // Server host name (64 bytes) and Boot file name (128 bytes)
        reader.slice(64, true)?; // Server host name
        reader.slice(128, true)?; // Boot file name

        // Magic cookie (should be 0x63825363)
        let _magic_cookie = reader.read32(true)?;

        // Parse DHCP options
        let mut msg_type: u8 = 0;

        // Parse options until we reach the end option (0xFF) or run out of data
        while reader.left() > 0 {
            let option_type = reader.read8()?;

            // End option
            if option_type == 255 {
                break;
            }

            // Pad option
            if option_type == 0 {
                continue;
            }

            // Read option length and data
            let option_len = reader.read8()? as usize;

            // Check for DHCP message type option (53)
            if option_type == 53 && option_len == 1 {
                msg_type = reader.read8()?;
            } else {
                // Skip other options
                reader.slice(option_len, true)?;
            }
        }

        // Store DHCP information in the frame
        frame.protocol_field = ProtocolInfoField::DHCP(msg_type);

        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];

        reader.forward(1);
        if let ProtocolInfoField::DHCP(msg_type) = &frame.protocol_field {
            field_back_format!(list, reader, 1, format!("Message Type: {} ({})", dhcp_type_mapper(*msg_type), msg_type));
        }
        let htype = reader.read8()?;
        field_back_format!(list, reader, 1, format!("Hardware type: {} ({})", htype, if htype == 1 { "Ethernet" } else { "Other" }));

        let hlen = read_field_format!(list, reader, reader.read8()?, "Hardware address length: {}");
        read_field_format!(list, reader, reader.read8()?, "Hops: {}");

        read_field_format!(list, reader, reader.read32(true)?, "Transaction ID: 0x{:08x}");
        read_field_format!(list, reader, reader.read16(true)?, "Seconds elapsed: {}");
        let flags = reader.read16(true)?;
        field_back_format!(
            list,
            reader,
            2,
            format!("Flags: 0x{:04x} ({})", flags, if (flags & 0x8000) != 0 { "Broadcast" } else { "Unicast" })
        );

        // IP addresses
        read_field_format!(list, reader, reader.read_ip4()?, "Client IP address: {}");
        read_field_format!(list, reader, reader.read_ip4()?, "Your (client) IP address: {}");
        read_field_format!(list, reader, reader.read_ip4()?, "Next server IP address: {}");
        read_field_format!(list, reader, reader.read_ip4()?, "Relay agent IP address: {}");

        // Client hardware address
        let client_mac_data = reader.slice(hlen as usize, true)?;
        let client_mac = MacAddress::from(<[u8; 6]>::try_from(client_mac_data)?);
        field_back_format!(list, reader, hlen as usize, format!("Client MAC address: {}", client_mac));

        // Skip the rest of the chaddr field
        reader.slice(16 - hlen as usize, true)?;
        field_back_format!(list, reader, 16 - hlen as usize, "Client hardware address padding".to_string());

        // Server host name and Boot file name
        let _sname = reader.slice(64, true)?;
        // let sname_len = sname.iter().position(|&x| x == 0).unwrap_or(64);
        // if sname_len > 0 {
        //     field_back_format!(list, reader, 64, format!("Server host name: {}",
        //         String::from_utf8_lossy(&sname[..sname_len])));
        // } else {
        //     field_back_format!(list, reader, 64, "Server host name not given".to_string());
        // }

        let file = reader.slice(128, true)?;
        let file_len = file.iter().position(|&x| x == 0).unwrap_or(128);
        if file_len > 0 {
            let _content = format!("Boot file name: {}", String::from_utf8_lossy(&file[..file_len]));
            field_back_format!(list, reader, 128, _content);
        } else {
            field_back_format!(list, reader, 128, "Boot file name not given".to_string());
        }

        // // Magic cookie
        let magic_cookie = reader.read32(true)?;
        field_back_format!(list, reader, 4, format!("Magic cookie: {:#010x}", magic_cookie));

        // Parse options until we reach the end option (0xFF) or run out of data
        while reader.left() > 0 {
            let start = reader.cursor;
            if let Ok((option_type, option_str, option_list)) = read_dhcp_option(reader) {
                let size = reader.cursor - start;
                field_back_format_with_list!(list, reader, size, format!("Option: ({}) {}", option_type, option_str), option_list);
            } else {
                break;
            }
        }

        if let ProtocolInfoField::DHCP(msg_type) = &frame.protocol_field {
            let msg_type_str = if *msg_type > 0 { dhcp_type_mapper(*msg_type) } else { "Unknown" };
            field.summary = format!("Dynamic Host Configuration Protocol ({})", msg_type_str);
        }
        field.children = Some(list);

        Ok(Protocol::None)
    }
}

fn read_dhcp_option(reader: &mut Reader) -> Result<(u8, String, Vec<Field>)> {
    let option_type = reader.read8()?;
    let mut options_list = vec![];
    // End option
    if option_type == 255 {
        field_back_format!(options_list, reader, 1, "Option: (255) End".to_string());
        return Ok((option_type, "Option: (255) End".to_string(), options_list));
    }

    // Pad option
    if option_type == 0 {
        field_back_format!(options_list, reader, 1, "Option: (0) Pad".to_string());
        return Ok((option_type, "Option: (0) Pad".to_string(), options_list));
    }

    // Read option length and data
    let option_len = read_field_format!(options_list, reader, reader.read8()?, "Option length: {}") as usize;
    let option_data = reader.slice(option_len, true)?;

    // Format specific options
    let option_str = match option_type {
        53 if option_len == 1 => {
            let msg_type = option_data[0];
            format!("DHCP Message Type: {} ({})", dhcp_type_mapper(msg_type), msg_type)
        }
        1 if option_len == 4 => {
            let subnet = Ipv4Addr::from([option_data[0], option_data[1], option_data[2], option_data[3]]);
            format!("Subnet Mask: {}", subnet)
        }
        3 => {
            let mut routers = String::new();
            for i in (0..option_len).step_by(4) {
                if option_len < i + 4 {
                    break;
                }
                let router = Ipv4Addr::from([option_data[i], option_data[i + 1], option_data[i + 2], option_data[i + 3]]);
                if !routers.is_empty() {
                    routers.push_str(", ");
                }
                routers.push_str(&router.to_string());
            }
            format!("Router: {}", routers)
        }
        6 if option_len % 4 == 0 => {
            let mut dns_servers = String::new();
            for i in 0..(option_len / 4) {
                let start = i * 4;
                let dns = Ipv4Addr::from([option_data[start], option_data[start + 1], option_data[start + 2], option_data[start + 3]]);
                if !dns_servers.is_empty() {
                    dns_servers.push_str(", ");
                }
                dns_servers.push_str(&dns.to_string());
            }
            format!("Domain Name Server: {}", dns_servers)
        }
        51 if option_len == 4 => {
            let lease_time = u32::from_be_bytes([option_data[0], option_data[1], option_data[2], option_data[3]]);
            format!("IP Address Lease Time: {} seconds", lease_time)
        }
        54 if option_len == 4 => {
            let server_id = Ipv4Addr::from([option_data[0], option_data[1], option_data[2], option_data[3]]);
            format!("DHCP Server Identifier: {}", server_id)
        }
        _ => format!("Option: ({}) {}, Length: {}", option_type, dhcp_option_type_mapper(option_type), option_len),
    };

    field_back_format!(options_list, reader, option_len, option_str.clone());
    Ok((option_type, option_str, options_list))
}
