use std::{cell::UnsafeCell, collections::HashMap, time::Instant};

static mut _SINGLETON: UnsafeCell<Option<HashMap<&'static str, Instant>>> = UnsafeCell::new(None);
static mut _SET: UnsafeCell<Vec<Benchmark>> = UnsafeCell::new(Vec::new());

struct Benchmark {
    name: &'static str,
    time: u128,
}
impl Benchmark {
    fn new(name: &'static str, time: u128) -> Self {
        Self { name, time }
    }
}

#[macro_export]
macro_rules! arch_start {
    ($name:expr) => {{
        unsafe {
            if (*crate::benchmark::_SINGLETON.get()).is_none() {
                *crate::benchmark::_SINGLETON.get() = Some(HashMap::new());
            }
            (*crate::benchmark::_SINGLETON.get()).as_mut().unwrap().insert($name, std::time::Instant::now());
        }
    }};
}

#[macro_export]
macro_rules! arch_finish {
    ($name:expr) => {{
        unsafe {
            if (*crate::benchmark::_SINGLETON.get()).is_none() {
                *crate::benchmark::_SINGLETON.get() = Some(HashMap::new());
            }
            if let Some(start) = (*crate::benchmark::_SINGLETON.get()).as_mut().unwrap().remove($name) {
                (*crate::benchmark::_SET.get()).push(crate::benchmark::Benchmark::new($name, start.elapsed().as_millis()));
                // let elapsed = start.elapsed();
                // println!("[{}] Elapsed time: {:.3} ms", $name, elapsed.as_millis());
            } else {
                println!("[{}] Timer not found or already finished", $name);
            }
        }
    }};
}
#[macro_export]
macro_rules! arch_print {
    ($($vals:expr),+) => {
        unsafe {
            let mut table = prettytable::Table::new();
            let mut header:Vec<prettytable::Cell> = vec![$($vals),+].iter().map(|v| prettytable::Cell::new(v)).collect();
            header.push(prettytable::Cell::new("Elapsed"));
            table.add_row(prettytable::Row::new(header));
            for b in (*crate::benchmark::_SET.get()).iter() {
                let mut list = b.name.split("#").map(|f| prettytable::Cell::new(f)).collect::<Vec<_>>();
                list.push(prettytable::Cell::new(format!("{}ms", b.time).as_str()));
                table.add_row(prettytable::Row::new(list));
            }
            table.printstd();

        }
    };
    () => {{
        unsafe {
            for b in (*crate::benchmark::_SET.get()).iter() {
                println!("[{}] Elapsed time: {:.3} ms", b.name, b.time);
            }
        }
    }};
}

#[cfg(test)]
mod benchmark {

    use std::{collections::HashMap, fs};

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
        let f = "pppoe.pcap";
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
