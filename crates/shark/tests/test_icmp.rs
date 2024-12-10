
#[cfg(test)]
mod tc;

#[cfg(test)]
mod tests {

    use shark::{common::io::Reader, specs::{self, ProtocolData}};

    use crate::tc::{build_reader, inspect};
    #[test]
    fn test_icmp_echo(){
        let data: Vec<u8> = build_reader("icmp_echo");
        let reader = Reader::new_raw(data);
        let (prop, _) = specs::icmp::ICMPVisitor.visit2(&reader).unwrap();
        match &prop {
            ProtocolData::ICMP(el) => {
                inspect(el); 
            }
            _ => {
                assert!(false);
            }
        }
    }
    
}