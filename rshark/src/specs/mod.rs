use crate::{common::Reader, files::{Frame, Visitor}};

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
use anyhow::Result;

pub fn execute(link_type: u16, frame: &Frame, reader: &Reader)-> Result<()>{
  match link_type {
    113 => ethernet::SSLVisitor.visit(frame, reader),
    _ => ethernet::EthernetVisitor.visit(frame, reader),
  }
}