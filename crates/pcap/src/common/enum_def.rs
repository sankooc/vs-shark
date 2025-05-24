use std::net::Ipv4Addr;


use strum_macros::{Display, EnumString};
use thiserror::Error;


#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "camel_case")]
pub enum PROPS {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "enthernet.source.mac")]
    EnthernetSourceMac,
    #[strum(serialize = "enthernet.destination.mac")]
    EnthernetDestinationMac,
    #[strum(serialize = "enthernet.protocol.type")]
    EnthernetProtocolType,
    #[strum(serialize = "ip.source")]
    IpSource,
    #[strum(serialize = "ip.destination")]
    IpDestination,
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("unsupport file type")]
    UnsupportFileType,
    #[error("bit error")]
    BitSize,
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
    DHCP,
    DHCPv6,
    HTTP,
    HTTPS,
    TLS,
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
    HttpDetected(usize),
    // HttpHeadParsing,
    // HttpHeadParsed(usize, bool),
    HttpContentContinue(usize, usize),
    HttpChunkedContinue(usize, usize),
    // HttpContentParsing,
    Error,
    Finish,
}

#[derive(Default)]
pub enum AddressField {
    #[default]
    None,
    Mac(Vec<u8>, Vec<u8>),
    IPv4(Ipv4Addr, Ipv4Addr),
    IPv6(u64),
}
#[derive(Default)]
pub enum InfoField {
    #[default]
    None,
    Ethernet(u64),
    Http(Vec<u8>),
    Icmp(u8, u8),
    Icmp6(u8, u8),
    HttpSegment,
    PPPoES(Option<u8>),
}


