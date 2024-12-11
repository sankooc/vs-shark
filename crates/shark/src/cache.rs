use std::collections::HashMap;
use std::cell::UnsafeCell;

// use std::sync::RwLock;
// use once_cell::sync::Lazy;
// use crate::specs::ethernet::ii::Ethernet;

use crate::specs::sip::SIPURI;
struct Singleton {
    inner: UnsafeCell<Option<HashMap<&'static str, SIPURI>>>,
    // ether: UnsafeCell<Option<HashMap<&'static [u8], Ethernet>>>,
}

// pub static ETHERNET_MAP: Lazy<RwLock<HashMap<&'static [u8], Ethernet>>> = Lazy::new(|| RwLock::new(HashMap::new()));

// pub fn add_ethernet_map(key: &'static [u8], data: Ethernet) {
//     ETHERNET_MAP.write().unwrap().insert(key, data);
// }
// pub fn get_ethernet_map(key: &'static [u8]) -> Option<Ethernet> {
//     ETHERNET_MAP.read().unwrap().get(key).map(|f| f.clone())
// }

unsafe impl Sync for Singleton {}

static SINGLETON: Singleton = Singleton {
    inner: UnsafeCell::new(None),
};

fn get_sip_singleton() -> &'static mut HashMap<&'static str, SIPURI> {
    unsafe {
        let inner = &mut *SINGLETON.inner.get();
        if inner.is_none() {
            *inner = Some(HashMap::new()); // 初始化 HashMap
        }
        inner.as_mut().unwrap()
    }
}

pub fn get_sip_url(line: &str) -> Option<&SIPURI> {
     get_sip_singleton().get(line) 
}

pub fn add_sip_url(line: &'static str, data: SIPURI) {
     get_sip_singleton().insert(line, data);
}
