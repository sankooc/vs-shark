// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{
    add_field_format, common::{concept::Field, core::Context, enum_def::Protocol, io::Reader, Frame}
};
use anyhow::Result;

pub struct Visitor;
impl Visitor {
    pub fn parse(_: &mut Context, _: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.forward(7);
        Ok(Protocol::None)
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        add_field_format!(field, reader, reader.read8()?, "Message version: {}");
        reader.forward(1);
        add_field_format!(field, reader, reader.read16(true)?, "Message type: ({})");
        add_field_format!(field, reader, reader.read16(true)?, "Message id: ({})");
        add_field_format!(field, reader, reader.read8()?, "Fragment id: ({})");
        field.summary = String::from("IEEE 1905.1a");
        // TODO 
        Ok(Protocol::None)
    }
}
