use std::{ops::Range, str::from_utf8};

use crate::{
    common::io::{AReader, Reader}, constants::oid_map_mapper, common::base::PacketContext
};
use anyhow::{bail, Result};


pub const BER_SEQUENCE: u8 = 0x30;
pub const BER_SEQUENCE_OF: u8 = 0x0a;
pub const BER_SET: u8 = 0x31;
pub const BER_SET_OF: u8 = 0x0b;
pub const BER_INT: u8 = 0x02;
pub const BER_BIT_STRING: u8 = 0x03;
pub const BER_OCTET_STRING: u8 = 0x04;
pub const BER_NULL: u8 = 0x05;
pub const BER_OBJECT_IDENTIFIER: u8 = 0x06;
pub const BER_UTF_STR: u8 = 0x0c;
pub const BER_PRINTABLE_STR: u8 = 0x13;
pub const BER_IA5STRING: u8 = 0x16;
pub const BER_UTC_TIME: u8 = 0x17;
pub const BER_GENERALIZED_TIME: u8 = 0x18;
// pub enum BER_TYPE {
//     BER_SEQUENCE(30u8),
// }
pub struct TLV;
impl TLV {
    pub fn _read_len(reader: &Reader) -> Result<usize> {
        let _next = reader.read8()?;
        let _len = match _next {
            0x81 => reader.read8()? as usize,
            0x82 => reader.read16(true)? as usize,
            0x83 => {
                let a = reader.read8()? as usize;
                let b = reader.read16(true)? as usize;
                (a << 8) + b
            }
            0x84 => reader.read32(true)? as usize,
            _ => _next as usize,
        };
        Ok(_len)
    }
    pub fn decode(reader: &Reader, len: usize) -> Result<(u8, usize)> {
        let finish = reader.cursor() + len;
        let _type = reader.read8()?;
        // if !self.check(_type) {
        //     bail!("type mismatch");
        // }
        let _len = TLV::_read_len(reader)?;
        if reader.cursor() + _len != finish {
            bail!("length mismatch")
        }
        Ok((_type, _len))
        // self._decode(reader, _type, _len)
    }
}

pub fn pcom_mapper(key: &'static str) -> &'static str {
    match key {
        "4944" => "ID",
        "434352" => "CCR",
        "434353" => "CCS",
        "434345" => "CCE",
        "434349" => "CCI",
        "4343" => "CC",
        "5547" => "UG",
        "5553" => "US",
        "5243" => "RC",
        "5343" => "SC",
        "5245" => "RE",
        "5241" => "RA",
        "4753" => "GS",
        "4746" => "GF",
        "524e48" => "RNH",
        "524e4a" => "RNJ",
        "5242" => "RB",
        "5257" => "RW",
        "524e4c" => "RNL",
        "524e44" => "RND",
        "524e" => "RN",
        "5341" => "SA",
        "5353" => "SS",
        "5346" => "SF",
        "534e48" => "SNH",
        "534e4a" => "SNJ",
        "5342" => "SB",
        "5357" => "SW",
        "534e4c" => "SNL",
        "534e44" => "SND",
        "534e" => "SN",
        _ => key,
    }
}

fn parse_oid(te: &[u8]) -> u64 {
    let _len = te.len();
    let mut val: u64 = 0;
    for inx in 0.._len {
        let v = (te[inx] & 0x7f) as u64;
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
    list.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(".")
}

//https://learn.microsoft.com/zh-cn/windows/win32/seccertenroll/about-object-identifier
// pub trait OBJECTIDENDITY {
//     fn _decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
//         let slice = reader.slice(len);
//         let str = format!("{:x?}", slice);
//         // self.parsed(packet, str); //Algorithm Id: 1.2.840.113549.1.1.11 (sha256WithRSAEncryption)
//         Ok(())
//     }
// }

pub trait SEQUENCE {
    fn decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
        let (_type, _len) = TLV::decode(reader, len)?;
        self._decode(packet, reader, _len)
    }
    fn _sequence(&mut self, packet: &PacketContext<Self>, index: usize, reader: &Reader, _type: u8, len: usize) -> Result<()>;

    fn check(&self, _type: u8) -> bool {
        _type == BER_SEQUENCE || _type == BER_SEQUENCE_OF
    }
    fn _decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
        let finish = reader.cursor() + len;
        let mut index: usize = 0;
        loop {
            let _type = reader.read8()?;
            let _len = TLV::_read_len(reader)?;
            let _finish = _len + reader.cursor();
            self._sequence(packet, index, reader, _type, _len)?;
            let cur = reader.cursor();
            index += 1;
            if cur > _finish {
                bail!("parse_failed");
            }
            if cur < _finish {
                reader._move(_finish - cur);
            }
            if reader.cursor() >= finish {
                break;
            }
        }
        Ok(())
    }
}

