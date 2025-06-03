use crate::common::{concept::Field, io::Reader};
use crate::constants::{tls_cipher_suites_mapper, tls_hs_message_type_mapper};
use crate::protocol::transport::tls::{extension, field_tls_version};
use crate::{add_field_backstep, add_field_format, add_field_format_fn, add_field_format_fn_nors, add_sub_field_with_reader};
use anyhow::Result;

// TLS Record Content Types
const CHANGE_CIPHER_SPEC: u8 = 20;
// const ALERT: u8 = 21;
const HANDSHAKE: u8 = 22;
// const APPLICATION_DATA: u8 = 23;
const HEARTBEAT: u8 = 24;

// TLS Handshake Types
const HELLO_REQUEST: u8 = 0;
const CLIENT_HELLO: u8 = 1;
const SERVER_HELLO: u8 = 2;
const CERTIFICATE: u8 = 11;
const SERVER_KEY_EXCHANGE: u8 = 12;
const CERTIFICATE_REQUEST: u8 = 13;
const SERVER_HELLO_DONE: u8 = 14;
const CERTIFICATE_VERIFY: u8 = 15;
const CLIENT_KEY_EXCHANGE: u8 = 16;
const FINISHED: u8 = 20;

fn field_random_str(data: &[u8]) -> String {
    let mut rs = String::with_capacity(72);
    rs.push_str("Random: ");
    let len = std::cmp::min(32, data.len());
    for i in 0..len {
        rs.push_str(&format!("{:02x}", data[i]));
    }
    rs
}
fn field_session_id_str(data: &[u8]) -> String {
    let len = std::cmp::min(32, data.len());
    let mut rs = String::with_capacity(12 + len * 2);
    rs.push_str("Session ID: ");
    for i in 0..len {
        rs.push_str(&format!("{:02x}", data[i]));
    }
    rs
}

fn field_ciper_suite_str(code: u16) -> String {
    format!("Cipher Suite: Reserved ({}) ({:06x})", tls_cipher_suites_mapper(code), code)
}

fn field_compress_str(code: u8) -> String {
    let method = match code {
        0 => "null (No Compression)",
        1 => "DEFLATE",
        2..=63 => "IETF Standards Track Protocol",
        64..=223 => "Non-Standards Track Method",
        224..=255 => "Private Use",
    };
    format!("Compression Method: {} ({})", method, code)
}

fn field_ciper_suite_list(len: u16, reader: &mut Reader, field: &mut Field) -> Result<()> {
    if reader.left() < len as usize {
        return Ok(());
    }
    let count = len / 2;
    let finish = reader.cursor + len as usize;
    field.summary = format!("Cipher Suites Count: {}", count);
    for _ in 0..count {
        add_field_format_fn!(field, reader, reader.read16(true).unwrap(), field_ciper_suite_str);
    }

    reader.cursor = finish;
    Ok(())
}
fn field_compression_list(len: u8, reader: &mut Reader, field: &mut Field) -> Result<()> {
    if reader.left() < len as usize {
        return Ok(());
    }
    let finish = reader.cursor + len as usize;
    field.summary = format!("Compression Methods: {}", len);
    for _ in 0..len {
        add_field_format_fn!(field, reader, reader.read8().unwrap(), field_compress_str);
    }

    reader.cursor = finish;
    Ok(())
}

/// Parse TLS record based on content type
pub fn parse_record_detail(content_type: u8, _version: u16, reader: &mut Reader, field: &mut Field) -> Result<()> {
    match content_type {
        CHANGE_CIPHER_SPEC => parse_change_cipher_spec(reader, field),
        HANDSHAKE => parse_handshake(reader, field)?,
        HEARTBEAT => parse_heartbeat(reader, field),
        _ => {
            field.summary = "Application Data".to_string();
        }
    }
    Ok(())
}

fn parse_change_cipher_spec(_reader: &mut Reader, field: &mut Field) {
    field.summary = "Change Cipher Spec".to_string();
}

/// Parse Handshake record
fn parse_handshake(reader: &mut Reader, field: &mut Field) -> Result<()> {
    field.summary = "Handshake".to_string();
    //tls_hs_message_type_mapper
    if reader.left() >= 4 {
        // Handshake has a 4-byte header: type(1) + length(3)
        let msg_type = add_field_format_fn!(field, reader, reader.read8()?, handshake_type);
        field.summary = format!("Handshake: {}", tls_hs_message_type_mapper(msg_type));
        // Read length (3 bytes)
        let mut length: u32 = 0;
        if let Ok(b1) = reader.read8() {
            if let Ok(b2) = reader.read8() {
                if let Ok(b3) = reader.read8() {
                    length = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
                    add_field_backstep!(field, reader, 3, format!("Length: {}", length));
                }
            }
        }

        // Parse specific handshake message based on type
        if reader.left() >= length as usize {
            let mut _reader = reader.slice_as_reader(length as usize)?;
            match msg_type {
                CLIENT_HELLO => parse_client_hello(&mut _reader, field)?,
                SERVER_HELLO => parse_server_hello(&mut _reader, field)?,
                // CERTIFICATE => parse_certificate(&mut _reader, field),
                // Other handshake types could be implemented here
                _ => {
                    // Skip the content as we don't parse it in detail
                }
            }
        }
    }
    Ok(())
}

