// use std::time::{SystemTime, UNIX_EPOCH};

use crate::files::{pcap, pcapng};
use crate::common::io::IO;

use crate::common::base::Instance;
use crate::common::DataError;
use anyhow::{bail, Result};

pub fn  load_data<'a> (data: &[u8] )->  Result<Instance>{
  // let start = SystemTime::now().duration_since(UNIX_EPOCH);

  let head: &[u8] = &data[..4];
  let head_str = format!("{:x}", IO::read32(head, false)?);
  match head_str.as_str() {
    "a1b2c3d4" => {
      return pcap::parse(&data)
    },
    "a0d0d0a" => {
      return pcapng::parse(&data)
    },
    _ => bail!(DataError::UnsupportFileType)
  };
}