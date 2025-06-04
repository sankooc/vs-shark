use crate::{
    add_field_backstep, add_field_format, common::{
        concept::Field,
        core::Context,
        enum_def::{AddressField, Protocol, ProtocolInfoField},
        io::{read_mac, Reader},
        quick_hash, EthernetCache, Frame,
    }, constants::etype_mapper, protocol::enthernet_protocol_mapper
};
use anyhow::Result;

pub struct EthernetVisitor {}

impl EthernetVisitor {
    pub fn info(ctx: &Context, frame: &Frame) -> Option<String> {
        if let ProtocolInfoField::Ethernet(key) = &frame.protocol_field {
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
        frame.address_field = AddressField::Mac(key);
        frame.protocol_field = ProtocolInfoField::Ethernet(key);
        if let Some(cache) = ctx.ethermap.get(&key) {
            Ok(enthernet_protocol_mapper(cache.ptype))
        } else {
            let target: [u8; 6] = _reader.slice(6, true)?.try_into()?;
            let source: [u8; 6] = _reader.slice(6, true)?.try_into()?;
            let mut ptype = _reader.read16(true)?;
            if reader.left() == ptype as usize {
                ptype = 1010; // IEEE 802.3
            }
            ctx.ethermap.insert(key, EthernetCache::new(source.into(), target.into(), ptype));
            Ok(enthernet_protocol_mapper(ptype))
        }
    }

    pub fn detail(field: &mut Field, _: &Context, _: &Frame, reader: &mut Reader) -> Result<Protocol> {
        let target = add_field_format!(field, reader, read_mac(reader.slice(6, true)?), "Destination: {}");
        let source = add_field_format!(field, reader, read_mac(reader.slice(6, true)?), "Source: {}");
        let mut ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            ptype = 1010; // IEEE 802.3
            add_field_backstep!(field, reader, 2, format!("Length: {}", ptype));
        } else {
            add_field_backstep!(field, reader, 2, format!("Type: {} ({:#06x})", etype_mapper(ptype), ptype));
        }
        field.summary = format!("Ethernet II, Src: {}, Dst: {}", source, target);
        Ok(enthernet_protocol_mapper(ptype))
    }
}
