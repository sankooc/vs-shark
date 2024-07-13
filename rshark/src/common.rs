use std::marker::Copy;
pub trait Context {
  fn get_file_type(&self) -> FileInfo;

}

#[derive(Copy, Clone)]
pub struct FileInfo<'a> {
  link_type: &'a str,
  file_type: FileType,
  start_time: u64,
}

enum IpAddrKind {
  V4,
  V6,
}
#[derive(Copy, Clone)]
pub enum FileType {
  PCAP,
  PCAPNG,
}
// impl Context<'a> {
    
// }