
use strum_macros::{EnumString, Display};
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

#[derive(Default, Display, Debug, Clone, Copy)]
pub enum Protocol {
    #[default]
    None,
    ETHERNET,
    SSL,
    Loopback,
    IP4,
    IP6,
    ICMP,
    ARP,
    RARP,
    RADIOTAP,
    Ieee1905a,
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