pub mod ieee1905a;
pub mod ii;
pub mod pppoes;
pub mod radiotap;
pub mod ssl;

pub fn get_next_from_type(ptype: u16) -> &'static str {
    match ptype {
        0x893a => "ieee1905.a",
        0x0800 => "ipv4",
        0x086dd => "ipv6",
        0x0806 => "arp",
        0x8864 => "pppoess",
        _ => "none",
    }
}
