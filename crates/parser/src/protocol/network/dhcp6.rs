// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_backstep, common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    }
};
use anyhow::Result;

// DHCPv6 message types
pub fn dhcpv6_msg_type_mapper(code: u8) -> &'static str {
    match code {
        1 => "SOLICIT",
        2 => "ADVERTISE",
        3 => "REQUEST",
        4 => "CONFIRM",
        5 => "RENEW",
        6 => "REBIND",
        7 => "REPLY",
        8 => "RELEASE",
        9 => "DECLINE",
        10 => "RECONFIGURE",
        11 => "INFORMATION-REQUEST",
        12 => "RELAY-FORW",
        13 => "RELAY-REPL",
        14 => "LEASEQUERY",
        15 => "LEASEQUERY-REPLY",
        16 => "LEASEQUERY-DONE",
        17 => "LEASEQUERY-DATA",
        18 => "RECONFIGURE-REQUEST",
        19 => "RECONFIGURE-REPLY",
        20 => "DHCPV4-QUERY",
        21 => "DHCPV4-RESPONSE",
        22 => "ACTIVE-LEASEQUERY",
        23 => "START-TLS",
        24 => "BNDUPD",
        25 => "BNDREPLY",
        26 => "POOLREQ",
        27 => "POOLRESP",
        28 => "UPDREQ",
        29 => "UPDRESP",
        30 => "CONNECT",
        31 => "CONNECTREPLY",
        32 => "DISCONNECT",
        33 => "STATE",
        34 => "CONTACT",
        _ => "Unknown",
    }
}

