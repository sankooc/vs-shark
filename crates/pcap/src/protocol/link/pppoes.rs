use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{ProtocolInfoField, Protocol},
        io::Reader,
        Frame,
    },
    constants::{ppp_lcp_type_mapper, ppp_type_mapper},
    field_back_format, field_rest_format, read_field_format,
};
use anyhow::Result;

const SUMMARY: &'static str = "PPP-over-Ethernet Session";

fn payload(protocol: u16, reader: &mut Reader) -> Result<Vec<Field>> {
    let mut list = vec![];
    match protocol {
        0xc021 => {
            if reader.left() >= 4 {
                let code = reader.read8()?;
                field_back_format!(list, reader, 1, format!("LCP Code: {} ({})", ppp_lcp_type_mapper(code), code));
                read_field_format!(list, reader, reader.read8()?, "Identifier: {}");
                read_field_format!(list, reader, reader.read16(true)?, "Length: {} bytes");
                field_rest_format!(list, reader, format!("LCP Data: {} bytes", reader.left()));
            } else {
                reader.forward(reader.left());
                field_rest_format!(list, reader, "Incomplete LCP Header".to_string());
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
                field_back_format!(list, reader, 1, format!("PAP Code: {} ({})", pap_code, code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

                field_rest_format!(list, reader, format!("PAP Data: {} bytes", reader.left()));
            } else {
                field_rest_format!(list, reader, "Incomplete PAP Header".to_string());
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
                field_back_format!(list, reader, 1, format!("CHAP Code: {} ({})", chap_code, code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

                field_rest_format!(list, reader, format!("CHAP Data: {} bytes", reader.left()));
            } else {
                field_rest_format!(list, reader, "Incomplete CHAP Header".to_string());
            }
        }
        0x8021 => {
            // IPCP (IP Control Protocol)
            if reader.left() >= 4 {
                let code = reader.read8()?;
                field_back_format!(list, reader, 1, format!("IPCP Code: {} ({})", ppp_lcp_type_mapper(code), code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

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
                                        options_data.push(format!("IP-Address: {}", ip));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("IP-Address (Invalid Length: {})", option_length));
                                    }
                                }
                                2 => {
                                    // IP-Compression-Protocol
                                    if option_length >= 4 {
                                        let protocol = reader.read16(true)?;
                                        options_data.push(format!("IP-Compression-Protocol: {:#06x}", protocol));
                                        reader.forward(option_length as usize - 4);
                                        remaining -= option_length as usize - 2;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("IP-Compression-Protocol (Invalid Length: {})", option_length));
                                    }
                                }
                                3 => {
                                    // IP-Address (Primary DNS)
                                    if option_length == 6 {
                                        let ip = reader.read_ip4()?;
                                        options_data.push(format!("Primary DNS: {}", ip));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Primary DNS (Invalid Length: {})", option_length));
                                    }
                                }
                                129 => {
                                    // IP-Address (Secondary DNS)
                                    if option_length == 6 {
                                        let ip = reader.read_ip4()?;
                                        options_data.push(format!("Secondary DNS: {}", ip));
                                        remaining -= 4;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Secondary DNS (Invalid Length: {})", option_length));
                                    }
                                }
                                _ => {
                                    reader.forward(option_length as usize - 2);
                                    remaining -= option_length as usize - 2;
                                    options_data.push(format!("Option Type: {}, Length: {}", option_type, option_length));
                                }
                            }
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {}, Length {}", option_type, option_length));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        field_back_format!(list, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    field_rest_format!(list, reader, format!("IPCP Data: {} bytes", reader.left()));
                }
            } else {
                field_rest_format!(list, reader, "Incomplete IPCP Header".to_string());
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
                field_back_format!(list, reader, 1, format!("IPv6CP Code: {} ({})", ipv6cp_code, code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

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
                                        options_data.push(format!("Interface-Identifier: {:#010x}{:08x}", id_high, id_low));
                                        remaining -= 8;
                                    } else {
                                        reader.forward(option_length as usize - 2);
                                        remaining -= option_length as usize - 2;
                                        options_data.push(format!("Interface-Identifier (Invalid Length: {})", option_length));
                                    }
                                }
                                _ => {
                                    reader.forward(option_length as usize - 2);
                                    remaining -= option_length as usize - 2;
                                    options_data.push(format!("Option Type: {}, Length: {}", option_type, option_length));
                                }
                            }
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {}, Length {}", option_type, option_length));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        field_back_format!(list, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    field_rest_format!(list, reader, format!("IPv6CP Data: {} bytes", reader.left()));
                }
            } else {
                field_rest_format!(list, reader, "Incomplete IPv6CP Header".to_string());
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
                field_back_format!(list, reader, 1, format!("CCP Code: {} ({})", ccp_code, code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

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
                            options_data.push(format!("Compression Method: {} ({})", compression_type, option_type));
                            reader.forward(option_length as usize - 2);
                            remaining -= option_length as usize - 2;
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {}, Length {}", option_type, option_length));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        field_back_format!(list, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    field_rest_format!(list, reader, format!("CCP Data: {} bytes", reader.left()));
                }
            } else {
                field_rest_format!(list, reader, "Incomplete CCP Header".to_string());
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
                field_back_format!(list, reader, 1, format!("ECP Code: {} ({})", ecp_code, code));

                let identifier = reader.read8()?;
                field_back_format!(list, reader, 1, format!("Identifier: {}", identifier));

                let length = reader.read16(true)?;
                field_back_format!(list, reader, 2, format!("Length: {} bytes", length));

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
                            options_data.push(format!("Encryption Method: {} ({})", encryption_type, option_type));
                            reader.forward(option_length as usize - 2);
                            remaining -= option_length as usize - 2;
                        } else {
                            // Invalid option length
                            reader.forward(remaining);
                            options_data.push(format!("Invalid Option: Type {}, Length {}", option_type, option_length));
                            break;
                        }
                    }

                    if !options_data.is_empty() {
                        field_back_format!(list, reader, 0, format!("Options: {}", options_data.join(", ")));
                    }
                } else {
                    field_rest_format!(list, reader, format!("ECP Data: {} bytes", reader.left()));
                }
            } else {
                field_rest_format!(list, reader, "Incomplete ECP Header".to_string());
            }
        }
        _ => {
            field_rest_format!(list, reader, format!("Payload: {} bytes", reader.left()));
        }
    }
    Ok(list)
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

        // Determine next layer protocol based on PPP protocol type
        match protocol {
            0x0021 => Ok(Protocol::IP4), // IPv4
            0x0057 => Ok(Protocol::IP6), // IPv6
            _ => Ok(Protocol::None),
        }
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];

        let version_type = reader.read8()?;
        let version = (version_type >> 4) & 0x0F;
        let type_val = version_type & 0x0F;
        field_back_format!(list, reader, 1, format!("Version: {}, Type: {}", version, type_val));

        read_field_format!(list, reader, reader.read8()?, "Code: {}");

        read_field_format!(list, reader, reader.read16(true)?, "Session ID: {:#06x}");

        read_field_format!(list, reader, reader.read16(true)?, "Payload Length: {} bytes");

        let protocol = reader.read16(true)?;
        let protocol_name = ppp_type_mapper(protocol);
        let _index = field_back_format!(list, reader, 2, format!("PPP Protocol: {} ({:#06x})", protocol_name, protocol));

        let f = list.get_mut(_index).unwrap();

        f.children = Some(payload(protocol, reader)?);

        field.summary = SUMMARY.to_string();
        field.children = Some(list);

        match protocol {
            0x0021 => Ok(Protocol::IP4), // IPv4
            0x0057 => Ok(Protocol::IP6), // IPv6
            _ => Ok(Protocol::None),
        }
    }
}
