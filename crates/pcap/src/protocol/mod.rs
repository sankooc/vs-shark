use anyhow::Result;
use def::DefaultParser;

use crate::{cache::intern, common::{
    concept::Field, enum_def::{ FileType, Protocol}, io::Reader, Context, Frame
}};

pub mod link;
pub mod network;
pub mod def;
pub mod transport;

pub fn parse(protocol: Protocol, ctx: &mut Context, frame: &mut Frame, reader: &mut crate::common::io::Reader) -> Result<Protocol> {
    match &protocol {
        Protocol::ETHERNET => link::ethernet::EthernetVisitor::parse(ctx, frame, reader),
        Protocol::SSL => link::ssl::Visitor::parse(ctx, frame, reader),
        Protocol::Loopback => link::loopback::Visitor::parse(ctx, frame, reader),
        Protocol::Ieee1905a => link::ieee1905a::Visitor::parse(ctx, frame, reader),
        Protocol::IP4 => network::ip4::Visitor::parse(ctx, frame, reader),
        Protocol::IP6 => network::ip6::Visitor::parse(ctx, frame, reader),
        // "arp" => network::arp::Visitor::parse(frame, reader),
        // "icmp" => network::icmp::V4Visitor::parse(frame, reader),
        _ => {
            return DefaultParser::parse(frame, reader);
        }
    }
}
pub fn detail(protocol: Protocol, field: &mut Field, ctx: &Context, frame: &Frame, reader: &mut crate::common::io::Reader) -> Result<Protocol> {
    match &protocol {
        Protocol::ETHERNET => link::ethernet::EthernetVisitor::detail(field, ctx, frame, reader),
        Protocol::SSL => link::ssl::Visitor::detail(field, ctx, frame, reader),
        Protocol::Loopback => link::loopback::Visitor::detail(field, ctx, frame, reader),
        Protocol::Ieee1905a => link::ieee1905a::Visitor::detail(field, ctx, frame, reader),
        Protocol::IP4 => network::ip4::Visitor::detail(field, ctx, frame, reader),
        Protocol::IP6 => network::ip6::Visitor::detail(field, ctx, frame, reader),
        _ => {
            field.summary = intern(format!("Unimplement Protocol: {}", protocol));
            Ok(Protocol::None)
            // return DefaultParser::detail(field, ctx, frame, reader);
        }
    }
}


pub fn link_type_map(file_type: &FileType, link_type: u32, reader: &mut Reader) -> Protocol {
    match link_type {
        0 => {
            if let FileType::PCAPNG = file_type {
                return Protocol::Loopback;
            }
            let _head = reader.slice(16, false).unwrap();
            if _head[0] == 0 && _head[5] == 6 {
                let lat = &_head[14..16];
                let _flag = u16::from_be_bytes(lat.try_into().unwrap());
                return match _flag {
                    0x0806 | 0x0800 | 0x86dd | 0x8864 => Protocol::SSL,
                    _ => Protocol::ETHERNET,
                };
            }
            Protocol::ETHERNET
        }
        127 => Protocol::RADIOTAP,
        113 => Protocol::SSL,
        _ => Protocol::ETHERNET,
    }
}

pub fn enthernet_protocol_mapper(ptype: u16) -> Protocol {
    match ptype {
        0x893a => Protocol::Ieee1905a,
        0x0800 => Protocol::IP4,
        0x086dd => Protocol::IP6,
        0x0806 => Protocol::ARP,
        0x8035 => Protocol::RARP,
        // 0x8864 => "pppoes",
        // 0x8863 => "pppoed",
        _ => Protocol::None,
    }
}

pub fn ip4_mapper(ipprototype: u8) -> Protocol {
    match ipprototype {
        1 => Protocol::ICMP,
        2 => Protocol::IGMP,
        6 => Protocol::TCP,
        17 => Protocol::UDP,
        _ => Protocol::None,
    }
}
