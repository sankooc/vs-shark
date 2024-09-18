#[cfg(test)]
mod unit {
    use super::*;
    use core::common::io::AReader;
    use core::{common::{io::Reader, FileType}, files::{Frame, Instance, Visitor}, specs::{self, ProtocolData}};
    use std::{fs, rc::Rc, str::from_utf8};
    fn build_reader(name: &str) -> Vec<u8> {
      let fname = format!("./tests/bin/{}.in", name);
      let data: Vec<u8> = fs::read(&fname).expect("no_file");
      let str = from_utf8(&data).expect("parse_failed");
      let mut rs = Vec::new();
      for i in 0..(str.len()/2) {
        let _str = format!("{}", &str[(i*2)..(i*2 + 2)]);
        let val = u8::from_str_radix(&_str, 16).unwrap();
        rs.push(val);
      }
      rs
    }
    

    fn mock_frame() -> Frame {
      let instance = Instance::new(FileType::PCAPNG);
      Frame::new(instance.context(), Vec::new(), 12, 10000, 10000, 1, 1)
    }
    #[test]
    fn test_ethernet() {
      let frame = mock_frame();
      let data: Vec<u8> = build_reader("ethernet");
      let reader = Reader::new_raw(Rc::new(data));
      let (prop, next) = specs::ethernet::ii::EthernetVisitor.visit(&frame, &reader).unwrap();
      
      assert_eq!(next, "ipv4");
      match &prop {
        ProtocolData::ETHERNET(el) => {
          let val = el.get().borrow();
          assert_eq!(0x0800, val.ptype);
        },
        _ => {
          assert!(false);
        }
      }
    }
    #[test]
    fn test_certificate() {
      let data: Vec<u8> = build_reader("certificate");
      let reader = Reader::new_raw(Rc::new(data));
      let ins = specs::tls::handshake::Certificate::create(&reader, None).unwrap();
      println!("--");
    }

    #[test]
    fn test_certificates() {
      let data: Vec<u8> = build_reader("certificates");
      let reader = Reader::new_raw(Rc::new(data));
      let ins = specs::tls::handshake::Certificates::create(&reader, None).unwrap();
      println!("--");
    }

}
