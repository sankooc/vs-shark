// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_backstep, add_field_format, add_field_rest_format, add_sub_field_with_reader, common::{
        concept::Field,
        core::Context,
        enum_def::{Protocol, ProtocolInfoField},
        io::Reader,
        Frame,
    }, constants::{ppp_lcp_type_mapper, ppp_type_mapper}
};
use anyhow::Result;

const SUMMARY: &str = "PPP-over-Ethernet Session";


fn read_payload(reader: &mut Reader, field: &mut Field) -> Result<u16> {
    let protocol = reader.read16(true)?;
    let protocol_name = ppp_type_mapper(protocol);

    match protocol {
        0xc021 => {
            if reader.left() >= 4 {
                let code = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("LCP Code: {} ({})", ppp_lcp_type_mapper(code), code));
                add_field_format!(field, reader, reader.read8()?, "Identifier: {}");
                add_field_format!(field, reader, reader.read16(true)?, "Length: {} bytes");
                add_field_rest_format!(field, reader, format!("LCP Data: {} bytes", reader.left()));
            } else {
                reader.forward(reader.left());
                add_field_rest_format!(field, reader, "Incomplete LCP Header".to_string());
            }
        }
        0xc023 => {
            if reader.left() >= 4 {
                let code = reader.read8()?;
                let pap_code = match code {
                    1 => "Authenticate-Request",
                    2 => "Authenticate-Ack",
                    3 => "Authenticate-Nak",
                    _ => "Unknown",
                };
                add_field_backstep!(field, reader, 1, format!("PAP Code: {} ({})", pap_code, code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                add_field_rest_format!(field, reader, format!("PAP Data: {} bytes", reader.left()));
            } else {
                add_field_rest_format!(field, reader, "Incomplete PAP Header".to_string());
            }
        }
        0xc223 => {
            if reader.left() >= 4 {
                let code = reader.read8()?;
                let chap_code = match code {
                    1 => "Challenge",
                    2 => "Response",
                    3 => "Success",
                    4 => "Failure",
                    _ => "Unknown",
                };
                add_field_backstep!(field, reader, 1, format!("CHAP Code: {} ({})", chap_code, code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                add_field_rest_format!(field, reader, format!("CHAP Data: {} bytes", reader.left()));
            } else {
                add_field_rest_format!(field, reader, "Incomplete CHAP Header".to_string());
            }
        }
        0x8021 => {
            // IPCP (IP Control Protocol)
            if reader.left() >= 4 {
                let code = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("IPCP Code: {} ({})", ppp_lcp_type_mapper(code), code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                // Parse IPCP options if available
                if reader.left() > 0 && code == 1 {
                    let mut options_data = vec![];
                    let mut remaining = reader.left();

                    while remaining >= 2 {
                        let option_type = reader.read8()?;
                        let option_length = reader.read8()?;
                        remaining -= 2;

                        if option_length > 2 && remaining >= (option_length as usize - 2) {
                            match option_type {
                                1 => {
                                    // IP-Address
                                    if option_length == 6 {
                                        let ip = reader.read_ip4()?;
                                        options_data.push(format!("IP-Address: {ip}"));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("IP-Address (Invalid Length: {option_length})"));
                                    }
                                }
                                2 => {
                                    // IP-Compression-Protocol
                                    if option_length >= 4 {
                                        let protocol = reader.read16(true)?;
                                        options_data.push(format!("IP-Compression-Protocol: {protocol:#06x}"));
                                        reader.forward(option_length as usize - 4);
                                        remaining -= option_length as usize - 2;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("IP-Compression-Protocol (Invalid Length: {option_length})"));
                                    }
                                }
                                3 => {
                                    // IP-Address (Primary DNS)
                                    if option_length == 6 {
                                        let ip = reader.read_ip4()?;
                                        options_data.push(format!("Primary DNS: {ip}"));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Primary DNS (Invalid Length: {option_length})"));
                                    }
                                }
                                129 => {
                                    // IP-Address (Secondary DNS)
                                    if option_length == 6 {
                                        let ip = reader.read_ip4()?;
                                        options_data.push(format!("Secondary DNS: {ip}"));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Secondary DNS (Invalid Length: {option_length})"));
                                    }
                                }
                                _ => {
                                    reader.forward(option_length as usize - 2);
                                    remaining -= option_length as usize - 2;
                                    options_data.push(format!("Option Type: {option_type}, Length: {option_length}"));
                                }
                            }
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {option_type}, Length {option_length}"));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        add_field_backstep!(field, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    add_field_rest_format!(field, reader, format!("IPCP Data: {} bytes", reader.left()));
                }
            } else {
                add_field_rest_format!(field, reader, "Incomplete IPCP Header".to_string());
            }
        }
        0x8057 => {
            // IPv6CP (IPv6 Control Protocol)
            if reader.left() >= 4 {
                let code = reader.read8()?;
                let ipv6cp_code = match code {
                    1 => "Configure-Request",
                    2 => "Configure-Ack",
                    3 => "Configure-Nak",
                    4 => "Configure-Reject",
                    5 => "Terminate-Request",
                    6 => "Terminate-Ack",
                    7 => "Code-Reject",
                    _ => "Unknown",
                };
                add_field_backstep!(field, reader, 1, format!("IPv6CP Code: {} ({})", ipv6cp_code, code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                // Parse IPv6CP options if available
                if reader.left() > 0 && code == 1 {
                    let mut options_data = vec![];
                    let mut remaining = reader.left();

                    while remaining >= 2 {
                        let option_type = reader.read8()?;
                        let option_length = reader.read8()?;
                        remaining -= 2;

                        if option_length > 2 && remaining >= (option_length as usize - 2) {
                            match option_type {
                                1 => {
                                    // Interface-Identifier
                                    if option_length == 10 {
                                        let id_high = reader.read32(true)?;
                                        let id_low = reader.read32(true)?;
                                        options_data.push(format!("Interface-Identifier: {id_high:#010x}{id_low:08x}"));
                                        remaining -= 8;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Interface-Identifier (Invalid Length: {option_length})"));
                                    }
                                }
                                _ => {
                                    reader.forward(option_length as usize - 2);
                                    remaining -= option_length as usize - 2;
                                    options_data.push(format!("Option Type: {option_type}, Length: {option_length}"));
                                }
                            }
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {option_type}, Length {option_length}"));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        add_field_backstep!(field, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    add_field_rest_format!(field, reader, format!("IPv6CP Data: {} bytes", reader.left()));
                }
            } else {
                add_field_rest_format!(field, reader, "Incomplete IPv6CP Header".to_string());
            }
        }
        0x80fd => {
            // CCP (Compression Control Protocol)
            if reader.left() >= 4 {
                let code = reader.read8()?;
                let ccp_code = match code {
                    1 => "Configure-Request",
                    2 => "Configure-Ack",
                    3 => "Configure-Nak",
                    4 => "Configure-Reject",
                    5 => "Terminate-Request",
                    6 => "Terminate-Ack",
                    7 => "Code-Reject",
                    14 => "Reset-Request",
                    15 => "Reset-Ack",
                    _ => "Unknown",
                };
                add_field_backstep!(field, reader, 1, format!("CCP Code: {} ({})", ccp_code, code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                // Parse CCP options if available
                if reader.left() > 0 && (code == 1 || code == 2 || code == 3 || code == 4) {
                    let mut options_data = vec![];
                    let mut remaining = reader.left();

                    while remaining >= 2 {
                        let option_type = reader.read8()?;
                        let option_length = reader.read8()?;
                        remaining -= 2;

                        if option_length > 2 && remaining >= (option_length as usize - 2) {
                            let compression_type = match option_type {
                                1 => "Predictor type 1",
                                2 => "Predictor type 2",
                                3 => "Puddle Jumper",
                                4 => "HPPJ",
                                5 => "Stac-LZS",
                                17 => "MPPC",
                                18 => "MPPE",
                                19 => "DEFLATE",
                                20 => "V.42bis Compression",
                                21 => "BSD Compression",
                                26 => "LZS-DCP",
                                _ => "Unknown",
                            };
                            options_data.push(format!("Compression Method: {compression_type} ({option_type})"));
                            reader.forward(option_length as usize - 2);
                            remaining -= option_length as usize - 2;
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {option_type}, Length {option_length}"));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        add_field_backstep!(field, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    add_field_rest_format!(field, reader, format!("CCP Data: {} bytes", reader.left()));
                }
            } else {
                add_field_rest_format!(field, reader, "Incomplete CCP Header".to_string());
            }
        }
        0x8053 => {
            // ECP (Encryption Control Protocol)
            if reader.left() >= 4 {
                let code = reader.read8()?;
                let ecp_code = match code {
                    1 => "Configure-Request",
                    2 => "Configure-Ack",
                    3 => "Configure-Nak",
                    4 => "Configure-Reject",
                    5 => "Terminate-Request",
                    6 => "Terminate-Ack",
                    7 => "Code-Reject",
                    _ => "Unknown",
                };
                add_field_backstep!(field, reader, 1, format!("ECP Code: {} ({})", ecp_code, code));

                let identifier = reader.read8()?;
                add_field_backstep!(field, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                add_field_backstep!(field, reader, 2, format!("Length: {} bytes", length));

                // Parse ECP options if available
                if reader.left() > 0 && (code == 1 || code == 2 || code == 3 || code == 4) {
                    let mut options_data = vec![];
                    let mut remaining = reader.left();

                    while remaining >= 2 {
                        let option_type = reader.read8()?;
                        let option_length = reader.read8()?;
                        remaining -= 2;

                        if option_length > 2 && remaining >= (option_length as usize - 2) {
                            let encryption_type = match option_type {
                                1 => "OUI",
                                2 => "DESE-bis",
                                3 => "3DES",
                                4 => "AES-CBC",
                                _ => "Unknown",
                            };
                            options_data.push(format!("Encryption Method: {encryption_type} ({option_type})"));
                            reader.forward(option_length as usize - 2);
                            remaining -= option_length as usize - 2;
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {option_type}, Length {option_length}"));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        add_field_backstep!(field, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    add_field_rest_format!(field, reader, format!("ECP Data: {} bytes", reader.left()));
                }
            } else {
                add_field_rest_format!(field, reader, "Incomplete ECP Header".to_string());
            }
        }
        _ => {
            add_field_rest_format!(field, reader, format!("Payload: {} bytes", reader.left()));
        }
    }
    field.summary = format!("PPP Protocol: {protocol_name} ({protocol:#06x})");
    Ok(protocol)
}
pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        let msg = match frame.protocol_field {
            ProtocolInfoField::PPPoES(Some(code)) => ppp_lcp_type_mapper(code).to_string(),
            _ => SUMMARY.to_string(),
        };
        Some(msg)
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let _version_type = reader.read8()?;
        let _code = reader.read8()?;
        let _session_id = reader.read16(true)?;
        let _payload_length = reader.read16(true)?; // Use underscore to indicate unused variable

        // Read PPP protocol type
        let protocol = reader.read16(true)?;
        match protocol {
            0xc021 | 0x8021 | 0xc023 | 0x8057 => {
                let code = reader.read8()?;
                frame.protocol_field = ProtocolInfoField::PPPoES(Some(code));
            }
            _ => {
                frame.protocol_field = ProtocolInfoField::PPPoES(None);
            }
        }

        frame.add_proto(crate::common::ProtoMask::PPPOES);
        // Determine next layer protocol based on PPP protocol type
        match protocol {
            0x0021 => Ok(Protocol::IP4), // IPv4
            0x0057 => Ok(Protocol::IP6), // IPv6
            _ => Ok(Protocol::None),
        }
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let version_type = reader.read8()?;
        let version = (version_type >> 4) & 0x0F;
        let type_val = version_type & 0x0F;
        add_field_backstep!(field, reader, 1, format!("Version: {}, Type: {}", version, type_val));

        add_field_format!(field, reader, reader.read8()?, "Code: {}");

        add_field_format!(field, reader, reader.read16(true)?, "Session ID: {:#06x}");

        add_field_format!(field, reader, reader.read16(true)?, "Payload Length: {} bytes");

        let protocol = add_sub_field_with_reader!(field, reader, read_payload)?;
        // let protocol = reader.read16(true)?;
        // let protocol_name = ppp_type_mapper(protocol);
        // let _index = add_field_backstep!(field, reader, 2, format!("PPP Protocol: {} ({:#06x})", protocol_name, protocol));

        // let f = list.get_mut(_index).unwrap();

        // f.children = Some(read_payload(protocol, reader)?);

        field.summary = SUMMARY.to_string();
        match protocol {
            0x0021 => Ok(Protocol::IP4), // IPv4
            0x0057 => Ok(Protocol::IP6), // IPv6
            _ => Ok(Protocol::None),
        }
    }
}
