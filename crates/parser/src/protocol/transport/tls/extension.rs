// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::{concept::Field, io::Reader};
use crate::constants::tls_extension_mapper;
use crate::protocol::transport::tls::field_tls_version;
use crate::{add_field_backstep, add_field_format, add_field_format_fn, add_sub_field_with_reader};
use anyhow::{Ok, Result};

// TLS Extension Types as defined in RFC 6066, 7301, 7250, 7685, 8446, etc.
// Reference: https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml
const EXT_SERVER_NAME: u16 = 0; // RFC 6066
// const EXT_MAX_FRAGMENT_LENGTH: u16 = 1; // RFC 6066
// const EXT_CLIENT_CERTIFICATE_URL: u16 = 2; // RFC 6066
// const EXT_TRUSTED_CA_KEYS: u16 = 3; // RFC 6066
// const EXT_TRUNCATED_HMAC: u16 = 4; // RFC 6066
// const EXT_STATUS_REQUEST: u16 = 5; // RFC 6066
// const EXT_USER_MAPPING: u16 = 6; // RFC 4681
// const EXT_CLIENT_AUTHZ: u16 = 7; // RFC 5878
// const EXT_SERVER_AUTHZ: u16 = 8; // RFC 5878
// const EXT_CERT_TYPE: u16 = 9; // RFC 6091
const EXT_SUPPORTED_GROUPS: u16 = 10; // RFC 8446, 7919
const EXT_EC_POINT_FORMATS: u16 = 11; // RFC 8422
// const EXT_SRP: u16 = 12; // RFC 5054
const EXT_SIGNATURE_ALGORITHMS: u16 = 13; // RFC 8446
// const EXT_USE_SRTP: u16 = 14; // RFC 5764
// const EXT_HEARTBEAT: u16 = 15; // RFC 6520
const EXT_APPLICATION_LAYER_PROTOCOL_NEGOTIATION: u16 = 16; // RFC 7301
// const EXT_STATUS_REQUEST_V2: u16 = 17; // RFC 6961
// const EXT_SIGNED_CERTIFICATE_TIMESTAMP: u16 = 18; // RFC 6962
// const EXT_CLIENT_CERTIFICATE_TYPE: u16 = 19; // RFC 7250
// const EXT_SERVER_CERTIFICATE_TYPE: u16 = 20; // RFC 7250
// const EXT_PADDING: u16 = 21; // RFC 7685
// const EXT_ENCRYPT_THEN_MAC: u16 = 22; // RFC 7366
// const EXT_EXTENDED_MASTER_SECRET: u16 = 23; // RFC 7627
// const EXT_TOKEN_BINDING: u16 = 24; // RFC 8472
// const EXT_CACHED_INFO: u16 = 25; // RFC 7924
const EXT_KEY_SHARE: u16 = 51; // RFC 8446
// const EXT_PRE_SHARED_KEY: u16 = 41; // RFC 8446
// const EXT_EARLY_DATA: u16 = 42; // RFC 8446
const EXT_SUPPORTED_VERSIONS: u16 = 43; // RFC 8446
// const EXT_COOKIE: u16 = 44; // RFC 8446
// const EXT_PSK_KEY_EXCHANGE_MODES: u16 = 45; // RFC 8446
// const EXT_CERTIFICATE_AUTHORITIES: u16 = 47; // RFC 8446
// const EXT_OID_FILTERS: u16 = 48; // RFC 8446
// const EXT_POST_HANDSHAKE_AUTH: u16 = 49; // RFC 8446
// const EXT_SIGNATURE_ALGORITHMS_CERT: u16 = 50; // RFC 8446
// const EXT_RENEGOTIATION_INFO: u16 = 65281; // RFC 5746

