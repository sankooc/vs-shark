use std::{cell::UnsafeCell, collections::HashMap};

use crate::specs::sip::SIPURI;

static mut SINGLETON: UnsafeCell<Option<HashMap<&'static str, SIPURI>>> = UnsafeCell::new(None);
static mut _CONFIG: UnsafeCell<Option<HashMap<&'static str, bool>>> = UnsafeCell::new(None);



pub fn get_config(key: &'static str) -> bool {
    unsafe {
        if (*_CONFIG.get()).is_none() {
            *_CONFIG.get() = Some(HashMap::new());
        }
        *(*_CONFIG.get()).as_mut().unwrap().get(key).unwrap_or(&true)
    }

}
pub fn set_config(key: &'static str, value: bool) {
    unsafe {
        if (*_CONFIG.get()).is_none() {
            *_CONFIG.get() = Some(HashMap::new());
        }
        (*_CONFIG.get()).as_mut().unwrap().insert(key, value);
    }
}
fn get_sip_singleton() -> &'static mut HashMap<&'static str, SIPURI> {
    unsafe {
        if (*SINGLETON.get()).is_none() {
            *SINGLETON.get() = Some(HashMap::new());
        }
        (*SINGLETON.get()).as_mut().unwrap()
    }
}

pub fn get_sip_url(line: &str) -> Option<&SIPURI> {
    get_sip_singleton().get(line)
}
pub fn add_sip_url(line: &'static str, data: SIPURI) {
    get_sip_singleton().insert(line, data);
}
