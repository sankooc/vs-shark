use anyhow::{bail, Result};

use crate::{
    common::{
        concept::Field, core::Context, enum_def::{FileType, Protocol}, io::Reader, Frame
    },
};

pub mod application;
pub mod link;
pub mod network;
pub mod transport;

pub fn parse(protocol: Protocol, ctx: &mut Context, frame: &mut Frame, reader: &mut crate::common::io::Reader) -> Result<Protocol> {
    match &protocol {
        Protocol::ETHERNET => link::ethernet::EthernetVisitor::parse(ctx, frame, reader),
        Protocol::SSL => link::ssl::Visitor::parse(ctx, frame, reader),
        Protocol::Loopback => link::loopback::Visitor::parse(ctx, frame, reader),
        Protocol::IEEE1905A => link::ieee1905a::Visitor::parse(ctx, frame, reader),
        Protocol::IP4 => network::ip4::Visitor::parse(ctx, frame, reader),
        Protocol::IP6 => network::ip6::Visitor::parse(ctx, frame, reader),
        Protocol::TCP => transport::tcp::Visitor::parse(ctx, frame, reader),
        Protocol::HTTP => application::http::Visitor::parse(ctx, frame, reader),
        Protocol::ICMP => network::icmp::Visitor::parse(ctx, frame, reader),
        Protocol::ICMP6 => network::icmp6::Visitor::parse(ctx, frame, reader),
        Protocol::PPPoES => link::pppoes::Visitor::parse(ctx, frame, reader),
        Protocol::PPPoED => link::pppoed::Visitor::parse(ctx, frame, reader),
        // "arp" => network::arp::Visitor::parse(frame, reader),
        // "icmp" => network::icmp::V4Visitor::parse(frame, reader),
        _ => {
            bail!("finish");
        }
    }
}
pub fn detail(protocol: Protocol, field: &mut Field, ctx: &Context, frame: &Frame, reader: &mut crate::common::io::Reader) -> Result<Protocol> {
    match &protocol {
        Protocol::ETHERNET => link::ethernet::EthernetVisitor::detail(field, ctx, frame, reader),
        Protocol::SSL => link::ssl::Visitor::detail(field, ctx, frame, reader),
        Protocol::Loopback => link::loopback::Visitor::detail(field, ctx, frame, reader),
        Protocol::IEEE1905A => link::ieee1905a::Visitor::detail(field, ctx, frame, reader),
        Protocol::IP4 => network::ip4::Visitor::detail(field, ctx, frame, reader),
        Protocol::IP6 => network::ip6::Visitor::detail(field, ctx, frame, reader),
        Protocol::TCP => transport::tcp::Visitor::detail(field, ctx, frame, reader),
        Protocol::HTTP => application::http::Visitor::detail(field, ctx, frame, reader),
        Protocol::ICMP => network::icmp::Visitor::detail(field, ctx, frame, reader),
        Protocol::ICMP6 => network::icmp6::Visitor::detail(field, ctx, frame, reader),
        Protocol::PPPoES => link::pppoes::Visitor::detail(field, ctx, frame, reader),
        Protocol::PPPoED => link::pppoed::Visitor::detail(field, ctx, frame, reader),
        _ => {
            field.summary = format!("Unimplement Protocol: {}", protocol);
            Ok(Protocol::None)
            // return DefaultParser::detail(field, ctx, frame, reader);
        }
    }
}

pub fn summary(protocol: Protocol, ctx: &Context, frame: &Frame) -> Option<String> {
    match protocol {
        Protocol::TCP => transport::tcp::Visitor::info(ctx, frame),
        Protocol::IP4 => network::ip4::Visitor::info(ctx, frame),
        Protocol::IP6 => network::ip6::Visitor::info(ctx, frame),
        Protocol::HTTP => application::http::Visitor::info(ctx, frame),
        Protocol::ICMP => network::icmp::Visitor::info(ctx, frame),
        Protocol::ICMP6 => network::icmp6::Visitor::info(ctx, frame),
        Protocol::PPPoES => link::pppoes::Visitor::info(ctx, frame),
        Protocol::PPPoED => link::pppoed::Visitor::info(ctx, frame),
        _ => None
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
        0x893a => Protocol::IEEE1905A,
        0x0800 => Protocol::IP4,
        0x086dd => Protocol::IP6,
        0x0806 => Protocol::ARP,
        0x8035 => Protocol::RARP,
        0x8864 => Protocol::PPPoES,
        0x8863 => Protocol::PPPoED,
        _ => Protocol::None,
    }
}

pub fn ip4_mapper(ipprototype: u8) -> Protocol {
    match ipprototype {
        1 => Protocol::ICMP,
        2 => Protocol::IGMP,
        6 => Protocol::TCP,
        17 => Protocol::UDP,
        58 => Protocol::ICMP6,
        _ => Protocol::None,
    }
}
