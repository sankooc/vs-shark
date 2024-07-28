pub mod pcap;

use std::cell::{Cell, Ref, RefCell};

// pub mod pcapng;
use crate::common::{FileInfo, FileType, Protocol, Reader};

pub trait PNode {
    fn summary(&self) -> String;
    fn info(&self) -> String;
    fn children(&self) -> Vec<Box<dyn PNode>>;
}
#[derive(Default, Clone)]
pub struct Field {
    pub start: usize,
    pub size: usize,
    pub summary: String,
    pub children: RefCell<Vec<Field>>,
}

impl Field {
    pub fn new(start:usize, size: usize, summary: String) -> Field {
        Field {
            start,
            size,
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
}

pub trait Element {
    fn summary(&self) -> String;
    fn get_fields(&self) -> Vec<Field>;
    // fn add_next(&mut self, ele: Box<dyn Element>);
    fn get_protocol(&self) -> Protocol;
}

pub trait Visitor {
    fn visit(&self, frame: &Frame, reader: &Reader);
}

pub struct FramePacket {
    // pub frame: &'a Frame
}

impl Initer<FramePacket> for FramePacket {
    fn new() -> FramePacket {
        FramePacket {}
    }
    fn get_protocol(&self) -> Protocol {
        Protocol::UNKNOWN
    }
}

pub struct PacketContext<T>
where
    T: Initer<T>,
{
    pub next: Option<Box<dyn Element>>,
    pub val: T,
    fields: Vec<Position<T>>,
}

impl<T> Element for PacketContext<T>
where
    T: Initer<T> + ToString,
{
    fn summary(&self) -> String {
        self.val.to_string().clone()
    }
    fn get_fields(&self) -> Vec<Field>{
        let t = &self.val;
        let mut rs: Vec<Field> = Vec::new();
        for pos in self.fields.iter() {
            match pos.render {
                Some(_render) => {
                    rs.push(Field::new(pos.start, pos.size, _render(t)));
                },
                _ => (),
            }
        }
        rs
    }
    fn get_protocol(&self) -> Protocol {
        self.val.get_protocol()
    }
    // fn get_frame(&mut self)-> &Frame{
    //     return self.frame
    // }
    // fn add_next(&mut self, ele: Box<dyn Element>) {
    //     self.next = Some(ele)
    // }
}
impl<T> PacketContext<T>
where
    T: Initer<T>,
{
    pub fn read<K>(
        &mut self,
        reader: &Reader,
        opt: fn(&Reader) -> K,
        render: Option<fn(t: &T) -> String>,
    ) -> K {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        self.fields.push(Position {
            start,
            size: end - start,
            render,
        });
        val
    }
    pub fn iter(&self) {
        let v = &self.val;
        for pos in &self.fields {
            let str = pos.render.unwrap()(v);
        }
    }
    pub fn get_val(&self) -> &T {
        &self.val
    }
}
pub struct Position<T> {
    pub start: usize,
    pub size: usize,
    render: Option<fn(t: &T) -> String>,
}

pub trait Initer<T> {
    fn new() -> T;
    fn get_protocol(&self) -> Protocol;
}
#[derive(Default, Clone)]
pub struct FrameSummary {
    pub index: u32,
    pub source: String,
    pub target: String,
    pub protocol: Protocol,
}

pub struct Frame {
    pub ts: u64,
    pub capture_size: u32,
    pub origin_size: u32,
    pub summary: RefCell<FrameSummary>,
    pub data: Vec<u8>,
    pub eles: RefCell<Vec<Box<dyn Element>>>,
}
impl Frame {
    pub fn new(data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32, index: u32) -> Frame {
        Frame {
            eles: RefCell::new(Vec::new()),
            summary: RefCell::new(FrameSummary {
                index,
                ..Default::default()
            }),
            data,
            ts,
            capture_size,
            origin_size,
        }
    }
    pub fn get_info() -> String {
        String::from("Packet")
    }

    pub fn get_reader(&self) -> Reader {
        return Reader::new(&self.data);
    }
    pub fn create_packet<K>() -> PacketContext<K>
    where
        K: Initer<K>,
    {
        let val = K::new();
        PacketContext {
            next: None,
            val,
            fields: Vec::new(),
        }
    }
    pub fn add_element(&self, ele: Box<dyn Element>) {
        self.eles.borrow_mut().push(ele)
    }
}

pub struct CContext {
    count: u32,
    info: FileInfo,
    frames: RefCell<Vec<Frame>>,
}
impl<'a> CContext {
    pub fn new(ftype: FileType) -> CContext {
        CContext {
            count: 0,
            info: FileInfo {
                file_type: ftype,
                ..Default::default()
            },
            frames: RefCell::new(Vec::new()),
        }
    }
    pub fn create(&mut self, data: &[u8], ts: u64, capture_size: u32, origin_size: u32) {
        self.count += 1;
        let f = Frame::new(data.to_vec(), ts, capture_size, origin_size, self.count);
        let reader = f.get_reader();
        match crate::specs::get_visitor(self.info.link_type) {
            Some(visitor) => visitor.visit(&f, &reader),
            None => (),
        };
        self.frames.borrow_mut().push(f);
    }
    pub fn get_info(&mut self) -> &mut FileInfo {
        &mut self.info
    }
    pub fn get_frames(&self) -> Ref<Vec<Frame>> {
        self.frames.borrow()
    }
    pub fn update_ts(&mut self, ts: u64) {
        if self.get_info().start_time > 0 {
            return;
        }
        let info: &mut FileInfo = self.get_info();
        info.start_time = ts;
    }
}
