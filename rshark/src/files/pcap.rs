use crate::common::{FileType, Reader};

use super::Instance;

pub fn parse(data: &[u8]) -> Instance {
    let ctx = Instance::new(FileType::PCAP);
    let reader = Reader::new(data);
    let _magic = reader.read32(true);
    let major = reader.read16(true);
    let minor = reader.read16(true);
    
    let context = ctx.context();
    let mut info = context.info.borrow_mut();
    info.version = format!("{}-{}", major, minor);
    reader._move(8);
    let _snap_len = reader.read32(false);
    reader._move(2);
    let linktype = reader.read16(true);
    info.link_type = linktype;
    while reader.has() {
        let h_ts: u64 = reader.read32(false).into();
        let l_ts: u64 = reader.read32(false).into();
        let ts: u64 = h_ts * 1000000 + l_ts;
        ctx.update_ts(ts);
        let captured = reader.read32(false);
        let origin = reader.read32(false);
        // info!("ts {}, {}, {}", ts, captured, origin);
        let raw = reader.slice(origin as usize);
        ctx.create(raw, ts, captured, origin);
    }
    ctx
}
