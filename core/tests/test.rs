#[cfg(test)]
mod unit {
    //https://wiki.wireshark.org/samplecaptures
    //https://github.com/chrissanders/packets/tree/master
    use core::{
        common::{base::{PacketContext, Visitor}, concept::Field, io::Reader},
        specs::{self, ProtocolData},
    };
    use std::{fs, rc::Rc, str::from_utf8};
    fn build_reader(name: &str) -> Vec<u8> {
        let fname = format!("./tests/bin/{}.in", name);
        let data: Vec<u8> = fs::read(&fname).expect("no_file");
        let str = from_utf8(&data).expect("parse_failed");
        let mut rs = Vec::new();
        for i in 0..(str.len() / 2) {
            let _str = format!("{}", &str[(i * 2)..(i * 2 + 2)]);
            let val = u8::from_str_radix(&_str, 16).unwrap();
            rs.push(val);
        }
        rs
    }

    fn _dis(inx: usize, field: &Field) {
        //assert_eq!("hello       ", format!("{:width$}", "hello", width=12));
        println!("{:pad$}- {}", "", field.summary(), pad = inx);
        let fields = field.children();
        for f in fields.iter() {
            _dis(inx + 1, f);
        }
    }
    fn inspect<T>(packet: &PacketContext<T>) {
        let field = packet.get_fields();
        for f in field.iter() {
            _dis(1, f);
        }
    }
    #[test]
    fn test_ppp_lcp() {
        // let frame = mock_frame();
        let data: Vec<u8> = build_reader("ppp.lcp");
        let reader = Reader::new_raw(Rc::new(data));
        let (prop, _) = specs::ethernet::pppoes::PPPoESSVisitor.visit2(&reader).unwrap();
        match &prop {
            ProtocolData::PPPoESS(el) => {
                // let val = el.get().borrow();
                inspect(el); 
            } 
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_ethernet() {
        // let frame = mock_frame();
        let data: Vec<u8> = build_reader("ethernet");
        let reader = Reader::new_raw(Rc::new(data));
        let (prop, next) = specs::ethernet::ii::EthernetVisitor.visit2(&reader).unwrap();

        assert_eq!(next, "ipv4");
        match &prop {
            ProtocolData::ETHERNET(el) => {
                let val = el.get().borrow();
                assert_eq!(0x0800, val.ptype);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test] 
    fn test_dns_rr_srv() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        let data: Vec<u8> = build_reader("dns_srv");
        let reader = Reader::new_raw(Rc::new(data));
        let (prop, next) = specs::dns::DNSVisitor.visit2(&reader).unwrap();
        assert_eq!(next, "none");
        match &prop {
            ProtocolData::DNS(el) => {
                let val = el.get().borrow();
                assert_eq!(val.questions, 0);
                assert_eq!(val.answer_rr, 2);
                assert_eq!(val.authority_rr, 0);
                assert_eq!(val.additional_rr, 5);
                inspect(el);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_dns_query() { 
        let data: Vec<u8> = build_reader("dns_query");
        let reader = Reader::new_raw(Rc::new(data));
        let (prop, next) = specs::dns::DNSVisitor.visit2(&reader).unwrap();
        assert_eq!(next, "none");
        match &prop {
            ProtocolData::DNS(el) => {
                let val = el.get().borrow();
                assert_eq!(val.questions, 1);
                assert_eq!(val.answer_rr, 0);
                assert_eq!(val.authority_rr, 0);
                assert_eq!(val.additional_rr, 0);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_dns_auth() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        let data: Vec<u8> = build_reader("dns_auth");
        let reader = Reader::new_raw(Rc::new(data));
        let (prop, next) = specs::dns::DNSVisitor.visit2(&reader).unwrap();
        assert_eq!(next, "none");
        match &prop {
            ProtocolData::DNS(el) => {
                let val = el.get().borrow();
                assert_eq!(val.questions, 1);
                assert_eq!(val.answer_rr, 0);
                assert_eq!(val.authority_rr, 1);
                assert_eq!(val.additional_rr, 1);
                inspect(el);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_tls_err1() {
        let data: Vec<u8> = build_reader("tls");
        let reader = Reader::new_raw(Rc::new(data));
        let prop = specs::tls::TLSVisitor.visit(&reader).unwrap();
        if let ProtocolData::TLS(tls_packet) = prop {
            let tls = tls_packet.get().borrow();
            assert_eq!(tls.records.len(), 1);
        }
    }
    // #[test]
    // fn test_tls_err2() {
    //   let data: Vec<u8> = build_reader("tls_2");
    //   let reader = Reader::new_raw(Rc::new(data));
    //   let (_prop) = specs::tls::TLSVisitor.visit(&reader).unwrap();
    // }

    #[test]
    fn test_certificate() {
        // env_logger::builder().is_test(true).try_init().unwrap();
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
