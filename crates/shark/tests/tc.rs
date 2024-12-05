#[cfg(test)]
pub mod tc {
    use shark::common::filter::{PacketProps, Parser};
    #[test]
    fn test_filter_express() {
        {
            let input = "tcp";
            let mut parser = Parser::new(input);
            if let Err(_) = parser.parse() {
                assert!(false)
            }
        }
        {
            let input = "(tcp.ip == 123.32.2.1)";
            let mut parser = Parser::new(input);
            if let Err(_) = parser.parse() {
                assert!(false)
            }
        }
        {
            let input = "tcp.ip==123.32.2.1&&cpp=1";
            let mut parser = Parser::new(input);
            match parser.parse() {
                Ok(expr) => {
                    println!("{:#?}", expr);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        {
            let input = "tcp.ip ==123.32.2.1 && (act.count >= 1 || ppc.c < 12)";
            let mut parser = Parser::new(input);
            match parser.parse() {
                Ok(_expr) => {
                    // println!("{:#?}", expr);
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        // let mut data: HashMap<String, String> = HashMap::new();
        // data.insert("tcp.ip".to_string(), "123.32.2.1".to_string());
        // data.insert("act".to_string(), "out".to_string());

        // let input = "tcp.ip == 123.32.2.1 && act=out";
        // let mut parser = Parser::new(input);
        // match parser.parse() {
        //     Ok(expr) => {
        //         // println!("{:#?}", expr);
        //         let result = evaluate_expression(&expr, &data);
        //         println!("Match result: {}", result);
        //     }
        //     Err(e) => println!("Error: {}", e),
        // }
    }

    #[test]
    fn test_props() {
        let mut pp1 = PacketProps::new();
        let mut pp2 = PacketProps::new();
        pp1.add("udp", "");
        pp1.add("tcp.ip", "1234");
        pp1.add("tcp.ip", "1235");
        pp1.add("mac.address", "d1:23:a2:12");
        pp2.add("tcp.ip", "1236");
        assert_eq!(pp1.get("tcp.ip").unwrap(), ["1234", "1235"]);
        pp1.merge(&mut pp2);
        assert_eq!(pp1.get("tcp.ip").unwrap(), ["1234", "1235", "1236"]);
        assert_eq!(pp2.get("tcp.ip"), None);

        // println!("{}", pp1.match_expr("tcp.ip == 1234 || tcp == 12"));
        assert!(pp1.match_expr("udp"));
        // assert!(!pp1.match_expr("tcp.ip"));
        // assert!(pp1.match_expr("tcp.ip == 1234"));
        // assert!(!pp1.match_expr("tcp.ip == 100"));
        // assert!(pp1.match_expr("tcp.ip > 100"));
        // assert!(!pp1.match_expr("tcp.ip > 100 && tcp.ip < 200"));
        // assert!(pp1.match_expr("tcp.ip == 1234 || tcp == 12"));
        // assert!(pp1.match_expr("(tcp.ip == 1234 || tcp == 12)"));
        // assert!(!pp1.match_expr("tcp.ip == 1234 && tcp == 12"));
    }
}

use std::{fs, str::from_utf8};

use shark::common::{base::PacketContext, concept::Field};

#[cfg(test)]
#[allow(dead_code)]
pub fn build_reader(name: &str) -> Vec<u8> {
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
    println!("{:inx$}- {}", "", field.summary());
    let fields = field.children();
    for f in fields.iter() {
        _dis(inx + 1, f);
    }
}
#[cfg(test)]
#[allow(dead_code)]
pub fn inspect<T>(packet: &PacketContext<T>) {
    let field = packet.get_fields();
    for f in field.iter() {
        _dis(1, f);
    }
}
