pub mod pcap;
use std::borrow::Borrow;

// pub mod pcapng;
use crate::common::{FileInfo, FileType, Reader};

pub trait PNode {
    fn summary(&self) -> String;
    fn info(&self) -> String;
    fn children(&self) -> Vec<Box<dyn PNode>>;
}
// pub struct PLabel<'a, T> {
//     sup: &'a T,
// }
pub trait Element {
    fn get_frame(&self) -> &Frame;
}

pub trait Visitor<E, R> {
    fn visit(&self, ele: PacketContext<E>) -> PacketContext<R>;
}

pub struct FramePacket<'a> {
    pub frame: &'a Frame
}

impl <T> Element for PacketContext<'_, T> {
    fn get_frame(&self) -> &Frame {
        self.frame
    }
}

pub struct PacketContext<'a, T> {
    next: Option<Box<dyn Element>>,
    frame: &'a Frame,
    reader: &'a Reader,
    pub val: Option<T>,
    fields: Vec<Position<T>>
}

// impl <T> Element for PacketContext<'_, T> {
//     fn get_frame(&self) -> &Frame {
//         self.frame
//     }
    
//     // fn set_next(&mut self, next: Box<dyn Element>) {
//     //     self.next = Some(next);
//     // }
// }

impl <T>PacketContext<'_, T> {
    pub fn get_frame(&self) -> &Frame {
        self.frame
    }
    pub fn get_reader<'a>(&mut self) -> &Reader{
        &self.reader
    }
    pub fn read<K>(&mut self, opt: fn(&mut Reader) -> K, render: Option<fn(t: &T)->String> ) -> K {
        let start = self.get_reader().cursor;
        let mut _reader = self.reader;
        let val: K = opt(_reader);
        let end = self.get_reader().cursor;
        self.fields.push(Position{start, size: end - start, render });
        val
    }
    pub fn create_packet<K>(&mut self) -> PacketContext<K> where K: Initer<K> {
        let val = K::new();
        PacketContext{next: None, val: Some(val), frame: self.frame, reader: self.reader, fields: Vec::new()}
    }
    pub fn get_val(&self) -> &Option<T> {
        &self.val
    }
    
}
pub struct Position <T>{
    pub start: usize,
    pub size: usize,
    render: Option<fn(t: &T)->String>,
}

pub trait Initer<T> {
    fn new () -> T;
}

pub struct Frame {
    next: Option<Box<dyn Element>>,
    pub index: usize,
    pub ts: u64,
    pub capture_size: u32,
    pub origin_size: u32,
    reader: Reader,
}
impl Frame {
    pub fn get_info() -> String {
        String::from("Packet")
    }
    
    pub fn create_packet(&self) -> PacketContext<FramePacket> {
        // let reader2 = &mut self.reader;
        let f = FramePacket{frame: &self};
        // let _self = self.borrow();
        let reader = &self.reader;
        PacketContext{next: None, val: Some(f), frame: self, reader: &self.reader, fields: Vec::new()}
    }
    pub fn get_data(&mut self) -> &[u8] {
        self.reader.get_data()
    }
    
}
impl Element for Frame {
    fn get_frame(&self) -> &Frame {
        self
    }
    
    // fn set_next(&mut self, next: Box<dyn Element>) {
    //     self.next = Some(next);
    // }
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
        let mut f = Frame{next: None,reader: Reader::new(data.to_vec()), ts, capture_size, origin_size, index: self.count as usize};
        crate::specs::get_visitor(self.info.link_type).unwrap().visit(&mut f);
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