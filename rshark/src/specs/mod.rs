use crate::files::Visitor;

pub mod ethernet;
pub mod network;
pub mod transport;
pub mod application;

pub fn get_visitor(link_type: u16) -> Option<Box<impl Visitor>> {
  match link_type {
    0 => Some(Box::new(ethernet::EthernetVisitor)),
    1 => Some(Box::new(ethernet::EthernetVisitor)),
    _ => None
  }
}