use crate::files;

use std::str;
use crate::common::*;
use std::result::Result;
// use byteorder::{BigEndian, ReadBytesExt}


pub fn load_data(data: &[u8])-> Result<(), std::io::Error>{
  // data[0];
  let head: &[u8] = &data[..4];
  let head_str = format!("{:x}", u32::from_be_bytes(head.try_into().unwrap()));
  files::pcap::parse();
  // match head_str {
  //   "a1b2c3d4" => Ok(FileType::PCAP),
  //   "a0d0d0a" => Ok(FileType::PCAPNG),
  //   _ => Err(())
  // }
  Ok(())
}