fn field_extension_type(code: u16) -> String {
    format!("Type: {} ({:#06x})", tls_extension_mapper(code), code)
}
// Parse TLS extensions
// Reference: RFC 8446 Section 4.2
pub fn parse_detail(reader: &mut Reader, field: &mut Field, is_client: bool) -> Result<()> {
    if reader.left() < 2 {
        return Ok(());
    }

    // Read extensions length
    let extensions_len = add_field_format!(field, reader, reader.read16(true)?, "Extensions Length: {}");

    if reader.left() < extensions_len as usize {
        return Ok(());
    }

    let mut _reader = reader.slice_as_reader(extensions_len as usize)?;

    add_sub_field_with_reader!(field, &mut _reader, parse_extensions(is_client))?;

    Ok(())
}
fn parse_extensions(is_client: bool) -> Box<dyn Fn(&mut Reader, &mut Field) -> Result<()>> {
    Box::new(move |reader: &mut Reader, field: &mut Field| -> Result<()> {
        field.summary = "Extensions".into();
        while reader.left() >= 4 {
            add_sub_field_with_reader!(field, reader,|reader: &mut Reader, field: &mut Field| parse_extension(reader, field, is_client))?;
        }
        Ok(())
    })
}

fn parse_extension(reader: &mut Reader, ext_field: &mut Field, is_client: bool) -> Result<()> {
    let ext_type = add_field_format_fn!(ext_field, reader, reader.read16(true)?, field_extension_type);
    // Read extension length
    let ext_len = add_field_format!(ext_field, reader, reader.read16(true)?, "Length: {}");
    ext_field.summary = format!("Extension: {}", field_extension_type(ext_type));

    if ext_len == 0 {
        return Ok(());
    }
    let mut _reader = reader.slice_as_reader(ext_len as usize)?;

    let parser = match ext_type {
        EXT_SERVER_NAME => parse_server_name,
        EXT_SUPPORTED_GROUPS => if is_client { parse_supported_groups } else { parse_supported_groups_item },
        EXT_EC_POINT_FORMATS => if is_client { parse_ec_point_formats } else { parse_ec_point_formats_item },
        EXT_SIGNATURE_ALGORITHMS => if is_client { parse_signature_algorithms } else { parse_signature_algorithms_item },
        EXT_APPLICATION_LAYER_PROTOCOL_NEGOTIATION => if is_client { parse_alpn } else { parse_alpn_item },
        EXT_SUPPORTED_VERSIONS => if is_client { parse_supported_versions } else { parse_supported_versions_item },
        EXT_KEY_SHARE => if is_client { parse_key_share } else { parse_key_share_item },
        _ => {
            // Skip unknown extension data
            if ext_len > 0 {
                add_field_backstep!(ext_field, reader, ext_len as usize, "Extension Data".into());
            }
            return Ok(());
        }
    };
    add_sub_field_with_reader!(ext_field, &mut _reader, parser)?;

    Ok(())
}

// Parse Server Name Indication (SNI) extension
// Reference: RFC 6066 Section 3
fn parse_server_name(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left();
    if ext_len < 2 {
        return Ok(());
    }
    field.summary = "Server Name Indication extension".into();
    // Read server name list length
    let list_len = add_field_format!(field, reader, reader.read16(true)?, "Server Name List Length: {}");

    if list_len == 0 {
        return Ok(());
    }
    //  RFC 6066 "The server_name_list MUST contain exactly one host_name."
    add_field_format!(field, reader, reader.read8()?, "Server Name Type: host_name {}");

    let _len = add_field_format!(field, reader, reader.read16(true)?, "Server Name Length: {}");
    add_field_format!(field, reader, reader.read_string(_len as usize)?, "Server Name: {}");
    Ok(())
}

// Parse Supported Groups extension (previously known as Elliptic Curves)
// Reference: RFC 8446 Section 4.2.7, RFC 7919

fn parse_supported_groups_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    add_field_format_fn!(field, reader, reader.read16(true)?, field_group_support);
    field.summary = "Supported Group".into();
    Ok(())
}
fn parse_supported_groups(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left() as u16;
    if ext_len < 2 {
        return Ok(());
    }
    let list_len = reader.read16(true)?;
    if list_len == 0 || list_len > ext_len - 2 {
        return Ok(());
    }
    let count = list_len / 2;
    for _ in 0..count {
        parse_supported_groups_item(reader, field)?;
    }
    field.summary = format!("Supported Groups List Length: {}", list_len);

    Ok(())
}

fn field_group_support(group_id: u16) -> String {
    format!("Supported Group: {} ({:#06x})", named_group_to_string(group_id), group_id)
}
fn field_group_ks(group_id: u16) -> String {
    format!("Group: {} ({:#06x})", named_group_to_string(group_id), group_id)
}

