#[cfg(test)]
use std::{fs, str::from_utf8};

use pcap::common::concept::Field;

// use shark::common::{base::PacketContext, concept::Field};

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

pub fn print_field(inx: usize, field: &Field) {
    let start = field.start;
    let end= field.start + field.size;
    println!("[{:5}-{:5}] {:inx$}- {}",start, end, "", field.summary);
    if let Some(fields) = &field.children {
        for f in fields.iter() {
            print_field(inx + 1, f);
        }   
    }
}
#[cfg(test)]
#[allow(dead_code)]
pub fn print_fields(field: &[Field]) {
    for f in field.iter() {
        print_field(1, f);
    }
}
