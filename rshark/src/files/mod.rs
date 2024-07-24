pub mod pcap;

// pub mod pcapng;
use crate::common::{FileInfo, FileType, Reader};

pub trait PNode {
    fn summary(&self) -> String;
    fn info(&self) -> String;
    fn children(&self) -> Vec<Box<dyn PNode>>;
}
pub struct Field {
    pub start: usize,
    pub size: usize,
    pub summary: String,
}

pub trait Element {
    fn get_fields(&self);
    fn get_frame(&self) -> &Frame;
    // fn create_packet<K>(&self) -> PacketContext<K> where K: Initer<K>;
    fn add_next(&mut self, ele: Box<dyn Element>);
}

pub trait Visitor {
    fn visit(&self, ele: &dyn Element, reader: &mut Reader);
}

pub struct FramePacket{
    // pub frame: &'a Frame
}

impl Initer<FramePacket> for FramePacket {
    fn new() -> FramePacket {
        FramePacket{}
    }
}

pub struct PacketContext<'a, T> {
    pub next: Option<Box<dyn Element>>,
    pub val: T,
    fields: Vec<Position<T>>,
    pub frame: &'a Frame,
}


impl <T>Element for PacketContext<'_, T>  {
    fn get_fields(&self) {
        todo!()
    }
    fn get_frame(&self)-> &Frame{
        return self.frame
    }
    fn add_next(&mut self, ele: Box<dyn Element>) {
        self.next = Some(ele)
    }
}
impl <T>PacketContext<'_, T> {
    pub fn read<K>(&mut self, reader: &mut Reader, opt: fn(&mut Reader) -> K, render: Option<fn(t: &T)->String> ) -> K {
        let start = reader.cursor;
        let val: K = opt(reader);
        let end = reader.cursor;
        self.fields.push(Position{start, size: end - start, render });
        val
    }
    // pub fn create_packet<K>(&self) -> PacketContext<K> where K: Initer<K> {
    //     let val = K::new();
    //     PacketContext{next: None, frame: self.frame, val: Some(val), fields: Vec::new()}
    // }
    pub fn iter(&self){
        let v = &self.val;
        for pos in &self.fields {
            let str = pos.render.unwrap()(v);
        }
    }
    pub fn get_val(&self) -> &T {
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
    pub data: Vec<u8>
}
impl Frame {
    pub fn get_info() -> String {
        String::from("Packet")
    }

    pub fn get_reader(&self) -> Reader {
        return Reader::new(&self.data)
    }
    pub fn create_packet<K>(&self) -> PacketContext<K> where K: Initer<K> {
        let val = K::new();
        PacketContext{next: None, frame: self, val, fields: Vec::new()}
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
        let f = Frame{next: None,data:data.to_vec(), ts, capture_size, origin_size, index: self.count as usize};
        let packet:PacketContext<FramePacket> = f.create_packet();
        let mut reader = f.get_reader();
        match crate::specs::get_visitor(self.info.link_type) {
            Some(visitor) => visitor.visit(&packet, &mut reader),
            None =>(),
        }
        // crate::specs::get_visitor(self.info.link_type).unwrap().visit(&packet, &mut reader);
        // self.frames.push(f);
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