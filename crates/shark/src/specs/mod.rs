use crate::common::{base::{Context, Element, Frame, PacketContext, Visitor}, io::Reader, FileType};
use crate::common::concept::Field;

pub mod arp;
pub mod rarp;
pub mod dhcp;
pub mod dns;
pub mod ethernet;
pub mod http;
pub mod ssdp;
pub mod icmp;
pub mod igmp;
pub mod ip4;
pub mod ip6;
pub mod nbns;
pub mod tcp;
pub mod tls;
pub mod udp;
pub mod sip;
pub mod error;
use anyhow::bail;
use enum_dispatch::enum_dispatch;
use strum_macros::Display;
use crate::common::filter::PacketProps;
use std::cell::RefCell;

pub const DEF_STATUS: &str = "info";

use crate::common::io::AReader;
use crate::common::FIELDSTATUS;
pub fn execute(file_type: &FileType, link_type: u32, _: &Frame, reader: &Reader) -> &'static str {
    match link_type {
        0 => {
            if let FileType::PCAPNG = file_type {
                return "loopback";
            }
            let _head = reader._slice(16);
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

type ERROR = PacketContext<error::Error>;
type ETHERNET = PacketContext<ethernet::ii::Ethernet>;
type NULL = PacketContext<ethernet::null::NULL>;
type PPPoES = PacketContext<ethernet::pppoes::PPPoESS>;
type PPPoED = PacketContext<ethernet::pppoes::PPPoED>;
type SSL = PacketContext<ethernet::ssl::SSL>;
type IPV4 = PacketContext<ip4::IPv4>;
type IPV6 = PacketContext<ip6::IPv6>;
type ARP = PacketContext<arp::ARP>;
type RARP = PacketContext<rarp::RARP>;
type TCP = PacketContext<tcp::TCP>;
type UDP = PacketContext<udp::UDP>;
type ICMP = PacketContext<icmp::ICMP>;
type ICMPv6 = PacketContext<icmp::ICMP6>;
type DNS = PacketContext<dns::DNS>;
// type MDNS = PacketContext<dns::DNS>;
type DHCP = PacketContext<dhcp::DHCP>;
type HTTP = PacketContext<http::HTTP>;
type SSDP = PacketContext<ssdp::SSDP>;
type IGMP = PacketContext<igmp::IGMP>;
type TLS = PacketContext<tls::TLS>;
type IEEE1905A = PacketContext<ethernet::ieee1905a::IEEE1905A>;
type IEE80211 = PacketContext<ethernet::ieee80211::i802::IEE80211>;
type NBNS = PacketContext<nbns::NBNS>;
type Radiotap = PacketContext<ethernet::ieee80211::Radiotap>;
type SIP = PacketContext<sip::SIP>;

#[enum_dispatch]
#[derive(Display)]
// #[strum(serialize_all = "snake_case")]
pub enum ProtocolData {
    ERROR,
    ETHERNET,
    PPPoES,
    PPPoED,
    SSL,
    NULL,
    IPV4,
    IPV6,
    ARP,
    TCP,
    UDP,
    ICMP,
    ICMPv6,
    IGMP,
    RARP,
    DNS,
    // MDNS,
    DHCP,
    HTTP,
    SSDP,
    TLS,
    IEEE1905A,
    IEE80211,
    NBNS,
    Radiotap,
    SIP,
}

pub fn _parse(proto: &'static str) -> anyhow::Result<&'static dyn Visitor>{
    let rs:&dyn Visitor = match proto {
        "ethernet" => &ethernet::ii::EthernetVisitor,
        "pppoes" => &ethernet::pppoes::PPPoESSVisitor,
        "pppoed" => &ethernet::pppoes::PPPoEDVisitor,
        "ssl" => &ethernet::ssl::SSLVisitor,
        "802.11" => &ethernet::ieee80211::i802::IEE80211Visitor,
        "ieee1905.a" => &ethernet::ieee1905a::IEEE1905AVisitor,
        "ipv4" => &ip4::IP4Visitor,
        "ipv6" => &ip6::IP6Visitor,
        "arp" => &arp::ARPVisitor,
        "rarp" => &rarp::RARPVisitor,
        "tcp" => &tcp::TCPVisitor,
        "udp" => &udp::UDPVisitor,
        "icmp" => &icmp::ICMPVisitor,
        "icmpv6" => &icmp::ICMPv6Visitor,
        "igmp" => &igmp::IGMPVisitor,
        "nbns" => &nbns::NBNSVisitor,
        "dns" => &dns::DNSVisitor,
        "ssdp" => &ssdp::SSDPVisitor,
        "mdns" => &dns::MDNSVisitor,
        "dhcp" => &dhcp::DHCPVisitor,
        "loopback" => &ethernet::null::NullVisitor,
        "radiotap" => &ethernet::ieee80211::RadiotapVisitor,
        "sip" => &sip::SIPVisitor,
        // "tls" => &tls::TLSVisitor,
        // "http" => &http::HTTPVisitor,
        _ => bail!("none"),
    };
    Ok(rs)
}

pub fn parse(frame: &mut Frame, ctx: &mut Context, reader: &Reader, proto: &'static str) -> anyhow::Result<Option<(ProtocolData, &'static str)>> {
    let v = _parse(proto);
    match v {
        Ok(visitor) => {
            visitor.visit(frame, ctx, reader).map(|op| Some(op))
        },
        Err(_) => {
            Ok(None)
        }
    }
}
