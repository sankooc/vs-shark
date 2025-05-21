use crate::common::{
    core::Context, io::{Reader, IO}, Frame
};
use anyhow::{bail, Result};

pub struct PCAPNG {}

fn parse_interface(data: &[u8]) -> Result<u16> {
    // let reader = SliceReader::new(data);
    let head: &[u8] = &data[..2];
    let lt = IO::read16(head, false)?;
    // let lt = reader.read16(f)?;
    // let _revert = reader.read16(false)?;
    // let _snap_len = reader.read32(true)?;
    Ok(lt)
}

#[allow(dead_code)]
fn _parse_head(_data: &[u8]) {
    // let _magic = reader.read32(false)?;
    // let major = reader.read16(false)?;
    // let minor = reader.read16(false)?;
    // Ok(format!("{}-{}", major, minor))
}

impl PCAPNG {
    pub fn next(ctx: &mut Context, reader: &mut Reader) -> Result<(usize, Option<Frame>)> {
        let block_type = format!("{:#010x}", reader.read32(false)?);
        let len = reader.read32(false)?;
        let packet_size = len as usize - 12;
        //
        if reader.left() < packet_size {
            reader.back(8);
            bail!("end");
        }
        match block_type.as_str() {
            "0x0a0d0d0a" => {
                let _raw = reader.slice(packet_size, true)?;
                let _len = reader.read32(false)?;
                // info.version = parse_head(&raw)?;
                // parse_head(raw.to);

                Ok((reader.cursor, None))
            }
            "0x00000001" => {
                let raw = reader.slice(packet_size, true)?.to_vec();
                let _len = reader.read32(false)?;
                // let info = &mut ctx.info;
                let ltype = parse_interface(&raw)?;
                ctx.link_type = ltype as u32;
                Ok((reader.cursor, None))
                // info.link_type = ltype as u32;
            }
            "0x00000006" => {
                let finish = reader.cursor + packet_size;
                let _interface_id = reader.read32(false)?;
                // let t = reader.slice(8, true)?.to_vec();
                
                let mut ts = reader.read32(false)? as u64;
                let low_ts = reader.read32(false)? as u64;
                ts = (ts << 32) + low_ts;


                let _captured = reader.read32(false)?;
                let origin = reader.read32(false)?;
                let mut f = Frame::new();
                f.info.len = origin;
                f.info.time = ts;
                let end = reader.cursor + origin as usize;
                f.range = Some(reader.cursor..end);
                Ok((finish + 4, Some(f)))
            }
            _ => {
                reader.slice((len - 12) as usize, true)?;
                let _len = reader.read32(false)?;
                Ok((reader.cursor, None))
            }
        }
    }
}
