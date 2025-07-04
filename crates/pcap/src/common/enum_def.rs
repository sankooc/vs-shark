// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::net::Ipv4Addr;


use strum_macros::{Display, EnumString};
use thiserror::Error;

use crate::{common::concept::MessageIndex, protocol::transport::tls::TLSList};

use super::{connection::{TCPSegment, TLSSegment}, io::MacAddress};


#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "camel_case")]
pub enum PROPS {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "ethernet.source.mac")]
    EthernetSourceMac,
    #[strum(serialize = "ethernet.destination.mac")]
    EthernetDestinationMac,
    #[strum(serialize = "ethernet.protocol.type")]
    EthernetProtocolType,
    #[strum(serialize = "ip.source")]
    IpSource,
    #[strum(serialize = "ip.destination")]
    IpDestination,
}

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum DataError {
    #[error("unsupport file type")]
    UnsupportFileType,
    #[error("bit error")]
    BitSize,
    #[error("end of stream")]
    EndOfStream,
    #[error("format miss match")]
    FormatMismatch,
    #[error("unimplemented")]
    Unimplemented,
    #[error("ipv4 head length invalid")]
    Ipv4HeadLengthInvalid,
    #[error("ipv4 total length invalid")]
    Ipv4TotalLengthInvalid,
    #[error("http chunk forward error")]
    HttpChunkForwardErr,
}

#[derive(Default, Clone, Copy)]
pub enum FileType {
    PCAP,
    PCAPNG,
    #[default]
    NONE,
}

// pub enum Visitor {
//     ETHERNET(protocol::link::ethernet::Visitor),
//     // ETHERNET(protocol::link::ethernet::Visitor),
//     // SSL,
//     // Loopback(protocol::link::loopback::Visitor),
//     // PPPoES(protocol::link::pppoes::Visitor),
//     // PPPoED,
//     // IP4(protocol::network::ip4::Visitor),
//     // IP6(protocol::network::ip6::Visitor),
//     // ICMP(protocol::network::icmp::Visitor),
//     // ICMP6(protocol::network::icmp6::Visitor),
//     // ARP,
//     // RARP,
//     // RADIOTAP,
//     // IEEE1905A(protocol::link::ieee1905a::Visitor),
//     // IGMP,
//     // UDP,
//     // TCP(protocol::transport::tcp::Visitor),
//     // DNS,
//     // DHCP,
//     // DHCPv6,
//     // HTTP(protocol::application::http::Visitor),
//     // HTTPS,
//     // TLS,
// }
#[derive(Default, Display, Debug, Clone, Copy)]
pub enum Protocol {
    #[default]
    None,
    ETHERNET,
    SSL,
    Loopback,
    PPPoES,
    PPPoED,
    IP4,
    IP6,
    ICMP,
    ICMP6,
    ARP,
    RARP,
    RADIOTAP,
    IEEE1905A,
    IGMP,
    UDP,
    TCP,
    DNS,
    MDNS,
    NBNS,
    DHCP,
    DHCP6,
    HTTP,
    HTTPS,
    TLS,
    IEEE802_11,
}

// #[enum_dispatch]
// pub enum FieldDef {
//     UNKOWN(FieldElement),
// }

#[derive(Clone, PartialEq)]
pub enum TCPDetail {
    KEEPALIVE,
    NOPREVCAPTURE,
    RETRANSMISSION,
    DUMP,
    RESET,
    NEXT,
}

#[derive(Display, Debug, Clone, Copy)]
pub enum TCPFLAG {
    FIN = 0,
    SYNC,
    RESET,
    PUSH,
    ACK,
    URGENT,
    ECN,
    CWR,
    AccurateEcn,
    REVERVED,
}

#[derive(Default, Copy, Clone, serde::Serialize)]
pub enum PacketStatus {
    #[default]
    NORNAL,
    ERROR,
}

#[derive(Default, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TCPConnectStatus {
    #[default]
    INIT,
    CLOSED,
    LISTEN,
    SYN_SENT,
    SYN_RECEIVED,
    ESTABLISHED,
    FIN_WAIT_1,
    FIN_WAIT_2,
    CLOSE_WAIT,
    LAST_ACK,
    TIME_WAIT,
}

pub enum TCPProtocol {
    HTTP,
    HTTPS,
    TLS,
}

#[derive(Default, Clone)]
pub enum SegmentStatus {
    #[default]
    Init,
    HttpDetected(MessageIndex),
    HttpHeaderContinue(MessageIndex, Vec<u8>),
    HttpContentContinue(MessageIndex, usize),
    HttpChunkedContinue(MessageIndex, usize),
    HttpChunkedBroken(MessageIndex, Vec<u8>),
    Error,
    Finish,
    TlsHead(TCPSegment, Vec<u8>),
    TlsSegment(TLSSegment)
}

#[derive(Default)]
pub enum AddressField {
    #[default]
    None,
    Mac(u64),
    IPv4(Ipv4Addr, Ipv4Addr),
    IPv6(u64),
    Ieee80211(u64),
}
#[derive(Default)]
pub enum ProtocolInfoField {
    #[default]
    None,
    Ethernet(u64),
    Http(String, MessageIndex),
    HttpSegment(usize),
    Icmp(u8, u8),
    Icmp6(u8, u8),
    // HttpSegment,
    PPPoES(Option<u8>),
    UDP(u16),
    ARP(u16, u16, MacAddress, Ipv4Addr, MacAddress, Ipv4Addr),
    RARP(u16, u16, MacAddress, Ipv4Addr, MacAddress, Ipv4Addr),
    DHCP(u8),
    DHCPv6(u8, u32),
    DnsQUERY(u16),
    DnsRESPONSE(u16),
    NBNS(u16, bool, String),
    TLS(TLSList),
    TLSSegment,
    Ieee80211(u16),
}
