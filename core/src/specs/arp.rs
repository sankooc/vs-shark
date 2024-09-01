use std::fmt::Formatter;

use pcap_derive::Packet;
use anyhow::Result;

use crate::{
    common::{IPPacket, IPv4Address, MacAddress, Reader}, constants::{arp_hardware_type_mapper, arp_oper_type_mapper, etype_mapper}, files::{Frame, Initer, PacketContext}
};

use super::ProtocolData;

#[derive(Default, Packet)]
pub struct ARP {
    hardware_type: u16,
    protocol_type: u16,
    hardware_size: u8,
    protocol_size: u8,
    operation: u16,
    sender_mac: Option<MacAddress>,
    sender_ip: Option<IPv4Address>,
    target_mac: Option<MacAddress>,
    target_ip: Option<IPv4Address>,
}

impl IPPacket for ARP {
    fn source_ip_address(&self) -> String {
        self.sender_ip.as_ref().unwrap().to_string()
    }
    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
    fn payload_len(&self) -> u16 {
        0
    }
}


impl std::fmt::Display for ARP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Address Resolution Protocol ({})", self._operation_type()))
    }
}
impl crate::files::InfoPacket for ARP {
    fn info(&self) -> String {
        if self.operation == 1 {
            if self.source_ip_address() == self.target_ip_address(){
                format!("ARP Announcement for {}", self.source_ip_address())
            } else {
                format!("who has {}? tell {}", self.target_ip_address(), self.source_ip_address())
            }
        } else {
            format!("{} at {}", self.target_ip_address(), self.source_ip_address())
        }
    }
}
impl ARP {
    fn protocol_type_desc(&self) -> String {
        format!("Protocol type: {} ({})", etype_mapper(self.protocol_type),self.protocol_type)
    }
    fn hardware_type_desc(&self) -> String {
        format!("Hardware type: {} ({})", self._hardware_type(),self.hardware_type)
    }
    fn operation_type_desc(&self) -> String {
        format!("Opcode: {} ({})", self._operation_type(), self.operation)
    }
    fn _hardware_type(&self) -> String {
        arp_hardware_type_mapper(self.hardware_type)
    }
    
    fn _operation_type(&self) -> String {
        arp_oper_type_mapper(self.operation)
    }
}
pub struct ARPVisitor;

impl crate::files::Visitor for ARPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<ARP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.hardware_type = packet.read_with_string(reader, Reader::_read16_be, ARP::hardware_type_desc)?;
        p.protocol_type = packet.read_with_string(reader, Reader::_read16_be, ARP::protocol_type_desc)?;
        p.hardware_size = packet._read_with_format_string_rs(reader, Reader::_read8, "Hardware size: {}")?;
        p.protocol_size = packet._read_with_format_string_rs(reader, Reader::_read8, "Protocol size: {}")?;
        p.operation = packet.read_with_string(reader, Reader::_read16_be, ARP::operation_type_desc)?;
        p.sender_mac = packet._read_with_format_string_rs(reader, Reader::_read_mac, "Sender MAC address: ({})").ok();
        p.sender_ip = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Sender IP address: {}").ok();
        p.target_mac = packet._read_with_format_string_rs(reader, Reader::_read_mac, "Target MAC address: ({})").ok();
        p.target_ip = packet._read_with_format_string_rs(reader, Reader::_read_ipv4, "Target IP address: {}").ok();
        drop(p);
        frame.add_element(ProtocolData::ARP(packet));
        Ok(())
    }
}
