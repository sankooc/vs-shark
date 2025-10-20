// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::{
    cell::{Cell, RefCell},
    ops::Range,
    rc::Rc,
    str::from_utf8,
};

use crate::{
    add_field_backstep, add_field_forward, add_sub_field_with_reader, common::{concept::Field, io::Reader, util::{bytes_to_hex, bytes_to_hex_limit}}, constants::oid_map_mapper, protocol::transport::tls::record::read24
};
use anyhow::Result;
pub const BER_SEQUENCE: u8 = 0x30;
pub const BER_SEQUENCE_OF: u8 = 0x0a;
pub const BER_SET: u8 = 0x31;
pub const BER_SET_OF: u8 = 0x0b;
pub const BER_INT: u8 = 0x02;
pub const BER_BIT_STRING: u8 = 0x03;
// pub const BER_OCTET_STRING: u8 = 0x04;
pub const BER_NULL: u8 = 0x05;
pub const BER_OBJECT_IDENTIFIER: u8 = 0x06;
// pub const BER_UTF_STR: u8 = 0x0c;
pub const BER_PRINTABLE_STR: u8 = 0x13;
// pub const BER_IA5STRING: u8 = 0x16;
pub const BER_UTC_TIME: u8 = 0x17;
// pub const BER_GENERALIZED_TIME: u8 = 0x18;
pub const BER_CONSTRUCT: u8 = 0xa0;
pub const BER_CUSTOM_EXT: u8 = 0xa3;

fn parse_oid(te: &[u8]) -> u64 {
    let _len = te.len();
    let mut val: u64 = 0;
    for (inx, item) in te.iter().enumerate().take(_len) {
        let v = (*item & 0x7f) as u64;
        let offset = (_len - 1 - inx) * 7;
        val += v << offset;
    }
    val
}
pub fn to_oid(slice: &[u8]) -> String {
    let mut list: Vec<u64> = Vec::new();
    let _len = slice.len();
    let h = slice[0] / 40;
    let l = slice[0] % 40;
    list.push(h as u64);
    list.push(l as u64);
    let mut range = Range { start: 1, end: 1 };
    for inx in 1.._len {
        if slice[inx] > 0x7f {
            continue;
        } else {
            range.end = inx + 1;
            list.push(parse_oid(&slice[range]));
            range = Range { start: inx + 1, end: inx + 1 }
        }
    }
    list.iter().map(|f| format!("{f}")).collect::<Vec<_>>().join(".")
}

pub enum BerType {
    Int(Vec<u8>),
    // BitString,
    ObjectIdentifier(String),
    String(String),
    UTCTime(String),
}

pub fn _read_len(reader: &mut Reader) -> Result<usize> {
    let _next = reader.read8()?;
    let _len = match _next {
        0x81 => reader.read8()? as usize,
        0x82 => reader.read16(true)? as usize,
        0x83 => read24(reader)? as usize,
        0x84 => reader.read32(true)? as usize,
        _ => _next as usize,
    };
    Ok(_len)
}
pub trait Sequence {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>>;
    fn val(&self, index: usize, ber_type: BerType) -> Option<String>;
    fn unknown(&self, index: usize, reader: &mut Reader) -> Option<String>;
    fn text(&self) -> String;
}
pub struct Certificate;
pub struct SignedCertificate;

pub struct CollectionOnly {
    text: Option<String>,
    list: Vec<Rc<dyn Sequence>>,
    cb: Option<fn(usize) -> String>,
}

impl Sequence for CollectionOnly {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>> {
        self.list.get(index).cloned()
    }
    fn val(&self, _: usize, _: BerType) -> Option<String> {
        None
    }
    fn text(&self) -> String {
        if let Some(text) = &self.text {
            return text.clone();
        }
        if let Some(func) = &self.cb {
            return func(self.list.len());
        }
        "".to_string()
    }
    
    fn unknown(&self, _: usize, _: &mut Reader) -> Option<String> {
        None
    }
}

pub trait ContructMono {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>>;
    fn text(&self, text: &str, count: u8) -> String;
}

pub struct ContructOnly {
    text: RefCell<String>,
    count: Cell<u8>,
    mono: Rc<dyn ContructMono>,
}

impl ContructOnly {
    pub fn create(text: String, mono: impl ContructMono + 'static) -> Self {
        Self {
            text: RefCell::new(text),
            count: Cell::new(0),
            mono: Rc::new(mono),
        }
    }
}
impl Sequence for ContructOnly {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>> {
        self.count.set(self.count.get() + 1);
        self.mono.collection(index)
    }

    fn val(&self, _: usize, _: BerType) -> Option<String> {
        None
    }

    fn text(&self) -> String {
        self.mono.text(&self.text.borrow(), self.count.get())
    }
    
    
    fn unknown(&self, _: usize, _: &mut Reader) -> Option<String> {
        None
    }
}

