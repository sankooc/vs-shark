use anyhow::{bail, Result};
// // use instant::Instant;

// pub fn parse(_reader: Reader, conf: Configuration) -> Result<Instance> {
//     // let start = Instant::now();
//     let mut instance = Instance::new(_reader, FileType::PCAP, conf);
//     let _magic = instance.reader.read32(true)?;
//     let major = instance.reader.read16(false)?;
//     let minor = instance.reader.read16(false)?;

//     let context = &mut instance.ctx;

//     let info = &mut context.info;
//     info.version = format!("{}-{}", major, minor);
//     instance.reader._move(8);
//     let _snap_len = instance.reader.read32(false)?;
//     // instance.reader._move(2);
//     let linktype = instance.reader.read32(false)? & 0x0fffffff;
//     info.link_type = linktype;
//     while instance.reader.has() {
//         let h_ts: u64 = instance.reader.read32(false)?.into();
//         let l_ts: u64 = instance.reader.read32(false)?.into();
//         let ts: u64 = h_ts * 1000000 + l_ts;
//         instance.update_ts(ts);
//         let captured = instance.reader.read32(false)?;
//         let origin = instance.reader.read32(false)?;
//         // let raw = reader.slice(origin as usize);
//         let finish = instance.reader.cursor() + origin as usize;
//         let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| instance.create(ts, captured, origin)));
//         instance.reader._set(finish);
//     }
//     instance.flush();
//     // let elapsed = start.elapsed().as_millis() as usize;
//     // instance.ctx.cost = elapsed;
//     Ok(instance)
// }

use crate::common::{io::Reader, Frame};

pub struct PCAP {}

impl PCAP {
    pub fn new() -> Self {
        Self {}
    }
}

impl PCAP {

    pub fn next(reader: &mut Reader) -> Result<(usize, Frame)> {
        if reader.left() < 16 {
            bail!("end")
        }
        // let cursor = reader.cursor;
        let t = reader.slice(8, true)?.to_vec();
        let captured = reader.read32(false)?;
        let origin = reader.read32(false)?;
        if captured != origin {
            bail!("nomatch")
        }
        if reader.left() < (origin as usize) {
            reader.back(16);
            bail!("end of stream")
        }
        let mut f = Frame::new();
        f.size = origin;
        f.time = Some(t);
        f.range = Some(reader.cursor..reader.cursor + origin as usize);
        Ok((origin as usize + reader.cursor, f))
    }
}
