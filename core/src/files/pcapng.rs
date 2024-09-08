use crate::common::{FileType, Reader};
use anyhow::Result;
use log::info;

use super::Instance;

fn parse_head(data: &[u8]) -> Result<String> {
    let reader = Reader::new(data);
    let _magic = reader.read32(false)?;
    let major = reader.read16(false)?;
    let minor = reader.read16(false)?;
    Ok(format!("{}-{}", major, minor))
}

fn parse_interface(data: &[u8]) -> Result<u16> {
    let reader = Reader::new(data);
    reader.read16(false)
}

fn parse_enhance(ctx: &Instance, data: &[u8]) -> Result<()>{
    let reader = Reader::new(data);
    let _interface_id = reader.read32(false);
    let mut ts = reader.read32(false)? as u64;
    let low_ts = reader.read32(false)? as u64;
    let captured = reader.read32(false)?;
    let origin = reader.read32(false)?;
    ts = (ts << 32) + low_ts;
    ctx.update_ts(ts);
    let raw = reader.slice(captured as usize);
    let _mod = origin % 4;
    if _mod > 0 {
        reader._move((4 - _mod) as usize);
    }
    ctx.create(raw, ts, captured, origin);
    Ok(())
}
pub fn parse(data: &[u8]) -> Result<Instance> {
    let ctx = Instance::new(FileType::PCAPNG);
    let reader = Reader::new(data);

    loop {
        let block_type = format!("{:#010x}", reader.read32(false)?);
        let len = reader.read32(false)?;
        let raw = reader.slice((len - 12) as usize);
        let _len = reader.read32(false)?;
        let context = ctx.context();
        if len == _len {
            match block_type.as_str() {
                "0x0a0d0d0a" => {
                    let mut info = context.info.borrow_mut();
                    info.version = parse_head(&raw)?;
                }
                "0x00000001" => {
                    let mut info = context.info.borrow_mut();
                    let ltype = parse_interface(&raw)?;
                    info.link_type = ltype;
                }
                "0x00000006" => {
                    parse_enhance(&ctx, &raw)?;
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
    Ok(ctx)
}
