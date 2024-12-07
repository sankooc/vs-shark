use std::collections::HashMap;
use std::cell::UnsafeCell;

use crate::specs::sip::SIPURI;
struct Singleton {
    inner: UnsafeCell<Option<HashMap<&'static str, SIPURI>>>,
}

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
