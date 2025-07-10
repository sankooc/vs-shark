// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_format, add_field_format_fn, add_sub_field, common::{concept::Field, core::Context, enum_def::{AddressField, Protocol}, io::Reader, quick_hash, Frame}, constants::ip_protocol_type_mapper, protocol::ip4_mapper
};
use anyhow::Result;

pub struct Visitor;
pub fn t_protocol(protocol_type: u8) -> String {
    format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(protocol_type as u16), protocol_type)
}

pub fn t_traffic_class(header_field: u32, main_field: &mut Field) {
    main_field.summary = format!("Metadata: 0x{:08x}", header_field);
    if let Some(children) = &mut main_field.children {
        let version = (header_field >> 28) & 0x0F;
        let traffic_class = ((header_field >> 20) & 0xFF) as u8;
        let flow_label = header_field & 0xFFFFF;
        
        children.push(Field::label(
            format!("0110 .... .... .... .... .... .... .... = Version: {}", version),
            0, 1
        ));
        
        let mut tc_field = Field::label(
            format!(".... {:08b} .... .... .... .... .... = Traffic Class: 0x{:02x}", 
                   traffic_class, traffic_class),
            0, 1
        );
        
        let dscp = traffic_class >> 2;
        let ecn = traffic_class & 0x03;
        
        let tc_children = vec![
            Field::label(
                format!(".... {:06b}.. .... .... .... .... .... = Differentiated Services Codepoint: {} (0x{:02x})", 
                      dscp, dscp, dscp),
                0, 1
            ),
            Field::label(
                format!(".... ......{:02b} .... .... .... .... .... = Explicit Congestion Notification: {} (0x{:01x})", 
                      ecn, ecn, ecn),
                0, 1
            )
        ];
        
        tc_field.children = Some(tc_children);
        children.push(tc_field);
        
        children.push(Field::label(
            format!(".... .... .... .... {:020b} = Flow Label: 0x{:05x}", 
                   flow_label, flow_label),
            0, 1
        ));
    }
}
impl Visitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        if let AddressField::IPv6(key) = &frame.address_field {
            if let Some((_, source, target)) = ctx.ipv6map.get(key) {
                return Some(format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target));
            }
        }
        None
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut reader = _reader.slice_as_reader(40)?;
        let data = reader.refer()?;
        let key = quick_hash(data);
        frame.address_field = AddressField::IPv6(key);

        if let Some(enty) = ctx.ipv6map.get(&key) {
            Ok(ip4_mapper(enty.0))
        } else {
            reader.read32(true)?;
            frame.iplen = reader.read16(true)?;
            let protocol_type = reader.read8()?;
            reader.read8()?; //hop
            let source = reader.read_ip6()?;
            let target = reader.read_ip6()?; 
            ctx.ipv6map.insert(key, (protocol_type, source, target));
            Ok(ip4_mapper(protocol_type))
        }
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        // reader.read32(true)?;
        add_sub_field!(field, reader, reader.read32(true)?, t_traffic_class);
        add_field_format!(field, reader, reader.read16(true)?, "Payload Length: {}");
        let protocol_type = add_field_format_fn!(field, reader, reader.read8()?, t_protocol);
        add_field_format!(field, reader, reader.read8()?, "Hop Limit: {}");
        let source = add_field_format!(field, reader, reader.read_ip6()?, "Source Address: {}");
        let target = add_field_format!(field, reader, reader.read_ip6()?, "Destination Address: {}");
        field.summary = format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target);
        Ok(ip4_mapper(protocol_type))
    }
}
