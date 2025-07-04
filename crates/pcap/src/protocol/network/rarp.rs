// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::net::Ipv4Addr;

use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{ AddressField, Protocol, ProtocolInfoField},
        io::{MacAddress, Reader},
        Frame,
    },
    constants::{arp_hardware_type_mapper, arp_oper_type_mapper},
    field_back_format,
    read_field_format, read_field_format_fn,
};
use anyhow::Result;

pub fn hardware_type_str(hw_type: u16) -> String {
    format!("Hardware type: {} ({:#06x})", arp_hardware_type_mapper(hw_type), hw_type)
}


pub fn operation_str(operation: u16) -> String {
    format!("Operation: {} ({:#06x})", arp_oper_type_mapper(operation), operation)
}

pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::RARP(_, operation, sender_mac, sender_ip, target_mac, target_ip) = &frame.protocol_field {
            let op_str = arp_oper_type_mapper(*operation);
            return Some(format!("Reverse Address Resolution Protocol ({} {:#06x}), Sender: {} ({}), Target: {} ({})",
                op_str, operation, sender_ip, sender_mac, target_ip, target_mac));
        }
        None
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let hw_type = reader.read16(true)?;
        reader.read16(true)?; 
        let hw_size = reader.read8()?; 
        let proto_size = reader.read8()?;
        let operation = reader.read16(true)?;
        
        let sender_mac_data = reader.slice(hw_size as usize, true)?;
        let sender_mac = MacAddress::from(<[u8; 6]>::try_from(sender_mac_data)?);
        
        let sender_ip_data = reader.slice(proto_size as usize, true)?;
        let sender_ip = Ipv4Addr::from(<[u8; 4]>::try_from(sender_ip_data)?);
        
        let target_mac_data = reader.slice(hw_size as usize, true)?;
        let target_mac = MacAddress::from(<[u8; 6]>::try_from(target_mac_data)?);
        
        let target_ip_data = reader.slice(proto_size as usize, true)?;
        let target_ip = Ipv4Addr::from(<[u8; 4]>::try_from(target_ip_data)?);
        
        frame.protocol_field = ProtocolInfoField::RARP(
            hw_type,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        );
        frame.address_field = AddressField::IPv4(sender_ip, target_ip);
        
        Ok(Protocol::None)
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = vec![];
        
        read_field_format_fn!(list, reader, reader.read16(true)?, hardware_type_str);
        read_field_format!(list, reader, reader.read16(true)?, "Protocol type: IPv4 ({:#06x})");
        
        let hw_size = read_field_format!(list, reader, reader.read8()?, "Hardware size: {}");
        let proto_size = read_field_format!(list, reader, reader.read8()?, "Protocol size: {}");
        
        let operation = read_field_format_fn!(list, reader, reader.read16(true)?, operation_str);
        
        let sender_mac_data = reader.slice(hw_size as usize, true)?;
        let sender_mac = MacAddress::from(<[u8; 6]>::try_from(sender_mac_data)?);
        field_back_format!(list, reader, 6, format!("Sender MAC address: {}", sender_mac));
        
        let sender_ip_data = reader.slice(proto_size as usize, true)?;
        let sender_ip = Ipv4Addr::from(<[u8; 4]>::try_from(sender_ip_data)?);
        field_back_format!(list, reader, 4, format!("Sender IP address: {}", sender_ip));
        
        let target_mac_data = reader.slice(hw_size as usize, true)?;
        let target_mac = MacAddress::from(<[u8; 6]>::try_from(target_mac_data)?);
        field_back_format!(list, reader, 6, format!("Target MAC address: {}", target_mac));
        
        let target_ip_data = reader.slice(proto_size as usize, true)?;
        let target_ip = Ipv4Addr::from(<[u8; 4]>::try_from(target_ip_data)?);
        field_back_format!(list, reader, 4, format!("Target IP address: {}", target_ip));
        
        let op_str = arp_oper_type_mapper(operation);
        field.summary = format!("Reverse Address Resolution Protocol ({} {:#06x})", op_str, operation);
        field.children = Some(list);
        
        Ok(Protocol::None)
    }
}
