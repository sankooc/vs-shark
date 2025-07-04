// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::{concept::Field, io::Reader};
use crate::constants::{tls_cipher_suites_mapper, tls_hs_message_type_mapper};
use crate::protocol::transport::tls::decode::parse_cert;
use crate::protocol::transport::tls::{extension, field_tls_version};
use crate::{add_field_backstep, add_field_format, add_field_format_fn, add_field_format_fn_nors, add_sub_field_with_reader};
use anyhow::{Result};

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


// pub const BER_SEQUENCE: u8 = 0x30;
// pub const BER_SEQUENCE_OF: u8 = 0x0a;
// pub const BER_SET: u8 = 0x31;
// pub const BER_SET_OF: u8 = 0x0b;
// pub const BER_INT: u8 = 0x02;
// pub const BER_BIT_STRING: u8 = 0x03;
// pub const BER_OCTET_STRING: u8 = 0x04;
// pub const BER_NULL: u8 = 0x05;
// pub const BER_OBJECT_IDENTIFIER: u8 = 0x06;
// pub const BER_UTF_STR: u8 = 0x0c;
// pub const BER_PRINTABLE_STR: u8 = 0x13;
// pub const BER_IA5STRING: u8 = 0x16;
// pub const BER_UTC_TIME: u8 = 0x17;
// pub const BER_GENERALIZED_TIME: u8 = 0x18;

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
    for (_, item) in data.iter().enumerate().take(len) {
        rs.push_str(&format!("{:02x}", *item));
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


pub fn read24(reader: &mut Reader) -> Result<u32>{
    if reader.left() >= 3 {
        let b1 = reader.read8()?;
        let b2 = reader.read8()?;
        let b3 = reader.read8()?;
        let length = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
        return Ok(length);
    }
    Ok(0)
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
        let length = read24(reader)?;
        add_field_backstep!(field, reader, 3, format!("Length: {}", length));

        // Parse specific handshake message based on type
        if reader.left() >= length as usize {
            let mut _reader = reader.slice_as_reader(length as usize)?;
            match msg_type {
                CLIENT_HELLO => parse_client_hello(&mut _reader, field)?,
                SERVER_HELLO => parse_server_hello(&mut _reader, field)?,
                CERTIFICATE => parse_certificates(&mut _reader, field)?,
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
        add_sub_field_with_reader!(field, reader, move |reader, field| field_ciper_suite_list(cipher_suites_len, reader, field))?;
    }

    // Parse compression methods
    if reader.left() >= 1 {
        let clen = add_field_format!(field, reader, reader.read8()?, "Compression Methods Length: {}");
        add_sub_field_with_reader!(field, reader, move |reader, field| field_compression_list(clen, reader, field))?;
    }

    if reader.left() >= 2 {
        extension::parse_detail(reader, field, true)?;
    }
    Ok(())
}

/// Parse ServerHello message
pub fn parse_server_hello(reader: &mut Reader, field: &mut Field) -> Result<()> {
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
pub fn _read_len(reader: &mut Reader) -> Result<usize> {
    let _next = reader.read8()?;
    let _len = match _next {
        0x81 => reader.read8()? as usize,
        0x82 => reader.read16(true)? as usize,
        0x83 => {
            read24(reader)? as usize
        }
        0x84 => reader.read32(true)? as usize,
        _ => _next as usize,
    };
    Ok(_len)
}
pub fn parse_certificate(_reader: &mut Reader, _field: &mut Field) -> Result<()> {
    // let _type = reader.read8()?;
    // let _len = _read_len(reader)?;
    // if _type == BER_SEQUENCE {
    // }
    // match _type {
    //     BER_SEQUENCE | BER_SEQUENCE_OF => {
            
    //     }
    //     _ => {}
    // }
    Ok(())
}
/// Parse Certificate message
pub fn parse_certificates(reader: &mut Reader, field: &mut Field) -> Result<()> {

    let total_length = read24(reader)?;
    add_field_format!(field, reader, total_length, "Certificates Length: {}");

    // Parse individual certificates
    let mut cert_count = 0;

    loop {
        if reader.left() < 3 {
            break;
        }
        let bytes_read = read24(reader)? as usize;
        if reader.left() >= bytes_read {
            let mut _reader = reader.slice_as_reader(bytes_read)?;
            // add_sub_field_with_reader!(field, &mut _reader, |reader, field| parse_sequence(decode::Certificate, reader, field)).unwrap();

            if let Ok(_) = add_sub_field_with_reader!(field, &mut _reader, parse_cert) {

            }
            cert_count += 1;
        } else {
            break;
        }
    }

    add_field_format!(field, reader, cert_count, "Certificate Count: {}");
    field.summary = format!("Certificate: {}", cert_count);
    Ok(())
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
