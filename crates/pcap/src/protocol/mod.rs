use def::DefaultParser;
use anyhow::Result;

use crate::common::ProtocolElement;

pub mod ethernet;
pub mod def;


pub fn parse(protocol: &'static str, reader: &mut crate::common::io::Reader) -> Result<(&'static str, ProtocolElement)> {
    match protocol {
        "ethernet" => ethernet::index::EthernetVisitor::parse(reader),
        _ => {
            return DefaultParser::parse(reader);
        },
    }
    
}