// use super::FileSolve;
// use crate::common::FileInfo;

// struct  Pcapng;

// impl FileSolve for Pcapng {
//     fn get_info(&self) -> FileInfo {
//       FileInfo{ file_type: crate::common::FileType::PCAPNG,..Default::default() }
//     }
// }
use crate::common::{FileType, Reader};

use super::CContext;

fn parse_head(data: &[u8]) -> String {
    let reader = Reader::new(data);
    let _magic = reader.read32(false);
    let major = reader.read16(false);
    let minor = reader.read16(false);
    format!("{}-{}", major, minor)
}

fn parse_interface(data: &[u8]) -> u16 {
    let reader = Reader::new(data);
    reader.read16(false)
}

fn parse_enhance(ctx: &mut CContext, data: &[u8]) {
    let reader = Reader::new(data);
    let _interface_id = reader.read32(false);
    let mut ts = reader.read32(false) as u64;
    let low_ts = reader.read32(false) as u64;
    let captured = reader.read32(false);
    let origin = reader.read32(false);
    ts = (ts << 32) + low_ts;
    ctx.update_ts(ts);
    let raw = reader.slice(captured as usize);
    let _mod = origin % 4;
    if _mod > 0 {
        reader._move((4 - _mod) as usize);
    }
    ctx.create(raw, ts, captured, origin);
}
pub fn parse(data: &[u8]) -> CContext {
    let mut ctx = CContext::new(FileType::PCAPNG);
    let reader = Reader::new(data);

    loop {
        let block_type = format!("{:#010x}", reader.read32(false));
        let len = reader.read32(false);
        let raw = reader.slice((len - 12) as usize);
        let _len = reader.read32(false);
        if len == _len {
            match block_type.as_str() {
                "0x0a0d0d0a" => {
                    ctx.get_info().version = parse_head(&raw);
                }
                "0x00000001" => {
                    ctx.get_info().link_type = parse_interface(&raw);
                }
                "0x00000006" => {
                    parse_enhance(&mut ctx, &raw);
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
    ctx
}