fn field_ec_point_format(format: u8) -> String {
    match format {
        0 => "EC Point Format: uncompressed".to_string(),
        1 => "EC Point Format: ansiX962_compressed_prime".to_string(),
        2 => "EC Point Format: ansiX962_compressed_char2".to_string(),
        _ => "EC Point Format: unknown".to_string(),
    }
}
// Convert named group ID to string
// Reference: RFC 8446 Section 4.2.7, RFC 7919
fn named_group_to_string(group_id: u16) -> String {
    match group_id {
        // Elliptic Curve Groups (ECDHE)
        0x0001 => "sect163k1".to_string(),
        0x0002 => "sect163r1".to_string(),
        0x0003 => "sect163r2".to_string(),
        0x0004 => "sect193r1".to_string(),
        0x0005 => "sect193r2".to_string(),
        0x0006 => "sect233k1".to_string(),
        0x0007 => "sect233r1".to_string(),
        0x0008 => "sect239k1".to_string(),
        0x0009 => "sect283k1".to_string(),
        0x000A => "sect283r1".to_string(),
        0x000B => "sect409k1".to_string(),
        0x000C => "sect409r1".to_string(),
        0x000D => "sect571k1".to_string(),
        0x000E => "sect571r1".to_string(),
        0x000F => "secp160k1".to_string(),
        0x0010 => "secp160r1".to_string(),
        0x0011 => "secp160r2".to_string(),
        0x0012 => "secp192k1".to_string(),
        0x0013 => "secp192r1".to_string(),
        0x0014 => "secp224k1".to_string(),
        0x0015 => "secp224r1".to_string(),
        0x0016 => "secp256k1".to_string(),
        0x0017 => "secp256r1".to_string(),
        0x0018 => "secp384r1".to_string(),
        0x0019 => "secp521r1".to_string(),
        0x001D => "x25519".to_string(),
        0x001E => "x448".to_string(),
        // Finite Field Groups (DHE)
        0x0100 => "ffdhe2048".to_string(),
        0x0101 => "ffdhe3072".to_string(),
        0x0102 => "ffdhe4096".to_string(),
        0x0103 => "ffdhe6144".to_string(),
        0x0104 => "ffdhe8192".to_string(),
        _ => format!("unknown_group_{}", group_id),
    }
}

// Parse EC Point Formats extension
// Reference: RFC 8422 Section 5.1.2

fn parse_ec_point_formats_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    add_field_format_fn!(field, reader, reader.read8()?, field_ec_point_format);
    field.summary = "Elliptic curves point format".into();
    Ok(())
}
fn parse_ec_point_formats(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left() as u16;
    if ext_len < 1 {
        return Ok(());
    }
    // Read EC point formats length
    let formats_len = add_field_format!(field, reader, reader.read8()?, "EC Point Formats Length: {}");

    if formats_len == 0 {
        return Ok(());
    }
    for _ in 0..formats_len {
        parse_ec_point_formats_item(reader, field)?;
    }
    field.summary = format!("Elliptic curves point formats ({})", formats_len);
    Ok(())
}

// Parse Signature Algorithms extension
// Reference: RFC 8446 Section 4.2.3

