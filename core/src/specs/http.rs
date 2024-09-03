use std::fmt::Formatter;

use pcap_derive::Packet;

use crate::{
    common::Reader,
    constants::{arp_hardware_type_mapper, arp_oper_type_mapper, etype_mapper},
    files::{Frame, Initer, PacketContext},
};
use anyhow::Result;

struct Request {
    method: String,
    path: String,
    version: String,
}
struct Response {
    version: String,
    code: String,
    status: String,
}
#[derive(Default)]
enum HttpType {
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
}
impl crate::files::InfoPacket for HTTP {
    fn info(&self) -> String {
        self.head.clone()
    }
}
impl std::fmt::Display for HTTP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Hypertext Transfer Protocol")
    }
}
pub struct HTTPVisitor;

impl HTTPVisitor {
    pub fn check(reader: &Reader) -> bool {
        let method = reader._read_space(10);
        match method {
            Some(_method) => {
                return match _method.as_str() {
                    "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "CONNECT" | "OPTIONS"
                    | "TRACE" | "PATCH" => true,
                    "HTTP/1.1" => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl crate::files::Visitor for HTTPVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<HTTP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let v = packet.build_format(reader, Reader::_read_enter, "{}")?;
        p.head = v.clone();
        let spl: Vec<_> = v.split(" ").collect();
        if spl.len() == 3 {
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
                break;
            }
            let header = packet.build_format(reader, Reader::_read_enter, "{}")?;
            p.header.push(header);
        }
        let dlen = reader.left()?;
        packet._build(reader, reader.cursor(), dlen, format!("File Data: {} bytes",dlen));
        drop(p);
        frame.add_element(super::ProtocolData::HTTP(packet));
        Ok(())
    }
}
