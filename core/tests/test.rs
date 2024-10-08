#[cfg(test)]
mod unit {
    use log::info;

    use core::{common::io::Reader, common::base::{Field, PacketContext}, specs::{self, ProtocolData}};
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
    
    fn _dis(inx: usize, field: &Field) {
        //assert_eq!("hello       ", format!("{:width$}", "hello", width=12));
        info!("{:pad$}- {}", "", field.summary(), pad = inx);
        let fields = field.children();
        for f in fields.iter() {
            _dis(inx + 1, f);
        }
    } 
    fn inspect<T>(packet: &PacketContext<T>){
      let field = packet.get_fields();
      for f in field.iter() {
        _dis(1, f);
      }
    }

    // fn mock_frame() -> Frame {
    //   // let instance = Instance::new(FileType::PCAPNG);
    //   Frame::new(Vec::new(), 12, 10000, 10000, 1, 1)
    // }
    #[test]
    fn test_ethernet() {
      // let frame = mock_frame();
      let data: Vec<u8> = build_reader("ethernet");
      let reader = Reader::new_raw(Rc::new(data));
      let (prop, next) = specs::ethernet::ii::EthernetVisitor.visit2( &reader).unwrap();
       
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
      env_logger::builder().is_test(true).try_init().unwrap();
      let data: Vec<u8> = build_reader("certificate");
      let reader = Reader::new_raw(Rc::new(data));
      let ins = specs::tls::handshake::Certificate::create(&reader, Some(1425)).unwrap();
      inspect(&ins);
      println!("finish");
    }

    #[test]
    fn test_certificates() {
      // let data: Vec<u8> = build_reader("certificates");
      // let reader = Reader::new_raw(Rc::new(data));
      // let ins = specs::tls::handshake::Certificates::create(&reader, Some(1425)).unwrap();
      // println!("--");
    }

}
