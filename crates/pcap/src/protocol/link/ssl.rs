// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::concept::Field;
use crate::common::core::Context;
use crate::constants::{link_type_mapper, ssl_type_mapper};
use crate::{add_field_format, add_field_format_fn};
use crate::{
    common::{
        enum_def::Protocol,
        io::{read_mac, Reader},
        Frame,
    },
    constants::etype_mapper,
    protocol::enthernet_protocol_mapper,
};
use anyhow::Result;


const SUMMARY: &'static str = "Linux cooked capture v1";
pub struct Visitor;

pub fn typedesc(_type: u16) -> String {
    format!("Packet Type: {}", ssl_type_mapper(_type))
}

pub fn ptype_str(ptype: u16) -> String {
    format!("Protocol: {} ({:#06x})", etype_mapper(ptype), ptype)
}

fn link_address_type(addr_type: u16) -> String {
    format!("Link-layer address type: {} ({})", link_type_mapper(addr_type), addr_type)
}

impl Visitor {
    pub fn info(_: &Context, _: &Frame) -> Option<String> {
        Some(SUMMARY.to_string())
    }
    pub fn parse(_: &mut Context, _: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.read16(true)?;
        reader.read16(true)?;
        let _len = reader.read16(true)?;
        let _source = read_mac(reader.slice(6, true)?);
        reader.forward(2);
        let ptype = reader.read16(true)?;

        Ok(enthernet_protocol_mapper(ptype))
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let _type = add_field_format_fn!(field, reader, reader.read16(true)?, typedesc);
        add_field_format_fn!(field, reader, reader.read16(true)?, link_address_type);
        add_field_format!(field, reader, reader.read16(true)?, "Link-layer address length: {}");
        add_field_format!(field, reader, read_mac(reader.slice(6, true)?), "Source MAC: {}");
        reader.forward(2);
        let ptype = add_field_format_fn!(field, reader, reader.read16(true)?, ptype_str);
        field.summary = SUMMARY.to_string();
        Ok(enthernet_protocol_mapper(ptype))
    }
}