pub struct RNDItem;
impl ValueMono for RNDItem {
    fn val(&self, _: usize, ber_type: BerType) -> Option<String> {
        match ber_type {
            BerType::ObjectIdentifier(data) => {
                let oid = oid_map_mapper(&data);
                return Some(format!("Object Id: {data} ({oid})"));
            }
            BerType::String(content) => return Some(content.clone()),
            _ => {}
        }
        None
    }
}


pub struct SubjectPublicKeyInfo;
impl ContructMono for SubjectPublicKeyInfo {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>> {
        match index {
            0 => create_value_list("algorithm", SignedCertificateSignature),
            1 => create_value_list("subjectPublicKey", SubjectPublicKey),
            _ => None
        }
    }

    fn text(&self, _: &str, _: u8) -> String {
        "subjectPublicKeyInfo".into()
    }
}
pub struct SubjectPublicKey;
impl ValueMono for SubjectPublicKey {
    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        if let BerType::Int(data) = ber_type {
            let content = bytes_to_hex_limit(&data, 20);
            match index {
                0 => return Some(format!("modules: {content}")),
                1 => return Some(format!("public exponent: {content}")),
                _ => {}
            }
        }
        None
    }
}

pub struct Rdn;

impl ContructMono for Rdn {
    fn collection(&self, _: usize) -> Option<Rc<dyn Sequence>> {
        create_construct_list("item", RDNGroup)
    }
    fn text(&self, text: &str, count: u8) -> String {
        format!("{text} ({count})")
    }
}

pub struct RDNGroup;

impl ContructMono for RDNGroup {
    fn collection(&self, _: usize) -> Option<Rc<dyn Sequence>> {
        Some(Rc::new(ValueOnly::create("rdnSequence".into(), RNDItem)))
    }
    fn text(&self, text: &str, count: u8) -> String {
        format!("{text} ({count})")
        
    }
}


pub struct Extentions;
impl ContructMono for Extentions {
    fn collection(&self, _: usize) -> Option<Rc<dyn Sequence>> {
        create_value_list("Extension", Extension)
    }
    fn text(&self, _: &str, count: u8) -> String {
        format!("extensions: {count} items")
    }
}

pub struct Extension;
impl ValueMono for Extension {
    fn val(&self, _: usize, ber_type: BerType) -> Option<String> {
        if let BerType::ObjectIdentifier(data) = ber_type {
            let oid = oid_map_mapper(&data);
            return Some(format!("Extension Id: {} ({})", data, oid));
        }
        None
    }
}

pub trait ValueMono {
    fn val(&self, index: usize, ber_type: BerType) -> Option<String>;
}
pub struct ValueOnly {
    text: String,
    mono: Rc<dyn ValueMono>,
}

impl ValueOnly {
    pub fn create(text: String, mono: impl ValueMono + 'static) -> Self {
        Self { text, mono: Rc::new(mono) }
    }
}

impl Sequence for ValueOnly {
    fn collection(&self, _: usize) -> Option<Rc<dyn Sequence>> {
        None
    }

    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        self.mono.val(index, ber_type)
    }

    fn text(&self) -> String {
        self.text.clone()
    }
    
    
    fn unknown(&self, _: usize, _: &mut Reader) -> Option<String> {
        None
    }
}

pub struct SignedCertificateVersion;
pub struct SignedCertificateSignature;

impl Sequence for Certificate {
    fn text(&self) -> String {
        "Certificate".to_string()
    }

    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>> {
        match index {
            0 => Some(Rc::new(SignedCertificate)),
            1 => create_value_list("algorithmIdentifier", SignedCertificateSignature),
            _ => None,
        }
    }

    fn val(&self, _: usize, _: BerType) -> Option<String> {
        None
    }
    
    
    fn unknown(&self, index : usize, reader: &mut Reader) -> Option<String> {
        if index == 2 {
            let len: usize = reader.left();
            let size = std::cmp::min(len, 20);
            if size > 0 {
                let data = reader.slice(size, false).unwrap();
                return Some(format!("encrypted [truncated]: {}", bytes_to_hex(data)));
            }
            
        }
        None
    }
}

impl Sequence for SignedCertificate {
    fn collection(&self, index: usize) -> Option<Rc<dyn Sequence>> {
        match index {
            0 => Some(Rc::new(ValueOnly::create("SignedCertificateVersion".into(), SignedCertificateVersion))),
            2 => create_value_list("SignedCertificateSignature", SignedCertificateSignature),
            3 => create_construct_list("issuer: rdnSequence", Rdn),
            4 => create_value_list("validity", Validity),
            5 => create_construct_list("subject: rdnSequence", Rdn),
            6 => create_construct_list("subjectPublicKey", SubjectPublicKeyInfo),
            7 => create_construct_list("", Extentions),
            _ => None,
        }
    }

