use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;

#[warn(dead_code)]
static _SET: Lazy<RwLock<HashMap<&'static str, Benchmark>>> = Lazy::new(|| RwLock::new(HashMap::new()));

static _SINGLETON: Lazy<RwLock<HashMap<&'static str, Instant>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[allow(dead_code)]
struct Benchmark {
    total: u128,
    times: usize,
    min: u128,
    max: u128,
    // name: &'static str,
    // time: u128,
}

impl Benchmark {
    #[allow(dead_code)]
    fn new(time: u128) -> Self {
        Self {
            times: 1,
            total: time,
            min: time,
            max: time,
        }
    }
    #[allow(dead_code)]
    fn add(&mut self, time: u128) {
        self.times += 1;
        self.total += time;
        self.min = self.min.min(time);
        self.max = self.max.max(time);
    }
}
#[macro_export]
macro_rules! arch_start {
    ($name:expr) => {{
        crate::benchmark::_SINGLETON.write().unwrap().insert($name, std::time::Instant::now());
    }};
}

#[macro_export]
macro_rules! arch_finish {
    ($name:expr) => {{
        if let Some(start) = crate::benchmark::_SINGLETON.write().unwrap().remove($name) {
            let time = start.elapsed().as_millis();
            let mut set = crate::benchmark::_SET.write().unwrap();
            if let Some(cc) = set.get_mut($name) {
                cc.add(time);
            } else {
                set.insert($name, crate::benchmark::Benchmark::new(time));
            }
        } else {
            println!("[{}] Timer not found or already finished", $name);
        }
    }};
}

#[macro_export]
macro_rules! arch_print {
    () => {{
        let mut table = prettytable::Table::new();
        let header: Vec<prettytable::Cell> = vec!["name", "times", "min", "max", "avg"].iter().map(|v| prettytable::Cell::new(v)).collect();
        table.add_row(prettytable::Row::new(header));
        for (name, val) in crate::benchmark::_SET.read().unwrap().iter() {
            let mut ll: Vec<String> = vec![(*name).into()];
            ll.push(format!("{}", val.times));
            ll.push(format!("{}Ms", val.min));
            ll.push(format!("{}Ms", val.max));
            let avg = val.total as f64 / val.times as f64;
            ll.push(format!("{:.3}Ms", avg));
            let list = ll.iter().map(|f| prettytable::Cell::new(f)).collect::<Vec<_>>();
            table.add_row(prettytable::Row::new(list));
        }
        table.printstd();
    }};
}

#[cfg(test)]
mod benchmark {

    use std::{
        cell::{Cell, RefCell},
        fs,
    };

    use shark::{
        common::base::Configuration,
        specs::sip::{parse_token, parse_token_with_cache},
    };

    fn load_data(f: &str) {
        let fname = format!("../../../pcaps/{}", f);
        let times = 10;
        if let Ok(_) = fs::exists(&fname) {
            let data: Vec<u8> = fs::read(&fname).unwrap();
            let task = format!("{}", f).leak();
            for _ in 0..times {
                arch_start!(task);
                let _ = shark::entry::load_data(data.clone(), Configuration::new(false)).unwrap();
                arch_finish!(task);
            }
        }
    }
    #[test]
    fn load_bench() {
        // load_data("11.pcapng");
        load_data("http.pcap");
        load_data("tls.pcapng");
        load_data("wifi.pcap");
        load_data("dns.pcapng");
        load_data("pppoe.pcap");
        load_data("sip.pcap");
        // load_data("slow.pcap");
        arch_print!();
    }

    #[test]
    fn test_parse() {
        env_logger::builder().is_test(true).try_init().unwrap();
        // let f = "http.pcap";
        let f = "11.pcapng";
        let fname = format!("../../../pcaps/{}", f);
        if let Ok(_) = fs::exists(&fname) {
            let data: Vec<u8> = fs::read(&fname).unwrap();
            let task = format!("{}", f).leak();
            arch_start!(task);
            let _ = shark::entry::load_data(data, Configuration::new(false)).unwrap();
            arch_finish!(task);
            arch_print!();
        }
    }

    #[test]
    fn map_cache() {
        let token1 = "sip:test@10.0.2.15:5060";
        let token2 = "sip:sip.cybercity.dk";
        let token3 = "sip:user@example.com:5060;transport=udp?subject=project&priority=urgent";

        let count = 100000;
        arch_start!("parse_each");
        for _ in 0..count {
            parse_token(token1);
            parse_token(token2);
            parse_token(token3);
        }
        arch_finish!("parse_each");

        arch_start!("mapping");
        for _ in 0..count {
            parse_token_with_cache(token1);
            parse_token_with_cache(token2);
            parse_token_with_cache(token3);
        }
        arch_finish!("mapping");
        arch_print!();
    }
    #[test]
    fn test_mut() {
        struct A {
            count: Cell<usize>,
        }
        struct MutA {
            count: usize,
        }
        let times = 10000000;
        arch_start!("refcell");
        let a = RefCell::new(A { count: Cell::new(0) });
        for i in 0..times {
            let reff = a.borrow_mut();
            reff.count.set(i);
            drop(reff)
        }
        arch_finish!("refcell");

        arch_start!("cell");
        let a = A { count: Cell::new(0) };
        for i in 0..times {
            a.count.set(i);
        }
        arch_finish!("cell");

        arch_start!("mut");
        let mut a = MutA { count: 0 };
        for i in 0..times {
            a.count = i;
        }
        arch_finish!("mut");

        arch_print!()
    }

    // mod eth {
    //     use shark::{
    //         cache::{add_ethernet_map, get_ethernet_map},
    //         common::{
    //             io::{AReader, Reader},
    //             MacAddress,
    //         },
    //         specs::ethernet::ii::Ethernet,
    //     };

    //     // #[derive(Default)]
    //     // pub struct Ethernet {
    //     //     source_mac: Option<MacAddress>,
    //     //     target_mac: Option<MacAddress>,
    //     //     len: u16,
    //     //     pub ptype: u16,
    //     // }
    //     fn ether_parse(reader: &Reader) -> Ethernet {
    //         let mut p = Ethernet::default();
    //         p.source_mac = Some(reader.read_mac().unwrap());
    //         p.target_mac = Some(reader.read_mac().unwrap());
    //         p.ptype = reader.read16(true).unwrap();
    //         p
    //     }
    //     fn ether_parse_with_cache(reader: &'static Reader) -> Ethernet {
    //         let slice: &'static [u8] = reader._slice(14);
    //         if let Some(eth) = get_ethernet_map(slice) {
    //             return eth;
    //         } else {
    //             let rs = ether_parse(reader);
    //             add_ethernet_map(slice, rs.clone());
    //             return rs;
    //         }
    //     }

    //     #[test]
    //     fn parse_test() {
    //         let count = 10000;
    //         arch_start!("parse");
    //         for _ in 0..count {
    //             let reader = Reader::new_raw(vec![0xc4, 0xd0, 0xe3, 0xd8, 0x3e, 0x98, 0x26, 0x5a, 0xfc, 0x25, 0x9c, 0x53, 0x08, 0x00]);
    //             ether_parse_with_cache(&reader);
    //         }
    //         arch_finish!("parse");

    //         arch_start!("cache");
    //         arch_finish!("cache");

    //         arch_print!()
    //     }
    // }
}
