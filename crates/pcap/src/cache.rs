use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub struct StringPool {
    map: FastHashMap<&'static str, ()>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            map: FastHashMap::default(),
        }
    }

    #[inline(always)]
    pub fn intern(&mut self, s: &str) -> &'static str {
        if let Some((&existing, _)) = self.map.get_key_value(s) {
            return existing;
        }
        
        let boxed: Box<str> = s.to_owned().into_boxed_str();
        let static_ref: &'static str = Box::leak(boxed);
        self.map.insert(static_ref, ());
        static_ref
    }
}

static mut POOL: Option<StringPool> = None;

#[inline(always)]
pub fn intern(s: &str) -> &'static str {
    unsafe {
        if POOL.is_none() {
            POOL = Some(StringPool::new());
        }
        POOL.as_mut().unwrap_unchecked().intern(s)
    }
}
