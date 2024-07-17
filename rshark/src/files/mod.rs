pub mod pcap;
// pub mod pcapng;
use crate::common::{FileInfo, FileType, Reader};

pub trait PNode {
    fn summary(&self) -> String;
    fn info(&self) -> String;
    fn children(&self) -> Vec<Box<dyn PNode>>;
}
pub struct PLabel<'a, T> {
    sup: &'a T,
}
pub trait Element {
    fn get_frame(&self) -> &Frame;
}

pub trait Visitor {
    fn visit(&self, ele: &impl Element);
}
// pub struct EtherVisitor;

// impl Visitor for EtherVisitor {
//     fn visit(ele: impl Element) {}
// }

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

pub trait Packet<'a> {
    fn get_protocol();
    fn get_upper();
    fn get_frame() -> &'a Frame;
}
pub struct PacketContext<'a, T> {
    frame: &'a Frame,
    reader: Reader<'a>,
    fields: Vec<Position<T>>
}

impl <T>PacketContext<'_, T> {
    pub fn get_reader<'a>(&mut self) -> &Reader{
        &self.reader
    }
    pub fn read<K>(&mut self, opt: fn(&mut Reader) -> K, render: Option<fn(t: &T)->String> ) -> K {
        let start = self.get_reader().cursor;
        let _reader = &mut self.reader;
        let val: K = opt(_reader);
        let end = self.get_reader().cursor;
        self.fields.push(Position{start, size: end - start, render });
        val
    }
}
pub struct Position <T>{
    pub start: usize,
    pub size: usize,
    render: Option<fn(t: &T)->String>,
}

// pub trait Initer {
//     fn new <T> (packet: PacketContext<T>) -> T;
// }

pub struct Frame {
    pub index: usize,
    pub ts: u64,
    pub capture_size: u32,
    pub origin_size: u32,
    data: Vec<u8>,
}
impl Frame {
    pub fn get_info() -> String {
        String::from("Packet")
    }
    pub fn create_packet<T>(&self) -> PacketContext<T>  {
        // let p: PacketContext<T> = PacketContext{frame: self, reader: Reader::new(&self.data), fields: Vec::new()};
        // T::new(p)
        PacketContext{frame: self, reader: Reader::new(&self.data), fields: Vec::new()}
    }
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    
}
impl Element for Frame {
    fn get_frame(&self) -> &Frame {
        self
    }
}
pub struct CContext {
    count: u32,
    info: FileInfo,
    frames: Vec<Frame>,
}
impl<'a> CContext {
    pub fn new(ftype: FileType) -> CContext {
        CContext {
            count: 0,
            info: FileInfo {
                file_type: ftype,
                ..Default::default()
            },
            frames: Vec::new(),
        }
    }
    pub fn create(&mut self, data: &[u8], ts: u64, capture_size: u32, origin_size: u32) {
        self.count += 1;
        let f = Frame{data: data.to_vec(), ts, capture_size, origin_size, index: self.count as usize};
        crate::specs::get_visitor(self.info.link_type).unwrap().visit(&f);
        self.frames.push(f);
        // &(self.frames.last().unwrap())
    }
    pub fn get_info(&mut self) -> &mut FileInfo {
        &mut self.info
    }
    pub fn update_ts(&mut self, ts: u64) {
        if self.get_info().start_time > 0 {
            return;
        }
        let info: &mut FileInfo = self.get_info();
        info.start_time = ts;
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
