use crate::files::Visitor;

pub mod ethernet;

pub fn get_visitor(link_type: u16) -> Option<Box<dyn Visitor>> {
  Some(Box::new(ethernet::Visitor))
}