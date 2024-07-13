pub mod pcap;

use crate::common::{Context, FileInfo};
pub fn basic() {}

pub trait FileSolve {
    fn get_info(&self) -> FileInfo;
}

pub struct CContext {
    pub solve: Box<dyn FileSolve>,
}

impl CContext {
    fn new(solve: impl FileSolve + 'static) -> CContext {
        Self { solve: Box::new(solve) }
    }
}

impl Context for CContext {
    fn get_file_type(&self) -> FileInfo {
      self.solve.get_info().clone()
    }
}
