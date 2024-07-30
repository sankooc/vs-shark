pub mod pcap;
pub mod pcapng;

use std::{cell::{Ref, RefCell}, time::{Duration, UNIX_EPOCH}};
use crate::constants::link_type_mapper;

use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::*;

// pub mod pcapng;
use crate::common::{FileInfo, FileType, Protocol, Reader};

#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Field {
    pub start: usize,
    pub size: usize,
    summary: String,
    children: RefCell<Vec<Field>>,
}
#[wasm_bindgen]
impl Field {
    pub fn new(start: usize, size: usize, summary: String) -> Field {
        Field {
            start,
            size,
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
    pub fn new2(summary: String, vs: Vec<Field>) -> Field {
        Field {
            start: 0,
            size: 0,
            summary,
            children: RefCell::new(vs),
        }
    }
    pub fn new3(summary: String) -> Field {
        Field {
            start: 0,
            size: 0,
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn children(&self) -> Vec<Field> {
        let ch: Ref<Vec<Field>> = self.children.borrow();
        let mut children = Vec::new();
        for c in ch.iter() {
            children.push(c.clone());
        }
        children
    }
}



pub fn date_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    // let dt: DateTime<Utc> = d.clone().into();
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub trait Element {
    fn summary(&self) -> String;
    fn get_fields(&self) -> Vec<Field>;
    // fn add_next(&mut self, ele: Box<dyn Element>);
    fn get_protocol(&self) -> Protocol;
    fn info(&self) -> String;
}

// impl ToString for dyn Element{
//     fn to_string(&self) -> String {
//         self.to_string()
//     }
// }

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
    
    fn info(&self) -> String {
        "".into()
    }
}

pub struct PacketContext<T>
where
    T: Initer<T>,
{
    // pub next: Option<Box<dyn Element>>,
    pub val: T,
    fields: Vec<Position<T>>,
    // pos: RefCell<Position<T>>
}

impl<T> Element for PacketContext<T>
where
    T: Initer<T> + ToString,
{
    fn summary(&self) -> String {
        self.val.to_string().clone()
    }
    fn get_fields(&self) -> Vec<Field> {
        let t = &self.val;
        let mut rs: Vec<Field> = Vec::new();
        for pos in self.fields.iter() {
            rs.push((pos.render)(pos.start, pos.size, t));
        }
        rs
    }
    fn get_protocol(&self) -> Protocol {
        self.val.get_protocol()
    }
    
    fn info(&self) -> String {
        self.val.info()
    }
}
impl<T> PacketContext<T>
where
    T: Initer<T>,
{
    pub fn read<K>(
        &mut self,
        reader: &Reader,
        opt: fn(&Reader) -> K,
        render: Option<fn(usize, usize, &T) -> Field>,
    ) -> K {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        self.add_pos(start, end - start, render);
        val
    }
    pub fn read_empty(&mut self,reader: &Reader, size: usize, render: Option<fn(usize, usize, &T) -> Field>) {
        let start = reader.cursor();
        self.add_pos(start, size, render);
        // self.fields.push(Position {start, size, render});
    }

    fn add_pos(&mut self, start:usize, size: usize, _render: Option<fn(usize, usize, &T) -> Field>){
        match _render {
            Some(render) => self.fields.push(Position {start, size, render}),
            _ => (),
        }
    }
    // pub fn iter(&self) {
    //     let v = &self.val;
    //     // for pos in &self.fields {
    //     //     let str = pos.render.unwrap()(v);
    //     // }
    // }
    // pub fn get_val(&self) -> &T {
    //     &self.val
    // }
}

pub struct Position<T> {
    pub start: usize,
    pub size: usize,
    pub render: fn(usize, usize, &T) -> Field,
}

pub trait Initer<T> {
    fn new() -> T;
    fn get_protocol(&self) -> Protocol;
    fn info(&self) -> String;
}
#[derive(Default, Clone)]
pub struct FrameSummary {
    pub index: u32,
    pub source: String,
    pub target: String,
    pub protocol: Protocol,
    pub link_type: u16,
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
    pub fn new(data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32, index: u32, link_type: u16) -> Frame {
        let f = Frame {
            eles: RefCell::new(Vec::new()),
            summary: RefCell::new(FrameSummary {
                index,
                link_type,
                ..Default::default()
            }),
            data,
            ts,
            capture_size,
            origin_size,
        };
        // let mut pkg:PacketContext<FramePacket> = Frame::create_packet();
        // // let render = Some(|start, size, val| Field::new(start,size,"".into()));
        // pkg.fields.push(Position {start: 0, size:0, render:Some(|start, size, val| Field::new(start,size,"".into()))});
        f
    }
    pub fn to_string(&self) -> String {
        format!("Frame {}: {} bytes on wire ({} bits), {} bytes captured ({} bits)", self.summary.borrow().index, self.origin_size, self.origin_size * 8, self.capture_size, self.capture_size * 8)
    }

    pub fn info(&self) -> String {
        for e in self.eles.borrow().iter() {
            return e.as_ref().info();
        }
        self.to_string()
    }
    
    pub fn get_fields(&self) -> Vec<Field> {
        let mut rs = Vec::new();
        let mut lists = Vec::new();
        let ltype = self.summary.borrow().link_type;
        lists.push(Field::new3(format!("Encapsulation type: {} ({})", link_type_mapper(ltype), ltype)));
        lists.push(Field::new3(format!("UTC Arrival Time: {} UTC", date_str(self.ts))));
        lists.push(Field::new3(format!("Frame Number: {}", self.summary.borrow().index)));
        lists.push(Field::new3(format!("Frame Length: {} bytes ({} bits)", self.origin_size, self.origin_size * 8)));
        lists.push(Field::new3(format!("Capture Length: {} bytes ({} bits)", self.capture_size, self.capture_size * 8)));
        
        rs.push(Field::new2(self.to_string(), lists));
        for e in self.eles.borrow().iter() {
            let vs = e.get_fields();
            rs.push(Field::new2(e.summary(), vs));
        }
        rs
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
            // pos: RefCell::new(Position{start:0,size:0, render:None}),
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
        let f = Frame::new(data.to_vec(), ts, capture_size, origin_size, self.count, self.info.link_type);
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