fn parse_signature_algorithms_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let sig_alg = reader.read16(true)?;
    let sig_hash = (sig_alg >> 8) as u8;
    let sig_sign = (sig_alg & 0xFF) as u8;

    let hash_name = match sig_hash {
        0 => "none",
        1 => "md5",
        2 => "sha1",
        3 => "sha224",
        4 => "sha256",
        5 => "sha384",
        6 => "sha512",
        8 => "intrinsic",
        _ => "unknown",
    };

    let sign_name = match sig_sign {
        0 => "anonymous",
        1 => "rsa",
        2 => "dsa",
        3 => "ecdsa",
        7 => "ed25519",
        8 => "ed448",
        9 => "rsa_pss_rsae_sha256",
        10 => "rsa_pss_rsae_sha384",
        11 => "rsa_pss_rsae_sha512",
        _ => "unknown",
    };

    let alg_str = format!("Algorithm: {}_{}(0x{:04x})", hash_name, sign_name, sig_alg);
    add_field_backstep!(field, reader, 2, alg_str);
    Ok(())
}
fn parse_signature_algorithms(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left() as u16;
    if ext_len < 2 {
        return Ok(());
    }

    // Read signature algorithms length
    let list_len = add_field_format!(field, reader, reader.read16(true)?, "Signature Algorithms Length: {}");

    if list_len == 0 || list_len > ext_len - 2 {
        return Ok(());
    }

    // Create a field for the list
    let mut list_field = Field::with_children("Signature Algorithms".to_string(), reader.cursor, list_len as usize);

    // Parse each algorithm
    let count = list_len / 2;
    for _ in 0..count {
        parse_signature_algorithms_item(reader, &mut list_field)?;
    }

    field.summary = format!("Signature Hash Algorithms ({} algorithms)", count);
    // Add list field to parent
    if let Some(children) = &mut field.children {
        children.push(list_field);
    }

    Ok(())
}

// Parse Application Layer Protocol Negotiation (ALPN) extension
// Reference: RFC 7301
// 

fn parse_alpn_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let len = add_field_format!(field, reader, reader.read8()?, "ALPN Protocol Length: {}") as usize;
    let protocol = add_field_format!(field, reader, reader.read_string(len)?, "ALPN Next Protocol: {}");
    field.summary = format!("ALPN Protocol: {}", protocol);
    Ok(())
}
fn parse_alpn(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left() as u16;
    if ext_len < 2 {
        return Ok(());
    }


    // Read ALPN list length
    let list_len = add_field_format!(field, reader, reader.read16(true)?, "ALPN List Length: {}");

    if list_len == 0 || list_len > ext_len - 2 {
        return Ok(());
    }
    // Parse each protocol
    let end_pos = reader.cursor + list_len as usize;
    while reader.cursor < end_pos && reader.left() >= 1 {
        parse_alpn_item(reader, field)?;
    }

    field.summary = "ALPN Protocol".into();
    Ok(())
}

// Parse Supported Versions extension
// Reference: RFC 8446 Section 4.2.1

fn parse_supported_versions_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    add_field_format_fn!(field, reader, reader.read16(true)?, field_tls_version);
    field.summary = "Supported Versions".into();
    Ok(())
}
fn parse_supported_versions(reader: &mut Reader, field: &mut Field) -> Result<()> {
    if  reader.left() < 1 {
        return Ok(());
    }

    let versions_len = reader.read8()? as usize;

    // Read supported versions length

    if versions_len == 0 || reader.left() < versions_len {
        return Ok(());
    }
    // Parse each version
    let count = versions_len / 2;
    for _ in 0..count {
        parse_supported_versions_item(reader, field)?;
    }
    field.summary = format!("Supported Versions: {}", versions_len);

    Ok(())
}

// Parse Key Share extension
// Reference: RFC 8446 Section 4.2.8

fn parse_key_share_item(reader: &mut Reader, field: &mut Field) -> Result<()> {
    add_field_format_fn!(field, reader, reader.read16(true)?, field_group_ks);
    let key_len = add_field_format!(field, reader, reader.read16(true)?, "Key Exchange Length: {}");
    if key_len > 0 && reader.left() >= key_len as usize {
        reader.forward(key_len as usize);
        add_field_backstep!(field, reader, key_len as usize, format!("Key Exchange Data: {} bytes", key_len));
    }
    field.summary = "Key Share Entry".into();
    Ok(())
}

fn parse_key_share(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let ext_len = reader.left() as u16;
    if reader.left() < 2 {
        return Ok(());
    }

    // Read key share length
    let list_len = add_field_format!(field, reader, reader.read16(true)?, "Key Share Length: {}");

    if list_len == 0 || list_len > ext_len - 2 {
        return Ok(());
    }

    // Create a field for the list
    // let mut list_field = Field::with_children("Key Share Entries".to_string(), reader.cursor, list_len as usize);

    // Parse each key share entry
    let end_pos = reader.cursor + list_len as usize;
    while reader.cursor < end_pos && reader.left() >= 4 {
        parse_key_share_item(reader, field)?;
    }
    field.summary = "Key Share extension".into();
    Ok(())
}
