// #[cfg(test)]
// mod tc;
// #[cfg(test)]
// mod unit {
//     use std::net::Ipv4Addr;

//     use crate::tc::{build_reader, print_field};
//     use anyhow::Result;
//     use pcap::{
//         common::{
//             concept::Field,
//             core::Context,
//             enum_def::{InfoField, AddressField, Protocol},
//             io::{DataSource, Reader},
//             Frame,
//         },
//         protocol,
//     };

//     fn init(name: &str) -> (DataSource, Context, Frame) {
//         let data: Vec<u8> = build_reader(name);
//         let mut ds = DataSource::new();
//         let cx: Context = Context::default();
//         let frame = Frame::default();
//         ds.update(data);
//         return (ds, cx, frame);
//     }
//     #[test]
//     fn test_ethernet() -> Result<()> {
//         // let (ds, mut cx, mut frame) = init("ethernet");
//         // {
//         //     let mut reader = Reader::new(&ds);
//         //     let next = protocol::link::ethernet::EthernetVisitor::parse(&mut cx, &mut frame, &mut reader)?;
//         //     assert!(matches!(next, Protocol::IP4));
//         // }
//         // {
//         //     let mut reader = Reader::new(&ds);
//         //     let mut f = Field::default();
//         //     let next = protocol::link::ethernet::EthernetVisitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//         //     assert!(matches!(next, Protocol::IP4));
//         //     print_field(1, &f);
//         // }

//         Ok(())
//     }

//     #[test]
//     fn test_ssl() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("ssl");
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::link::ssl::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::IP4));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::link::ssl::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::IP4));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_ieee1905a() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("ieee1905a");
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::link::ieee1905a::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::link::ieee1905a::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_ipv4() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("ipv4");
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::network::ip4::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::TCP));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::network::ip4::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::TCP));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_ipv6() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("ipv6");
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::network::ip6::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::network::ip6::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_tcp() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("tcp");
//         {
//             let _data = [1, 3, 4, 5, 2, 3, 4, 5];
//             let source = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[..4])?);
//             let target = Ipv4Addr::from(<[u8; 4]>::try_from(&_data[4..])?);
//             frame.ip_field = AddressField::IPv4(source, target);
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::transport::tcp::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::transport::tcp::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_http() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("http");
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::application::http::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);

//         }
//         {
//             frame.info_field = InfoField::Http(vec![]);
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::application::http::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
//     #[test]
//     fn test_icmp4() -> Result<()> {
//         let (ds, mut cx, mut frame) = init("icmp");
//         {
//             let mut reader = Reader::new(&ds);
//             let next = protocol::network::icmp::Visitor::parse(&mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//         }
//         {
//             let mut reader = Reader::new(&ds);
//             let mut f = Field::default();
//             let next = protocol::network::icmp::Visitor::detail(&mut f, &mut cx, &mut frame, &mut reader)?;
//             assert!(matches!(next, Protocol::None));
//             print_field(1, &f);
//         }
//         Ok(())
//     }
// }
