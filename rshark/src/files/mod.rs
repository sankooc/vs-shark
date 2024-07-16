pub mod pcap;
pub mod pcapng;
use crate::common::{FileInfo, FileType};


pub trait PNode {
    fn summary(&self) -> String;
    fn info(&self) -> String;
    fn children(&self) -> Vec<Box<dyn PNode>>;
}

pub trait Element {
    fn accept(v: impl Visitor);
}

pub trait  Visitor {
    fn visit(ele: impl Element);
}
pub struct EtherVisitor;

impl Visitor for EtherVisitor {
    fn visit(ele: impl Element) {
        
    }
}

pub struct SSLVisitor;


pub struct EthernetPacket {
    data: Option<Vec<u8>>,
    pub source: String,
    pub target: String,
    pub ptype: String,
}

pub struct SSLPacket {
    data: Option<Vec<u8>>,
    pub target: String,
}

pub struct Frame {
    // protocol_bit: u64,
    index: usize,
    ts: u64,
    capture_size: u32,
    origin_size: u32,
    data: Vec<u8>,
    // children: Option<Box<Frame>>,
}
impl Frame {
    fn get_info()-> String{
        String::from("Packet")
    }
}
pub struct CContext {
    count: u32,
    info: FileInfo,
    frames: Vec<Frame>,
}
impl CContext {
    pub fn new(ftype: FileType) -> CContext {
        CContext {
            count: 0,
            info: FileInfo {
                file_type: ftype,
                ..Default::default()
            },
            frames: Vec::new()
        }
    }
    pub fn create(&mut self, data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32) -> &Frame{
        self.count += 1;
        (&mut self.frames).push(Frame{data, ts, capture_size, origin_size, index: self.count as usize});
        &(&mut self.frames).last().unwrap()
    }
    pub fn getInfo(&mut self) -> &mut FileInfo {
        &mut self.info
    }
    pub fn update_ts(&mut self, ts: u64) {
        if self.getInfo().start_time > 0 {
            return;
        }
        self.getInfo().start_time = ts;
    }
}

// impl<'a> CContext<'a> {
//     pub fn new (solve: impl FileSolve+ 'a ) -> CContext<'a> {
//         Self { solve: Box::new(solve) }
//     }
// }

// impl<'a> Context for CContext<'a> {
//     fn get_file_type(&self) -> FileInfo {
//       self.solve.get_info().clone()
//     }
// }
