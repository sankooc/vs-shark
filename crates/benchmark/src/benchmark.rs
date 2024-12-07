use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;
use once_cell::sync::Lazy;

#[warn(dead_code)]
static _SET: Lazy<RwLock<Vec<Benchmark>>> = Lazy::new(|| RwLock::new(Vec::new()));

static _SINGLETON: Lazy<RwLock<HashMap<&'static str, Instant>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[allow(dead_code)]
struct Benchmark {
    name: &'static str,
    time: u128,
}

impl Benchmark {
    #[allow(dead_code)]
    fn new(name: &'static str, time: u128) -> Self {
        Self { name, time }
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
            crate::benchmark::_SET.write().unwrap().push(crate::benchmark::Benchmark::new($name, start.elapsed().as_millis()));
        } else {
            println!("[{}] Timer not found or already finished", $name);
        }
    }};
}

#[macro_export]
macro_rules! arch_print {
    ($($vals:expr),+) => {
        let mut table = prettytable::Table::new();
        let mut header:Vec<prettytable::Cell> = vec![$($vals),+].iter().map(|v| prettytable::Cell::new(v)).collect();
        header.push(prettytable::Cell::new("Elapsed"));
        table.add_row(prettytable::Row::new(header));
        for b in crate::benchmark::_SET.read().unwrap().iter() {
            let mut list = b.name.split("#").map(|f| prettytable::Cell::new(f)).collect::<Vec<_>>();
            list.push(prettytable::Cell::new(format!("{}ms", b.time).as_str()));
            table.add_row(prettytable::Row::new(list));
        }
        table.printstd();

    };
    () => {{
        for b in crate::benchmark::_SET.read().unwrap().iter() {
            println!("[{}] Elapsed time: {:.3} ms", b.name, b.time);
        }
    }};
}

#[cfg(test)]
mod benchmark {

    use std::fs;

    use shark::{common::base::Configuration, specs::sip::{parse_token, parse_token_with_cache}};

    fn load_data(f: &str) {
        let fname = format!("../../../pcaps/{}", f);
        let times = 3;
        if let Ok(_) = fs::exists(&fname) {
            let data: Vec<u8> = fs::read(&fname).unwrap();
            let task = format!("{}#{}", f, times).leak();
            arch_start!(task);
            for _ in 0..times {
                let _ = shark::entry::load_data(&data, Configuration::new(false)).unwrap();
            }
            arch_finish!(task);
        }
    }
    #[test]
    fn load_bench() {
        load_data("http.pcap");
        load_data("tls.pcapng");
        load_data("wifi.pcap");
        load_data("dns.pcapng");
        // load_data("pppoe.pcap");
        // load_data("sip.pcap");
        // load_data("slow.pcap");
        arch_print!("type", "times");
    }

    #[test]
    fn test_parse(){
        env_logger::builder().is_test(true).try_init().unwrap();
        let f = "http.pcap";
        let fname = format!("../../../pcaps/{}", f);
        if let Ok(_) = fs::exists(&fname) {
            let data: Vec<u8> = fs::read(&fname).unwrap();
            let task = format!("file: {}", f).leak();
            arch_start!(task);
            let _ = shark::entry::load_data(&data, Configuration::new(false)).unwrap();
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
}
