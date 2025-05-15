use std::ops::Range;

use crate::{
    cache::intern,
    common::{
        concept::Field,
        enum_def::{FieldDef, Protocol},
        io::{read_mac, Reader},
        Context, Element, Frame,
    },
    constants::etype_mapper,
    field_back_format,
    protocol::enthernet_protocol_mapper,
    read_field_format,
};
use anyhow::Result;

pub struct EthernetElement {
    pub list: Vec<FieldDef>,
}

impl Element for EthernetElement {
    fn title(&self) -> &'static str {
        todo!()
    }

    fn position(&self) -> Option<Range<u64>> {
        todo!()
    }

    fn children(&self) -> Option<&[FieldDef]> {
        todo!()
    }
}

impl EthernetElement {
    pub fn parse(_data: &[u8]) -> Result<Field> {
        todo!()
    }
}

pub struct EthernetVisitor {}

impl EthernetVisitor {
    pub fn parse(_: &mut Context, frame: &mut Frame, _reader: &mut Reader) -> Result<Protocol> {
        let mut reader = _reader.create_child_reader(14)?;
        let source = read_mac(&mut reader)?;
        let target = read_mac(&mut reader)?;
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
        }
        let info = intern(format!("Ethernet II, Src: {}, Dst: {}", source, target));

        frame.info.info = info;
        frame.info.source = source;
        frame.info.dest = target;
        Ok(enthernet_protocol_mapper(ptype))
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = Vec::new();
        let source = read_field_format!(list, reader, read_mac(reader)?, "Source: {}");
        let target = read_field_format!(list, reader, read_mac(reader)?, "Destination: {}");
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
            field_back_format!(list, reader, 2, format!("Length: {}", ptype));
        } else {
            field_back_format!(list, reader, 2, format!("Type: {} ({:#06x})", etype_mapper(ptype), ptype));
        }
        let info = intern(format!("Ethernet II, Src: {}, Dst: {}", source, target));
        field.summary = info;
        field.children = Some(list);
        Ok(enthernet_protocol_mapper(ptype))
    }
}