/// Parse Heartbeat record
fn parse_heartbeat(reader: &mut Reader, field: &mut Field) {
    field.summary = "Heartbeat".to_string();

    if reader.left() >= 3 {
        // Heartbeat has type(1) + payload_length(2)
        if let Ok(hb_type) = reader.read8() {
            let type_str = match hb_type {
                1 => "Request",
                2 => "Response",
                _ => "Unknown",
            };
            add_field_format!(field, reader, type_str, "Type: {}");

            // Read payload length
            if let Ok(length) = reader.read16(true) {
                add_field_format!(field, reader, length, "Payload Length: {}");

                // Skip the payload and padding
                if reader.left() >= length as usize {
                    reader.forward(length as usize);
                }

                field.summary = format!("Heartbeat: {}", type_str);
            }
        }
    }
}

/// Parse ClientHello message
fn parse_client_hello(reader: &mut Reader, field: &mut Field) -> Result<()> {
    add_field_format_fn!(field, reader, reader.read16(true)?, field_tls_version);

    if reader.left() >= 32 {
        add_field_format_fn_nors!(field, reader, reader.slice(32, true)?, field_random_str);
    }

    // Parse session ID
    if reader.left() >= 1 {
        let session_id_len = add_field_format!(field, reader, reader.read8()?, "Session ID Length: {}");
        add_field_format_fn_nors!(field, reader, reader.slice(session_id_len as usize, true)?, field_session_id_str);
    }

    // Parse cipher suites
    if reader.left() >= 2 {
        let cipher_suites_len = add_field_format!(field, reader, reader.read16(true)?, "Cipher Suites Length: {}");
        add_sub_field_with_reader!(field, reader, move |reader, field| field_ciper_suite_list(cipher_suites_len, reader, field));
    }

    // Parse compression methods
    if reader.left() >= 1 {
        let clen = add_field_format!(field, reader, reader.read8()?, "Compression Methods Length: {}");
        add_sub_field_with_reader!(field, reader, move |reader, field| field_compression_list(clen, reader, field));
    }

    if reader.left() >= 2 {
        extension::parse_detail(reader, field, true)?;
    }
    Ok(())
}

/// Parse ServerHello message
fn parse_server_hello(reader: &mut Reader, field: &mut Field) -> Result<()> {

    // Parse server version

    add_field_format_fn!(field, reader, reader.read16(true)?, field_tls_version);

    // Parse server random (32 bytes)
    if reader.left() >= 32 {
        add_field_format_fn_nors!(field, reader, reader.slice(32, true)?, field_random_str);
    }

    // Parse session ID
    if reader.left() >= 1 {
        let session_id_len = add_field_format!(field, reader, reader.read8()?, "Session ID Length: {}");
        add_field_format_fn_nors!(field, reader, reader.slice(session_id_len as usize, true)?, field_session_id_str);
    }

    // // Parse cipher suites
    // if reader.left() >= 2 {
    //     let cipher_suites_len = add_field_format!(field, reader, reader.read16(true)?, "Cipher Suites Length: {}");
    //     add_sub_field_with_reader!(field, reader, move |reader, field| field_ciper_suite_list(cipher_suites_len, reader, field));
    // }
    
    add_field_format_fn!(field, reader, reader.read16(true)?, field_ciper_suite_str);
    add_field_format_fn!(field, reader, reader.read8()?, field_compress_str);

    if reader.left() >= 2 {
        extension::parse_detail(reader, field, false)?;
    }

    Ok(())
}

/// Parse Certificate message
fn _parse_certificate(reader: &mut Reader, field: &mut Field) {
    let start_pos = reader.cursor;

    // Create a child field for Certificate details
    let mut cert_field = Field::with_children("Certificate".to_string(), start_pos, 0);

    // Parse certificates length (3 bytes)
    let mut total_length: u32 = 0;
    if reader.left() >= 3 {
        if let Ok(b1) = reader.read8() {
            if let Ok(b2) = reader.read8() {
                if let Ok(b3) = reader.read8() {
                    total_length = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
                    add_field_format!(cert_field, reader, total_length, "Certificates Length: {}");
                }
            }
        }
    }

    // Parse individual certificates
    let mut cert_count = 0;
    let mut bytes_read = 0;

    while bytes_read < total_length && reader.left() >= 3 {
        // Each certificate has a 3-byte length prefix
        let mut _cert_length: u32 = 0;
        if let Ok(b1) = reader.read8() {
            if let Ok(b2) = reader.read8() {
                if let Ok(b3) = reader.read8() {
                    _cert_length = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
                    bytes_read += 3 + _cert_length;

                    // Skip certificate data
                    if reader.left() >= _cert_length as usize {
                        reader.forward(_cert_length as usize);
                        cert_count += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    add_field_format!(cert_field, reader, cert_count, "Certificate Count: {}");
    cert_field.size = (reader.cursor - start_pos) as usize;

    // Add the certificate field to the parent field
    if let Some(children) = &mut field.children {
        children.push(cert_field);
    }
}


/// Get string representation of handshake message type
fn handshake_type(msg_type: u8) -> String {
    match msg_type {
        HELLO_REQUEST => "Hello Request (0)".to_string(),
        CLIENT_HELLO => "Client Hello (1)".to_string(),
        SERVER_HELLO => "Server Hello (2)".to_string(),
        CERTIFICATE => "Certificate (11)".to_string(),
        SERVER_KEY_EXCHANGE => "Server Key Exchange (12)".to_string(),
        CERTIFICATE_REQUEST => "Certificate Request (13)".to_string(),
        SERVER_HELLO_DONE => "Server Hello Done (14)".to_string(),
        CERTIFICATE_VERIFY => "Certificate Verify (15)".to_string(),
        CLIENT_KEY_EXCHANGE => "Client Key Exchange (16)".to_string(),
        FINISHED => "Finished (20)".to_string(),
        _ => format!("Unknown ({}) ", msg_type),
    }
}
