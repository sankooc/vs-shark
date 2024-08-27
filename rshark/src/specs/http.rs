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
    _type: HttpType,
}
impl crate::files::InfoPacket for HTTP {
    fn info(&self) -> String {
        self.to_string()
    }
}
impl std::fmt::Display for HTTP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        // if self.operation == 1 {
        //     if self.source_ip_address() == self.target_ip_address(){
        //         fmt.write_fmt(format_args!("HTTP Announcement for {}", self.source_ip_address()))
        //     } else {
        //         fmt.write_fmt(format_args!("who has {}? tell {}", self.target_ip_address(), self.source_ip_address()))
        //     }
        // } else {
        //     fmt.write_fmt(format_args!("{} at {}", self.target_ip_address(), self.source_ip_address()))
        // }
        Ok(())
    }
}
impl HTTP {
    // fn _info(&self) -> String {
    //     self.to_string()
    // }
    // fn _summary(&self) -> String {
    //     self.to_string()
    // }
    // fn protocol_type_desc(&self) -> String {
    //     format!("Protocol type: {} ({})", etype_mapper(self.protocol_type),self.protocol_type)
    // }
    // fn hardware_type_desc(&self) -> String {
    //     format!("Hardware type: {} ({})", self._hardware_type(),self.hardware_type)
    // }
    // fn operation_type_desc(&self) -> String {
    //     format!("Opcode: {} ({})", self._operation_type(), self.operation)
    // }
    // fn _hardware_type(&self) -> String {
    //     arp_hardware_type_mapper(self.hardware_type)
    // }

    // fn _operation_type(&self) -> String {
    //     arp_oper_type_mapper(self.operation)
    // }
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
        let v = packet._read_with_format_string_rs(reader, Reader::_read_enter, "{}")?;
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
            let header = packet._read_with_format_string_rs(reader, Reader::_read_enter, "{}")?;
            p.header.push(header);
        }
        let dlen = reader.left()?;
        packet.read_txt(reader, reader.cursor(), dlen, format!("File Data: {} bytes",dlen));
        drop(p);
        frame.add_element(super::ProtocolData::HTTP(packet));
        Ok(())
    }
}
