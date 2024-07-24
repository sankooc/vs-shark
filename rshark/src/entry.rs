use log::info;
use js_sys::Uint8Array;

use crate::files::*;

// use std::str;
use crate::files::CContext;
use std::result::Result;
use std::fmt::Error;
use crate::common::IO;

pub struct Pack<'a>{
  data: &'a [u8]
}
impl Pack<'_> {
  pub fn new(data: &[u8])-> Pack{
    Pack{data}
  }
}
// pub fn load_data2 (data: & [u8])-> {

// }
pub fn  load_data<'a> (data: &[u8] )->  Result<CContext, Error>{
  // let data: &[u8] = &Vec::new();
  let head: &[u8] = &data[..4];
  let head_str = format!("{:x}", IO::read32(head, false));
  match head_str.as_str() {
    "a1b2c3d4" => {
      return Ok(pcap::parse(&data));
    },
    // "a0d0d0a" => {
    //   let solve = pcapng::parse(data);
    //   return Ok(CContext::new(solve))
    // },
    _ => return Err(Error{})
  };
}