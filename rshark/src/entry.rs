use crate::files::*;

use crate::files::CContext;
use std::result::Result;
use std::fmt::Error;
use crate::common::IO;

pub fn  load_data<'a> (data: &[u8] )->  Result<CContext, Error>{
  let head: &[u8] = &data[..4];
  let head_str = format!("{:x}", IO::read32(head, false));
  match head_str.as_str() {
    "a1b2c3d4" => {
      return Ok(pcap::parse(&data));
    },
    "a0d0d0a" => {
      return Ok(pcapng::parse(&data))
    },
    _ => return Err(Error{})
  };
}