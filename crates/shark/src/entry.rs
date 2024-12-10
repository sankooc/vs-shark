
use crate::files::{pcap, pcapng};
use crate::common::io::{ Reader, IO};

use crate::common::base::{Configuration, Instance};
use crate::common::DataError;
use anyhow::{bail, Result};

/// Given a byte slice, determine its type and parse it into an `Instance`.
/// Supported types are PCAP and PCAPNG.
///
/// # Errors
///
/// This function will return an error of type `DataError::UnsupportFileType` if the given byte slice
/// does not correspond to a supported file type.
pub fn  load_data<'a> (data: Vec<u8>, conf: Configuration)->  Result<Instance>{
  let head: &[u8] = &data[..4];
  let head_str = format!("{:x}", IO::read32(head, false)?);
  let reader = Reader::new_raw(data);
  // let head_str = format!("{:x}", reader.read32(false)?);
  match head_str.as_str() {
    "a1b2c3d4" => {
      return pcap::parse(reader, conf)
    },
    "a0d0d0a" => {
      return pcapng::parse(reader, conf)
    },
    _ => bail!(DataError::UnsupportFileType)
  };
}