use anyhow::Result;
use crate::{cache::{self, intern}, common::{enum_def::Protocol, io::{read_mac, Reader}, Frame, ProtocolElement}, constants::{arp_hardware_type_mapper, arp_oper_type_mapper, etype_mapper}, read_field_format, read_field_format_fn};

fn hardware_type_desc(hardware_type: u16) -> String {
    format!("Hardware type: {} ({})", arp_hardware_type_mapper(hardware_type), hardware_type)
}
fn protocol_type_desc(protocol_type: u16) -> String {
    format!("Protocol type: {} ({})", etype_mapper(protocol_type), protocol_type)
}
fn operation_type_desc(operation: u16) -> String {
    format!("Opcode: {} ({})", arp_oper_type_mapper(operation), operation)
}

pub struct Visitor {

}

impl Visitor {
    
    pub fn parse(frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {
        let mut fe = ProtocolElement::new(Protocol::SSL);
        let mut list = vec![];
        read_field_format_fn!(list, reader, reader.read16(true)?, hardware_type_desc);
        read_field_format_fn!(list, reader, reader.read16(true)?, protocol_type_desc);
        read_field_format!(list, reader, reader.read8()?, "Hardware size: {}");
        read_field_format!(list, reader, reader.read8()?, "Protocol size: {}");
        let operation = read_field_format_fn!(list, reader, reader.read16(true)?, operation_type_desc);
        read_field_format!(list, reader, read_mac(reader)?, "Sender MAC address: ({})");
        let source = read_field_format!(list, reader, cache::intern_ip4(reader)?, "Sender IP address: {}");
        read_field_format!(list, reader, read_mac(reader)?, "Target MAC address: ({})");
        let target = read_field_format!(list, reader, cache::intern_ip4(reader)?, "Target IP address: {}");
        
        frame.info.source = source;
        frame.info.dest = target;

        fe.element.title = intern(format!("Address Resolution Protocol ({})", operation));
        fe.element.children = Some(list);
        Ok(("none", fe))
    }
    // pub fn rparse(frame: &mut Frame, reader: &mut Reader) -> Result<(&'static str, ProtocolElement)> {

    // }
}