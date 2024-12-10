use crate::common::base::{Configuration, Instance};
use crate::common::io::{AReader, Reader, SliceReader};
use crate::common::FileType;
use anyhow::Result;
// use instant::Instant;

fn parse_head(data: &[u8]) -> Result<String> {
    let reader = SliceReader::new(data);
    let _magic = reader.read32(false)?;
    let major = reader.read16(false)?;
    let minor = reader.read16(false)?;
    Ok(format!("{}-{}", major, minor))
}

fn parse_interface(data: &[u8]) -> Result<u16> {
    let reader = SliceReader::new(data);
    let lt = reader.read16(false)?;
    let _revert = reader.read16(false)?;
    let _snap_len = reader.read32(true)?;
    Ok(lt)
}
pub fn parse(reader: Reader, conf: Configuration) -> Result<Instance> {
    let mut instance = Instance::new(reader, FileType::PCAPNG, conf);
    // let start = Instant::now();
    loop {
        let block_type = format!("{:#010x}", instance.reader.read32(false)?);
        let len = instance.reader.read32(false)?;
        let ctx = &mut instance.ctx;
        
        let packet_size = len as usize - 12;
        if instance.reader.left() < packet_size {
            break;
        }
        // if len == _len {
            match block_type.as_str() {
                "0x0a0d0d0a" => {
                    let raw = instance.reader.slice(packet_size);
                    let _len = instance.reader.read32(false)?;
                    let info = &mut ctx.info;
                    info.version = parse_head(&raw)?;
                }
                "0x00000001" => {
                    let raw = instance.reader.slice(packet_size);
                    let _len = instance.reader.read32(false)?;
                    let info = &mut ctx.info;
                    let ltype = parse_interface(&raw)?;
                    info.link_type = ltype as u32;
                }
                "0x00000006" => {
                    let finish = instance.reader.cursor() + packet_size;
                    let _interface_id = instance.reader.read32(false)?;
                    let mut ts = instance.reader.read32(false)? as u64;
                    let low_ts = instance.reader.read32(false)? as u64;
                    let captured = instance.reader.read32(false)?;
                    let origin = instance.reader.read32(false)?;
                    ts = (ts << 32) + low_ts;
                    instance.update_ts(ts);
                    // let _mod = origin % 4;
                    // if _mod > 0 {
                    //     instance.reader._move((4 - _mod) as usize);
                    // }
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        instance.create(ts, captured, origin);
                    }));
                    instance.reader._set(finish);
                    let _len = instance.reader.read32(false)?;
                }
                _ => {
                    instance.reader.slice((len - 12) as usize);
                    let _len = instance.reader.read32(false)?;
                },
            }
        // } else {
        //     break;
        // }
        if !instance.reader.has() {
            break;
        }
    }
    instance.flush();
    // let elapsed = start.elapsed().as_millis() as usize;
    // instance.ctx.cost = elapsed;
    Ok(instance)
}
