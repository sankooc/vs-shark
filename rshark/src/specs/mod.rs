use crate::{common::Reader, files::{Frame, Visitor}};

pub mod ethernet;
pub mod network;
pub mod transport;
pub mod application;

pub fn execute(link_type: u16, frame: &Frame, reader: &Reader){
  match link_type {
    113 => ethernet::SSLVisitor.visit(frame, reader),
    _ => ethernet::EthernetVisitor.visit(frame, reader),
  }
}