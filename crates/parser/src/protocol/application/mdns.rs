// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::{Frame, concept::{Field, NameService}, core::Context, enum_def::Protocol, io::Reader};
use anyhow::Result;


pub struct Visitor;

impl Visitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        super::dns::Visitor::info(ctx, frame)
    }

    pub fn parse(_: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        super::dns::name_service_parse(frame, reader, NameService::MDNS)
    }

    pub fn detail(field: &mut Field, ctx: &Context, frame: &Frame, reader: &mut Reader) -> Result<Protocol> {
        super::dns::Visitor::detail(field, ctx, frame, reader)
    }
}
