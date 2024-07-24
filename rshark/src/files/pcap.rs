use crate::common::{FileType, Reader};
use log::*;

use super::CContext;


pub fn parse (data: &[u8]) -> CContext {
  let mut ctx = CContext::new(FileType::PCAP);
  let mut reader = Reader::new(data);
  let _magic = reader.read32(true);
  let major = reader.read16(true);
  let minor = reader.read16(true);
  ctx.get_info().version = format!("{}-{}",major, minor);
  reader._move(8);
  let _snap_len = reader.read32(false);
  reader._move(2);
  let linktype = reader.read16(true);
  ctx.get_info().link_type = linktype;
  while reader.has() {
    let h_ts:u64= reader.read32(false).into();
    let l_ts:u64 = reader.read32(false).into();
    let ts: u64 = h_ts * 1000000 + l_ts;
    ctx.update_ts(ts);
    let captured = reader.read32(false);
    let origin = reader.read32(false);
    info!("ts {}, {}, {}", ts, captured, origin);
    let raw = reader.slice(origin as usize);
    ctx.create(raw, ts, captured, origin);
  }
  ctx
}