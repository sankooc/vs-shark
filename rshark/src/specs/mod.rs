use crate::files::Visitor;

pub mod ethernet;

pub fn get_visitor(link_type: u16) -> Option<impl Visitor> {
  Some(ethernet::Visitor)
}