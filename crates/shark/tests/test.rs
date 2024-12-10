
#[cfg(test)]
mod tc;
#[cfg(test)]
mod unit {
    //https://wiki.wireshark.org/samplecaptures
    //https://github.com/chrissanders/packets/tree/master
    use shark::{
        common::io::Reader,
        specs::{self, ProtocolData},
    };
    use crate::tc::{build_reader, inspect};

    

    #[test]
    fn test_radiotap(){
        let data: Vec<u8> = build_reader("radiotap3");
        let reader = Reader::new_raw(data);
        let (prop, _) = specs::ethernet::ieee80211::RadiotapVisitor.visit2(&reader).unwrap();
        match &prop {
            ProtocolData::Radiotap(el) => {
                inspect(el); 
            }
            _ => {
                assert!(false);
            }
        }
        
    }
    #[test]
    fn test_ppp_lcp() {
        // let frame = mock_frame();
        let data: Vec<u8> = build_reader("ppp.lcp");
        let reader = Reader::new_raw(data);
        let (prop, _) = specs::ethernet::pppoes::PPPoESSVisitor.visit2(&reader).unwrap();
        match &prop {
            ProtocolData::PPPoES(el) => {
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
        let reader = Reader::new_raw(data);
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
        let reader = Reader::new_raw(data);
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
        let reader = Reader::new_raw(data);
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
        let reader = Reader::new_raw(data);
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
        let reader = Reader::new_raw(data);
        let prop = specs::tls::TLSVisitor.visit(&reader).unwrap();
        if let ProtocolData::TLS(tls_packet) = prop {
            let tls = tls_packet.get().borrow();
            assert_eq!(tls.records.len(), 1);
        }
    }
    // #[test]
    // fn test_tls_err2() {
    //   let data: Vec<u8> = build_reader("tls_2");
    //   let reader = Reader::new_raw(data);
    //   let (_prop) = specs::tls::TLSVisitor.visit(&reader).unwrap();
    // }

    #[test]
    fn test_certificate() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        let data: Vec<u8> = build_reader("certificate");
        let reader = Reader::new_raw(data);
        let ins = specs::tls::handshake::Certificate::create(&reader, Some(1425)).unwrap();
        inspect(&ins);
        println!("finish");
    }

    #[test]
    fn test_certificates() {
        // let data: Vec<u8> = build_reader("certificates");
        // let reader = Reader::new_raw(data);
        // let ins = specs::tls::handshake::Certificates::create(&reader, Some(1425)).unwrap();
        // println!("--");
    }
}
