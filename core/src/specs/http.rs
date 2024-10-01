use std::fmt::Formatter;

use pcap_derive::{Packet, Visitor3};

use crate::{
    common::{
        io::{AReader, Reader},
        FIELDSTATUS,
    },
    common::base::{Frame, PacketBuilder, PacketContext},
};
use anyhow::Result;

use super::ProtocolData;

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
    pub content: Vec<u8>,
    pub len: usize,
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
#[derive(Visitor3)]
pub struct HTTPVisitor;

impl HTTPVisitor {
    pub fn check(reader: &Reader) -> bool {
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
    match aa.next() {
        Some(_head) => {
            if _head.to_lowercase() == key {
                let val = aa.next();
                match val {
                    Some(v) => {
                        let mut vs = v.split(";");
                        match vs.next() {
                            Some(value) => {
                                rs = Some(value.trim().into());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    rs
}
impl HTTPVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<HTTP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let v = packet.build_format(reader, Reader::_read_enter, "{}")?;
        p.head = v.clone();
        let spl: Vec<_> = v.split(" ").collect();
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
                break;
            }
            if reader.enter_flag(0) {
                reader._move(2);
                break;
            }
            let header = packet.build_format(reader, Reader::_read_enter, "{}")?;
            match pick_value(&header, "content-type") {
                Some(tp) => {
                    p.content_type = Some(tp);
                }
                _ => {}
            }
            p.header.push(header);
        }
        let dlen = reader.left()?;
        p.len =dlen;
        packet._build(reader, reader.cursor(), dlen, format!("File Data: {} bytes", dlen));
        p.content = reader.slice(dlen).to_vec();
        drop(p);
        Ok((super::ProtocolData::HTTP(packet), "none"))
    }
}
