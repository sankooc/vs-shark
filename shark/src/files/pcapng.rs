use crate::common::base::Instance;
use crate::common::io::{AReader, SliceReader};
use crate::common::FileType;
use anyhow::Result;

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

fn parse_enhance(instance: &mut Instance, data: &[u8]) -> Result<()> {
    let reader = SliceReader::new(data);
    let _interface_id = reader.read32(false)?;
    let mut ts = reader.read32(false)? as u64;
    let low_ts = reader.read32(false)? as u64;
    let captured = reader.read32(false)?;
    let origin = reader.read32(false)?;
    ts = (ts << 32) + low_ts;
    instance.update_ts(ts);
    let raw = reader.slice(captured as usize);
    let _mod = origin % 4;
    if _mod > 0 {
        reader._move((4 - _mod) as usize);
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        instance.create(raw.to_vec(), ts, captured, origin);
    }));
    Ok(())
}
pub fn parse(data: &[u8]) -> Result<Instance> {
    let mut instance = Instance::new(FileType::PCAPNG);
    let reader = SliceReader::new(data);

    loop {
        let block_type = format!("{:#010x}", reader.read32(false)?);
        let len = reader.read32(false)?;
        let raw = reader.slice((len - 12) as usize);
        let _len = reader.read32(false)?;
        let ctx = &mut instance.ctx;
        if len == _len {
            match block_type.as_str() {
                "0x0a0d0d0a" => {
                    let info = &mut ctx.info;
                    info.version = parse_head(&raw)?;
                }
                "0x00000001" => {
                    let info = &mut ctx.info;
                    let ltype = parse_interface(&raw)?;
                    info.link_type = ltype as u32;
                }
                "0x00000006" => {
                    parse_enhance(&mut instance, &raw)?;
                }
                _ => (),
            }
        } else {
            break;
        }
        if !reader.has() {
            break;
        }
    }
    instance.flush();
    Ok(instance)
}
