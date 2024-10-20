use std::fmt::Formatter;

use super::ProtocolData;
use crate::common::io::Reader;
use crate::{
    common::base::{Frame, PacketBuilder, PacketContext, PacketOpt},
    common::{IPPacket, MacAddress, FIELDSTATUS},
    constants::{arp_hardware_type_mapper, arp_oper_type_mapper, etype_mapper},
};
use anyhow::Result;
use pcap_derive::{Packet2, Visitor3};
use std::net::Ipv4Addr;

#[derive(Default, Packet2)]
pub struct ARP {
    hardware_type: u16,
    protocol_type: u16,
    hardware_size: u8,
    protocol_size: u8,
    operation: u16,
    sender_mac: Option<MacAddress>,
    sender_ip: Option<Ipv4Addr>,
    target_mac: Option<MacAddress>,
    target_ip: Option<Ipv4Addr>,
}

impl IPPacket for ARP {
    fn source_ip_address(&self) -> String {
        self.sender_ip.as_ref().unwrap().to_string()
    }
    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
    fn payload_len(&self) -> Option<u16> {
        Some(0)
    }
}

impl std::fmt::Display for ARP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Address Resolution Protocol ({})", self._operation_type()))
    }
}
impl crate::common::base::InfoPacket for ARP {
    fn info(&self) -> String {
        if self.operation == 1 {
            if self.source_ip_address() == self.target_ip_address() {
                format!("ARP Announcement for {}", self.source_ip_address())
            } else {
                format!("who has {}? tell {}", self.target_ip_address(), self.source_ip_address())
            }
        } else {
            format!("{} at {}", self.target_ip_address(), self.source_ip_address())
        }
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
impl ARP {
    fn protocol_type_desc(&self) -> String {
        format!("Protocol type: {} ({})", etype_mapper(self.protocol_type), self.protocol_type)
    }
    fn hardware_type_desc(&self) -> String {
        format!("Hardware type: {} ({})", self._hardware_type(), self.hardware_type)
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
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.hardware_type = packet.build_lazy(reader, Reader::_read16_be, Some("arp.hardware.type"), ARP::hardware_type_desc)?;
        p.protocol_type = packet.build_lazy(reader, Reader::_read16_be, Some("arp.protocol.type"), ARP::protocol_type_desc)?;
        p.hardware_size = packet.build_format(reader, Reader::_read8, Some("arp.hardware.len"), "Hardware size: {}")?;
        p.protocol_size = packet.build_format(reader, Reader::_read8, Some("arp.protocol.len"), "Protocol size: {}")?;
        p.operation = packet.build_lazy(reader, Reader::_read16_be, Some("arp.operation.type"), ARP::operation_type_desc)?;
        p.sender_mac = packet.build_format(reader, Reader::_read_mac, Some("arp.sender.mac.address"), "Sender MAC address: ({})").ok();
        p.sender_ip = packet.build_format(reader, Reader::_read_ipv4, Some("arp.sender.ip.address"), "Sender IP address: {}").ok();
        p.target_mac = packet.build_format(reader, Reader::_read_mac, Some("arp.target.mac.address"), "Target MAC address: ({})").ok();
        p.target_ip = packet.build_format(reader, Reader::_read_ipv4, Some("arp.target.mac.address"), "Target IP address: {}").ok();
        Ok(())
    }
}

#[derive(Visitor3)]
pub struct ARPVisitor;

impl ARPVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = ARP::create(reader, None)?;
        Ok((ProtocolData::ARP(packet), "none"))
    }
}
