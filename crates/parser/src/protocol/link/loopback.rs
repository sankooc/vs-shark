// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::{add_field_format, common::{concept::Field, core::Context, enum_def::Protocol, io::Reader, Frame}};
use anyhow::Result;

const SUMMARY: &str = "Null/Loopback";
pub struct Visitor;

impl Visitor {
    pub fn info(_: &Context, _: &Frame) -> Option<String> {
        Some(SUMMARY.to_string())
    }
    pub fn parse(_: &mut Context, _: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        reader.read32(false)?;
        let _next = reader.next()?;
        if _next == 0x45 {
            Ok(Protocol::IP4)
        } else {
            Ok(Protocol::None)
        }
    }
    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        add_field_format!(field, reader, reader.read32(false)?, "Family: {}");
        let _next = reader.next()?;
        field.summary = SUMMARY.to_string();
        if _next == 0x45 {
            Ok(Protocol::IP4)
        } else {
            Ok(Protocol::None)
        }
    }
}
