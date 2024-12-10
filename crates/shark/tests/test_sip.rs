
#[cfg(test)]
mod tc;

#[cfg(test)]
mod tests {

    use shark::{common::io::Reader, specs::{self, ProtocolData}};

    use crate::tc::{build_reader, inspect};

    
    #[test]
    fn test_icmp_echo(){
        let data: Vec<u8> = build_reader("sip_request_ack");
        let reader = Reader::new_raw(data);
        let (prop, _) = specs::sip::SIPVisitor.visit2(&reader).unwrap();
        match &prop {
            ProtocolData::SIP(el) => {
                inspect(el); 
            }
            _ => {
                assert!(false);
            }
        }
    }
    
}