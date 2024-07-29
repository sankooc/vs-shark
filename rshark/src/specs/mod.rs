use crate::files::Visitor;

pub mod ethernet;
pub mod network;

pub fn get_visitor(_link_type: u16) -> Option<Box<impl Visitor>> {
  // None
  Some(Box::new(ethernet::EthernetVisitor))
}