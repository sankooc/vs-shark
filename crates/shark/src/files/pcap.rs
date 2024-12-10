use crate::common::base::{Configuration, Instance};
use crate::common::io::{AReader, Reader};
use crate::common::FileType;
use anyhow::Result;

pub fn parse(_reader: Reader, conf: Configuration) -> Result<Instance> {
    let mut instance = Instance::new(_reader, FileType::PCAP, conf);
    // let reader = SliceReader::new(data);
    // let reader = &instance.reader;
    let _magic = instance.reader.read32(true)?;
    let major = instance.reader.read16(false)?;
    let minor = instance.reader.read16(false)?;

    let context = &mut instance.ctx;

    let info = &mut context.info;
    info.version = format!("{}-{}", major, minor);
    instance.reader._move(8);
    let _snap_len = instance.reader.read32(false)?;
    // instance.reader._move(2);
    let linktype = instance.reader.read32(false)? & 0x0fffffff;
    info.link_type = linktype;
    while instance.reader.has() {
        let h_ts: u64 = instance.reader.read32(false)?.into();
        let l_ts: u64 = instance.reader.read32(false)?.into();
        let ts: u64 = h_ts * 1000000 + l_ts;
        instance.update_ts(ts);
        let captured = instance.reader.read32(false)?;
        let origin = instance.reader.read32(false)?;
        // let raw = reader.slice(origin as usize);
        let finish = instance.reader.cursor() + origin as usize;
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| instance.create(ts, captured, origin)));
        instance.reader._set(finish);
    }
    instance.flush();
    Ok(instance)
}
