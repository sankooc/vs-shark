use std::{fmt::Formatter, rc::Rc};

use pcap_derive::Packet;

use crate::common::{
        base::{Frame, PacketBuilder, PacketContext}, io::{AReader, Reader}, Ref2, FIELDSTATUS
    };
use anyhow::Result;

use super::ProtocolData;


pub enum  TransferEncoding {
    CHUNKED,
    COMPRESS,
    DEFLATE,
    GZIP,
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}
pub struct Response {
    pub version: String,
    pub code: String,
    pub status: String,
}


#[derive(Default)]
pub enum HttpType {
    #[default]
    NONE,
    REQUEST(Request),
    RESPONSE(Response),
}

#[derive(Default, Packet)]
pub struct HTTP {
    header: Vec<String>,
    head: String,
    _type: HttpType,
    pub content_type: Option<String>,
    pub content: Option<Rc<Vec<u8>>>,
    pub len: usize,
    pub chunked: bool,
}
impl HTTP {
    pub fn head(&self) -> String {
        self.head.clone()
    }

    pub fn header(&self) -> Vec<String> {
        self.header.clone()
    }
    pub fn _type(&self) -> &HttpType {
        &self._type
    }
    pub fn wrap(&self) {
        
    }
}
impl crate::common::base::InfoPacket for HTTP {
    fn info(&self) -> String {
        self.head.clone()
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
impl std::fmt::Display for HTTP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Hypertext Transfer Protocol")
    }
}
// #[derive(Visitor3)]
pub struct HTTPVisitor;

impl HTTPVisitor {
    pub fn check(reader: &impl AReader) -> bool {
        let method = reader._read_space(10);
        match method {
            Some(_method) => {
                return match _method.as_str() {
                    "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "CONNECT" | "OPTIONS" | "NOTIFY" | "TRACE" | "PATCH" => true,
                    "HTTP/1.1" => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

fn pick_value(head: &str, key: &str) -> Option<String> {
    let mut aa = head.split(":");
    let mut rs = None;
    if let Some(_head) = aa.next() {
        if _head.to_lowercase() == key {
            if let Some(v) = aa.next() {
                let mut vs = v.split(";");
                if let Some(value) = vs.next() {
                    rs = Some(value.trim().into());
                }
            }
        }
    }
    rs
}

pub fn parse(reader: &impl AReader) -> Result<HTTP>  {
    let mut p = HTTP{..Default::default()};
    p.head = reader.read_enter()?;
    let spl: Vec<_> = p.head.split(" ").collect();
    if spl.len() > 2 {
        let head = *spl.get(0).unwrap();
        let head2 = *spl.get(1).unwrap();
        let head3 = *spl.get(2).unwrap();
        if head == "HTTP/1.1" {
            p._type = HttpType::RESPONSE(Response {
                version: head.into(),
                code: head2.into(),
                status: head3.into(),
            });
        } else {
            p._type = HttpType::REQUEST(Request {
                method: head.into(),
                path: head2.into(),
                version: head3.into(),
            })
        }
    }

    loop {
        if reader.left()? == 0 {
            return Ok(p);
        }
        if reader.enter_flag(0) {
            reader._move(2);
            return Ok(p);
        }
        let header = reader.read_enter()?;
        if let Some(ch) = pick_value(&header, "transfer-encoding")  {
            p.chunked = ch == "chunked";
        }
        if let Some(ch) = pick_value(&header, "content-length")  {
            p.len = ch.parse::<usize>()?;
        }
        p.header.push(header);
    }
}

pub fn no_content(_http: Ref2<HTTP>) ->Result<ProtocolData> {
    let packet: PacketContext<HTTP> = Frame::_create_packet(_http.clone());
    let p = packet.get().borrow();
    packet.build_txt(p.head.clone());
    for head in &p.header {
        packet.build_txt(head.clone());
    }
    drop(p);
    Ok(super::ProtocolData::HTTP(packet))
}

pub fn content_len(_http: Ref2<HTTP>, body: Vec<u8>) ->Result<ProtocolData> {
    let rs = no_content(_http.clone())?;
    if let ProtocolData::HTTP(packet) = &rs {
        let len = body.len();
        let content = Rc::new(body);
        let reader = Reader::new_raw(content.clone());
        packet._build(&reader, 0, len, format!("File Data: {} bytes", len));
        let mut reff = _http.as_ref().borrow_mut();
        reff.content = Some(content);
        drop(reff);
    }
    Ok(rs)
}