    fn text(&self) -> String {
        "signedCertificate".to_string()
    }

    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        match index {
            // 0 => {
            //     if let BerType::Int(data) = ber_type {
            //         return Some(format!("version: {}", data[0]));
            //     }
            // },
            1 => {
                if let BerType::Int(data) = ber_type {
                    return Some(format!("serialNumber: {}", bytes_to_hex(&data)));
                }
            }
            _ => {
                return Some("Unkown field".to_string());
            }
        }
        None
    }
    
    
    fn unknown(&self, _: usize, _: &mut Reader) -> Option<String> {
        None
    }
}

impl ValueMono for SignedCertificateVersion {
    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        match index {
            0 => {
                if let BerType::Int(data) = ber_type {
                    return Some(format!("version: {}", data[0]));
                }
            }
            1 => {
                if let BerType::Int(data) = ber_type {
                    return Some(format!("serialNumber: {}", bytes_to_hex(&data)));
                }
            }
            _ => {}
        }
        None
    }
}

impl ValueMono for SignedCertificateSignature {
    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        if index == 0 {
            if let BerType::ObjectIdentifier(data) = ber_type {
                let oid = oid_map_mapper(&data);
                return Some(format!("Algorithm Id: {data} ({oid})"));
            }
        }
        None
    }
}

pub struct Validity;
impl ValueMono for Validity {
    fn val(&self, index: usize, ber_type: BerType) -> Option<String> {
        if let BerType::UTCTime(data) = ber_type {
            match index {
                0 => return Some(format!("notBefore: {data}")),
                1 => return Some(format!("notAfter: {data}")),
                _ => {}
            }
        }
        None
    }
}


pub fn create_construct_list(text: impl ToString, mono: impl ContructMono + 'static) -> Option<Rc<dyn Sequence>> {
    Some(Rc::new(ContructOnly::create(text.to_string(), mono)))
}
pub fn create_value_list(text: impl ToString, mono: impl ValueMono + 'static) -> Option<Rc<dyn Sequence>> {
    Some(Rc::new(ValueOnly::create(text.to_string(), mono)))
}

pub fn parse_sequence(t: Rc<dyn Sequence>, _reader: &mut Reader, field: &mut Field) -> Result<()> {
    let mut index = 0;
    loop {
        if _reader.left() == 0 {
            break;
        }
        let _cursor = _reader.cursor;
        let _type = _reader.read8()?;
        let len = _read_len(_reader)?;
        match _type {
            BER_SEQUENCE | BER_SEQUENCE_OF | BER_CONSTRUCT | BER_SET | BER_SET_OF => {
                let mut reader2 = _reader.slice_as_reader(len)?;
                if let Some(sub) = t.collection(index) {
                    add_sub_field_with_reader!(field, &mut reader2, move |reader_, field_| {
                        let _ = parse_sequence(sub, reader_, field_);
                    });
                }
            }
            BER_CUSTOM_EXT => {
                continue;
            }
            BER_BIT_STRING => {
                let unused = _reader.read8()? as usize;
                if unused < len {
                    //todo 
                }
                if unused > 0 {
                    _reader.forward(unused);
                }
                continue;
            },
            BER_INT => {
                let data = _reader.slice(len, true)?.to_vec();
                if let Some(v) = t.val(index, BerType::Int(data)) {
                    add_field_backstep!(field, _reader, len, v);
                }
            }
            BER_OBJECT_IDENTIFIER => {
                let data = _reader.slice(len, true)?.to_vec();
                if let Some(v) = t.val(index, BerType::ObjectIdentifier(to_oid(&data))) {
                    add_field_backstep!(field, _reader, len, v);
                }
            }
            BER_PRINTABLE_STR => {
                let data = _reader.slice(len, true)?;
                let content = from_utf8(data)?.to_string();
                if let Some(v) = t.val(index, BerType::String(content)) {
                    add_field_backstep!(field, _reader, len, v);
                }
            }
            BER_UTC_TIME => {
                let data = _reader.slice(len, true)?;
                let content = format!("20{}", from_utf8(data)?);
                if let Some(v) = t.val(index, BerType::UTCTime(content)) {
                    add_field_backstep!(field, _reader, len, v);
                }
            }
            BER_NULL => {
                if len > 0 {
                    _reader.slice(len, true)?;
                }
            }
            // BER_CONSTRUCT => {
            //     let mut reader2 = _reader.slice_as_reader(len)?;
            //     add_sub_field_with_reader!(field, &mut reader2, move |reader, field| T::parse(index, reader, field))?;
            // }
            _ => {
                _reader.cursor = _cursor;
                let len = _reader.left();
                if let Some(v) = t.unknown(index, _reader) {
                    add_field_forward!(field, _reader, len, v);
                }
                break;
            }
        }
        // T::parse(index, &mut reader2)?;
        index += 1;
    }
    field.summary = t.text();
    Ok(())
}

pub fn parse_cert(reader: &mut Reader, field: &mut Field) -> Result<()> {
    let _type = reader.read8()?; // 0x30
    let len = _read_len(reader)?;
    let mut _reader = reader.slice_as_reader(len)?;
    parse_sequence(Rc::new(Certificate), &mut _reader, field)
}
