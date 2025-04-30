
use crate::common::{FileType, io::Reader};

pub fn execute(file_type: &FileType, link_type: u32, reader: &mut Reader) -> &'static str {
    match link_type {
        0 => {
            if let FileType::PCAPNG = file_type {
                return "loopback";
            }
            let _head = reader.slice(16, false).unwrap();
            if _head[0] == 0 && _head[5] == 6 {
                let lat = &_head[14..16];
                let _flag = u16::from_be_bytes(lat.try_into().unwrap());
                return match _flag {
                    0x0806 | 0x0800 | 0x86dd | 0x8864 => "ssl",
                    _ => "ethernet",
                };
            }
            "ethernet"
        }
        127 => "radiotap",
        113 => "ssl",
        _ => "ethernet",
    }
}


pub mod index;