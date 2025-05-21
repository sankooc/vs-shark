use crate::{
    cache::intern,
    common::{
        concept::Field,
        core::Context,
        enum_def::Protocol,
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
    pub fn parse(ctx: &mut Context, frame: &mut Frame, reader: &mut Reader) -> Result<Protocol> {
        let mut _reader = reader.slice_as_reader(14)?;
        let data = _reader.refer()?;
        let key = quick_hash(data);
        frame.ptr = Some(key);
        if let Some(cache) = ctx.ethermap.get(&key) {
            Ok(enthernet_protocol_mapper(cache.ptype))
        } else {
            let source = _reader.slice(6, true)?.try_into()?;
            let target = _reader.slice(6, true)?.try_into()?;
            let mut ptype = _reader.read16(true)?;
            if reader.left() == ptype as usize {
                ptype = 1010; // IEEE 802.3
            }
            ctx.ethermap.insert(key, EthernetCache::new(source, target, ptype));
            Ok(enthernet_protocol_mapper(ptype))
        }
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