// DHCPv6 option codes
pub fn dhcpv6_option_code_mapper(code: u16) -> &'static str {
    match code {
        1 => "CLIENTID",
        2 => "SERVERID",
        3 => "IA_NA",
        4 => "IA_TA",
        5 => "IAADDR",
        6 => "ORO",
        7 => "PREFERENCE",
        8 => "ELAPSED_TIME",
        9 => "RELAY_MSG",
        11 => "AUTH",
        12 => "UNICAST",
        13 => "STATUS_CODE",
        14 => "RAPID_COMMIT",
        15 => "USER_CLASS",
        16 => "VENDOR_CLASS",
        17 => "VENDOR_OPTS",
        18 => "INTERFACE_ID",
        19 => "RECONF_MSG",
        20 => "RECONF_ACCEPT",
        21 => "SIP_SERVER_D",
        22 => "SIP_SERVER_A",
        23 => "DNS_SERVERS",
        24 => "DOMAIN_LIST",
        25 => "IA_PD",
        26 => "IAPREFIX",
        27 => "NIS_SERVERS",
        28 => "NISP_SERVERS",
        29 => "NIS_DOMAIN_NAME",
        30 => "NISP_DOMAIN_NAME",
        31 => "SNTP_SERVERS",
        32 => "INFORMATION_REFRESH_TIME",
        33 => "BCMCS_SERVER_D",
        34 => "BCMCS_SERVER_A",
        36 => "GEOCONF_CIVIC",
        37 => "REMOTE_ID",
        38 => "SUBSCRIBER_ID",
        39 => "CLIENT_FQDN",
        40 => "PANA_AGENT",
        41 => "NEW_POSIX_TIMEZONE",
        42 => "NEW_TZDB_TIMEZONE",
        43 => "ERO",
        44 => "LQ_QUERY",
        45 => "CLIENT_DATA",
        46 => "CLT_TIME",
        47 => "LQ_RELAY_DATA",
        48 => "LQ_CLIENT_LINK",
        49 => "MIP6_HNINF",
        50 => "MIP6_RELAY",
        51 => "V6_LOST",
        52 => "CAPWAP_AC_V6",
        53 => "RELAY_ID",
        54 => "IPv6_Address-MoS",
        55 => "IPv6_FQDN-MoS",
        56 => "NTP_SERVER",
        57 => "V6_ACCESS_DOMAIN",
        58 => "SIP_UA_CS_LIST",
        59 => "BOOTFILE_URL",
        60 => "BOOTFILE_PARAM",
        61 => "CLIENT_ARCH_TYPE",
        62 => "NII",
        63 => "GEOLOCATION",
        64 => "AFTR_NAME",
        65 => "ERP_LOCAL_DOMAIN_NAME",
        66 => "RSOO",
        67 => "PD_EXCLUDE",
        68 => "VSS",
        69 => "MIP6_IDINF",
        70 => "MIP6_UDINF",
        71 => "MIP6_HNP",
        72 => "MIP6_HAA",
        73 => "MIP6_HAF",
        74 => "RDNSS_SELECTION",
        75 => "KRB_PRINCIPAL_NAME",
        76 => "KRB_REALM_NAME",
        77 => "KRB_DEFAULT_REALM_NAME",
        78 => "KRB_KDC",
        79 => "CLIENT_LINKLAYER_ADDR",
        80 => "LINK_ADDRESS",
        81 => "RADIUS",
        82 => "SOL_MAX_RT",
        83 => "INF_MAX_RT",
        84 => "ADDRSEL",
        85 => "ADDRSEL_TABLE",
        86 => "V6_PCP_SERVER",
        87 => "DHCPV4_MSG",
        88 => "DHCP4_O_DHCP6_SERVER",
        89 => "S46_RULE",
        90 => "S46_BR",
        91 => "S46_DMR",
        92 => "S46_V4V6BIND",
        93 => "S46_PORTPARAMS",
        94 => "S46_CONT_MAPE",
        95 => "S46_CONT_MAPT",
        96 => "S46_CONT_LW",
        97 => "4RD",
        98 => "4RD_MAP_RULE",
        99 => "4RD_NON_MAP_RULE",
        100 => "LQ_BASE_TIME",
        101 => "LQ_START_TIME",
        102 => "LQ_END_TIME",
        103 => "CAPTIVE_PORTAL",
        104 => "MPL_PARAMETERS",
        105 => "ANI_ATT",
        106 => "ANI_NETWORK_NAME",
        107 => "ANI_AP_NAME",
        108 => "ANI_AP_BSSID",
        109 => "ANI_OPERATOR_ID",
        110 => "ANI_OPERATOR_REALM",
        111 => "S46_PRIORITY",
        112 => "MUD_URL_V6",
        113 => "V6_PREFIX64",
        114 => "F_BINDING_STATUS",
        115 => "F_CONNECT_FLAGS",
        116 => "F_DNS_REMOVAL_INFO",
        117 => "F_DNS_HOST_NAME",
        118 => "F_DNS_ZONE_NAME",
        119 => "F_DNS_FLAGS",
        120 => "F_EXPIRATION_TIME",
        121 => "F_MAX_UNACKED_BNDUPD",
        122 => "F_MCLT",
        123 => "F_PARTNER_LIFETIME",
        124 => "F_PARTNER_LIFETIME_SENT",
        125 => "F_PARTNER_DOWN_TIME",
        126 => "F_PARTNER_RAW_CLT_TIME",
        127 => "F_PROTOCOL_VERSION",
        128 => "F_KEEPALIVE_TIME",
        129 => "F_RECONFIGURE_DATA",
        130 => "F_RELATIONSHIP_NAME",
        131 => "F_SERVER_FLAGS",
        132 => "F_SERVER_STATE",
        133 => "F_START_TIME_OF_STATE",
        134 => "F_STATE_EXPIRATION_TIME",
        143 => "IPV6_ADDRESS_ANDSF",
        _ => "Unknown",
    }
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::DHCPv6(msg_type, transaction_id) = &frame.protocol_field {
            let msg_type_str = dhcpv6_msg_type_mapper(*msg_type);
            return Some(format!("DHCPv6 {msg_type_str} (Transaction ID: 0x{transaction_id:06x})"));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        // DHCPv6 header starts with message type (1 byte)
        let msg_type = reader.read8()?;
        
        // Transaction ID (3 bytes)
        let b1 = reader.read8()? as u32;
        let b2 = reader.read8()? as u32;
        let b3 = reader.read8()? as u32;
        let transaction_id = (b1 << 16) | (b2 << 8) | b3;
        
        // Store DHCPv6 information in the frame
        frame.protocol_field = ProtocolInfoField::DHCPv6(msg_type, transaction_id);
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {        
        // Parse DHCPv6 header
        let msg_type = reader.read8()?;
        add_field_backstep!(field, reader, 1, format!("Message type: {} ({})", dhcpv6_msg_type_mapper(msg_type), msg_type));
        
        // Transaction ID (3 bytes)
        let b1 = reader.read8()? as u32;
        let b2 = reader.read8()? as u32;
        let b3 = reader.read8()? as u32;
        let transaction_id = (b1 << 16) | (b2 << 8) | b3;
        add_field_backstep!(field, reader, 3, format!("Transaction ID: 0x{:06x}", transaction_id));
        
        // Parse options
        while reader.left() >= 4 {
            let option_start = reader.cursor;
            let option_code = reader.read16(true)?;
            let option_len = reader.read16(true)? as usize;
            
            // Create option field
            let mut option_field = Field::with_children(
                format!("Option: ({}) {}", option_code, dhcpv6_option_code_mapper(option_code)),
                option_start,
                option_len + 4
            );
            
            add_field_backstep!(option_field, reader, 2, format!("Option code: {} ({})", dhcpv6_option_code_mapper(option_code), option_code));
            add_field_backstep!(option_field, reader, 2, format!("Option length: {}", option_len));
            
            // Parse specific options
            if option_len > 0 {
                let _data_start = reader.cursor;
                
                match option_code {
                    // Client ID or Server ID
                    1 | 2 => {
                        if reader.left() >= 2 {
                            let duid_type = reader.read16(true)?;
                            add_field_backstep!(option_field, reader, 2, format!("DUID type: {}", duid_type));
                            
                            // Skip remaining data
                            if option_len > 2 {
                                reader.slice(option_len - 2, true)?;
                                add_field_backstep!(option_field, reader, option_len - 2, "DUID data".to_string());
                            }
                        } else {
                            reader.slice(option_len, true)?;
                            add_field_backstep!(option_field, reader, option_len, "Option data".to_string());
                        }
                    },
                    // DNS Servers
                    // 23 => {
                    //     let mut dns_servers = String::new();
                    //     let addr_count = option_len / 16;
                        
                    //     for _ in 0..addr_count {
                    //         if reader.left() >= 16 {
                    //             let addr_bytes = reader.slice(16, true)?;
                    //             if let Ok(addr) = <[u8; 16]>::try_from(addr_bytes) {
                    //                 let dns_server = Ipv6Addr::from(addr);
                    //                 if !dns_servers.is_empty() {
                    //                     dns_servers.push_str(", ");
                    //                 }
                    //                 dns_servers.push_str(&dns_server.to_string());
                                    
                    //                 field_back_format!(option_list, reader, 16, format!("DNS server address: {}", dns_server));
                    //             }
                    //         }
                    //     }
                    // },
                    // Other options - just skip the data
                    _ => {
                        reader.slice(option_len, true)?;
                        add_field_backstep!(option_field, reader, option_len, "Option data".to_string());
                    }
                }
            }
            

            if let Some(list) = field.children.as_mut() {
                list.push(option_field);
            }
        }
        
        // Set summary
        field.summary = format!("DHCPv6 {} (Transaction ID: 0x{:06x})", dhcpv6_msg_type_mapper(msg_type), transaction_id);
        Ok(Protocol::None)
    }
}
