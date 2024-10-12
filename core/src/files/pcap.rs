use crate::common::base::Instance;
use crate::common::FileType;
use crate::common::io::{AReader, SliceReader};
use anyhow::Result;


pub fn parse(data: &[u8]) -> Result<Instance> {
    let mut instance = Instance::new(FileType::PCAP);
    let reader = SliceReader::new(data);
    let _magic = reader.read32(true)?;
    let major = reader.read16(false)?;
    let minor = reader.read16(false)?;
    
    let context = &mut instance.ctx;

    let info = &mut context.info;
    info.version = format!("{}-{}", major, minor);
    reader._move(8);
    let _snap_len = reader.read32(false)?;
    // reader._move(2);
    let linktype = reader.read32(false)? & 0x0fffffff;
    info.link_type = linktype;
    while reader.has() {
        let h_ts: u64 = reader.read32(false)?.into();
        let l_ts: u64 = reader.read32(false)?.into();
        let ts: u64 = h_ts * 1000000 + l_ts;
        instance.update_ts(ts);
        let captured = reader.read32(false)?;
        let origin = reader.read32(false)?;
        let raw = reader.slice(origin as usize);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            instance.create(raw.to_vec(), ts, captured, origin)
        }));
    }
    instance.flush();
    Ok(instance)
}