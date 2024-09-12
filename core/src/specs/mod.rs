use crate::{common::{Reader, IO}, files::{Element, Field, Frame, PacketContext, Visitor}};

pub mod ethernet;
pub mod ip4;
pub mod ip6;
pub mod arp;
pub mod icmp;
pub mod udp;
pub mod dns;
pub mod tcp;
pub mod dhcp;
pub mod http;
pub mod igmp;
pub mod tls;
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use strum_macros::Display;

pub fn execute(link_type: u32, frame: &Frame, reader: &Reader)-> Result<()>{
  match link_type {
    0 => {
      let _head = reader._slice(16);
      if _head[0] == 0 && _head[5] == 6 {
        let lat = &_head[14..16];
        let _flag = u16::from_be_bytes(lat.try_into().unwrap());
        return match _flag {
          0x0806 | 0x0800 | 0x86dd | 0x8864 => ethernet::SSLVisitor.visit(frame, reader),
          _ => ethernet::EthernetVisitor.visit(frame, reader),
        }
      }
      ethernet::EthernetVisitor.visit(frame, reader)
    },
    113 => ethernet::SSLVisitor.visit(frame, reader),
    _ => ethernet::EthernetVisitor.visit(frame, reader),
  }
}

type ETHERNET = PacketContext<ethernet::Ethernet>;
type PPPoESS = PacketContext<ethernet::PPPoESS>;
type SSL = PacketContext<ethernet::SSL>;
type IPV4 = PacketContext<ip4::IPv4>;
type IPV6 = PacketContext<ip6::IPv6>;
type ARP = PacketContext<arp::ARP>;
type TCP = PacketContext<tcp::TCP>;
type UDP = PacketContext<udp::UDP>;
type ICMP = PacketContext<icmp::ICMP>;
type ICMPv6 = PacketContext<icmp::ICMP6>;
type DNS = PacketContext<dns::DNS>;
type DHCP = PacketContext<dhcp::DHCP>;
type HTTP = PacketContext<http::HTTP>;
type IGMP = PacketContext<igmp::IGMP>;
type TLS = PacketContext<tls::TLS>;
type IEEE1905A = PacketContext<ethernet::IEEE1905A>;


#[enum_dispatch]
#[derive(Display)]
// #[strum(serialize_all = "snake_case")]
pub enum ProtocolData {
    ETHERNET,
    PPPoESS,
    SSL,
    IPV4,
    IPV6,
    ARP,
    TCP,
    UDP,
    ICMP,
    ICMPv6,
    IGMP,
    DNS,
    DHCP,
    HTTP,
    TLS,
    IEEE1905A,
}
