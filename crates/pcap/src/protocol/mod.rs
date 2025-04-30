use def::DefaultParser;
use anyhow::Result;

use crate::common::{enum_def::FileType, io::Reader, ProtocolElement};

pub mod ethernet;
pub mod def;


pub fn parse(protocol: &'static str, reader: &mut crate::common::io::Reader) -> Result<(&'static str, ProtocolElement)> {
    match protocol {
        "ethernet" => ethernet::index::EthernetVisitor::parse(reader),
        _ => {
            return DefaultParser::parse(reader);
        },
    }
    
}

pub fn link_type_map(file_type: &FileType, link_type: u32, reader: &mut Reader) -> &'static str {
    match link_type {
        0 => {
            if let FileType::PCAPNG = file_type {
                return "loopback";
            }
            let _head = reader.slice(16, false).unwrap();
            if _head[0] == 0 && _head[5] == 6 {
                let lat = &_head[14..16];
                let _flag = u16::from_be_bytes(lat.try_into().unwrap());
                return match _flag {
                    0x0806 | 0x0800 | 0x86dd | 0x8864 => "ssl",
                    _ => "ethernet",
                };
            }
            "ethernet"
        }
        127 => "radiotap",
        113 => "ssl",
        _ => "ethernet",
    }
}

pub fn enthernet_protocol_mapper(ptype: u16) -> &'static str {
    match ptype {
        0x893a => "ieee1905.a",
        0x0800 => "ipv4",
        0x086dd => "ipv6",
        0x0806 => "arp",
        0x8035 => "rarp",
        0x8864 => "pppoes",
        0x8863 => "pppoed",
        _ => "none",
    }
}