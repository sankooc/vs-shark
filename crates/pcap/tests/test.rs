
#[cfg(test)]
mod tc;
#[cfg(test)]
mod unit {
    use pcap::{common::{concept::Field, enum_def::Protocol, io::{DataSource, Reader}, core::Context, Frame}, protocol};
    use anyhow::Result;
    use crate::tc::{build_reader, print_field};

    fn init(name: &str) -> (DataSource, Context, Frame) {
        let data: Vec<u8> = build_reader(name);
        let mut ds = DataSource::new();
        let cx: Context = Context::default();
        let frame = Frame::default();
        ds.update(data);
        return (ds, cx, frame)
    }
    #[test]
    fn test_ethernet() -> Result<()>{
        let (ds, mut cx, mut frame) = init("ethernet");
        {
            let mut reader = Reader::new(&ds);
            let next = protocol::link::ethernet::EthernetVisitor::parse(&mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::IP4));
        }
        {
            let mut reader = Reader::new(&ds);
            let mut f = Field::empty();
            let next = protocol::link::ethernet::EthernetVisitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::IP4));
            print_field(1,&f);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_ssl() -> Result<()>{
        let (ds, mut cx, mut frame) = init("ssl");
        {
            let mut reader = Reader::new(&ds);
            let next = protocol::link::ssl::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::IP4));
        }
        {
            let mut reader = Reader::new(&ds);
            let mut f = Field::empty();
            let next = protocol::link::ssl::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::IP4));
            print_field(1,&f);
        }
        Ok(())
    }
    #[test]
    fn test_ieee1905a() -> Result<()>{
        let (ds, mut cx, mut frame) = init("ieee1905a");
        {
            let mut reader = Reader::new(&ds);
            let next = protocol::link::ieee1905a::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::None));
        }
        {
            let mut reader = Reader::new(&ds);
            let mut f = Field::empty();
            let next = protocol::link::ieee1905a::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
            assert!(matches!(next, Protocol::None));
            print_field(1,&f);
        }
        Ok(())
    }
}
