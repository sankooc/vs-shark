use crate::{
    common::{
        concept::Field,
        core::Context,
        enum_def::{AddressField, InfoField, Protocol},
        io::{read_mac, Reader},
        quick_hash, EthernetCache, Frame,
    },
    constants::etype_mapper,
    field_back_format,
    protocol::enthernet_protocol_mapper,
    read_field_format,
};
use anyhow::Result;

pub struct EthernetVisitor {}

impl EthernetVisitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        if let InfoField::Ethernet(key) = &frame.info_field {
            if let Some(ech) = ctx.ethermap.get(&key) {
                return Some(format!("Ethernet II, Src: {}, Dst: {}", ech.source, ech.target));
            }
        }
        None
    }
    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut _reader = reader.slice_as_reader(14)?;
        let data = _reader.refer()?;
        let key = quick_hash(data);
        frame.ip_field = AddressField::Mac(key);
        frame.info_field = InfoField::Ethernet(key);
        if let Some(cache) = ctx.ethermap.get(&key) {
            Ok(enthernet_protocol_mapper(cache.ptype))
        } else {
            let source: [u8; 6] = _reader.slice(6, true)?.try_into()?;
            let target: [u8; 6] = _reader.slice(6, true)?.try_into()?;
            let mut ptype = _reader.read16(true)?;
            if reader.left() == ptype as usize {
                ptype = 1010; // IEEE 802.3
            }
            ctx.ethermap.insert(key, EthernetCache::new(source.into(), target.into(), ptype));
            Ok(enthernet_protocol_mapper(ptype))
        }
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut list = Vec::new();
        let source = read_field_format!(list, reader, read_mac(reader.slice(6, true)?), "Source: {}");
        let target = read_field_format!(list, reader, read_mac(reader.slice(6, true)?), "Destination: {}");
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
            field_back_format!(list, reader, 2, format!("Length: {}", ptype));
        } else {
            field_back_format!(list, reader, 2, format!("Type: {} ({:#06x})", etype_mapper(ptype), ptype));
        }
        field.summary = format!("Ethernet II, Src: {}, Dst: {}", source, target);
        field.children = Some(list);
        Ok(enthernet_protocol_mapper(ptype))
    }
}