pub trait BITSTRING {
    fn decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
        let (_type, _len) = TLV::decode(reader, len)?;
        self._decode(packet, reader, _len)
    }

    fn _decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
        let slice = reader.slice(len);
        let str = format!("{:x?}", slice);
        self.parsed(packet, str);
        Ok(())
    }
    fn parsed(&mut self, packet: &PacketContext<Self>, val: String);
}

pub trait INT {
    fn _decode(&mut self, packet: &PacketContext<Self>, reader: &Reader, len: usize) -> Result<()> {
        let slice = reader.slice(len);
        let str = format!("{:#x?}", slice);
        self.parsed(packet, str);
        Ok(())
    }
    fn parsed(&mut self, packet: &PacketContext<Self>, val: String);
}

pub fn parse(_type: u8, data: &[u8]) -> anyhow::Result<TLVOBJ> {
    match _type {
        0x17 => Ok(TLVOBJ::UTCTime(from_utf8(data)?.into())),
        0x06 => Ok(TLVOBJ::ObjectIdentifier(to_oid(data))),
        0x02 => Ok(TLVOBJ::INT(data.to_vec())),
        0x13 => Ok(TLVOBJ::PrintableStr(from_utf8(data)?.into())),
        BER_BIT_STRING => Ok(TLVOBJ::BITString(data.to_vec())),
        _ => Ok(TLVOBJ::UNKNOWN(data.to_vec()))
    }
}
#[derive(Default)]
pub enum TLVOBJ {
    #[default]
    DEF,
    UNKNOWN(Vec<u8>),
    ObjectIdentifier(String),
    INT(Vec<u8>),
    PrintableStr(String),
    UTCTime(String),
    BITString(Vec<u8>),
}


fn _dis(data: &[u8]) -> String {
    let _len = data.len();
    if _len > 20 {
        let stim = data[0..20].iter().map(|f|format!("{:02x}",f)).collect::<String>();
        return format!("{} ...",stim)
    }
    data.iter().map(|f|format!("{:x}",f)).collect::<String>()
}
impl TLVOBJ {
    pub fn desc(&self)-> &'static str { 
        match self {
            TLVOBJ::ObjectIdentifier(data) => {
                let v = data.clone();
                let s_slice: &'static str = v.leak();
                let outc = format!("{} ({})", data, oid_map_mapper(s_slice));
                outc.leak()
            },
            _ => "",
        }
    }
}
impl std::fmt::Display  for TLVOBJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TLVOBJ::PrintableStr(data) => f.write_str(&data),
            TLVOBJ::ObjectIdentifier(data) => f.write_str(&data),
            TLVOBJ::UTCTime(data) => f.write_fmt(format_args!("20{}", data)),
            TLVOBJ::INT(data) => f.write_str(&_dis(data)),
            TLVOBJ::UNKNOWN(data) => f.write_str(&_dis(data)),
            TLVOBJ::BITString(data) => f.write_str(&_dis(data)),
            TLVOBJ::DEF => f.write_str(""),
        }
    }
}