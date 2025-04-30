use rustc_hash::FxHasher;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub struct StringPool {
    map: FastHashMap<String, &'static str>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            map: FastHashMap::default(),
        }
    }

    #[inline(always)]
    pub fn intern(&mut self, s: String) -> &'static str {
        if let Some(v) = self.map.get(&s) {
            return *v;
        }
        let static_ref: &'static str = unsafe { std::mem::transmute(s.as_str()) };
        self.map.insert(s, static_ref);
        static_ref
    }
}


thread_local! {
    static STRING_POOL: UnsafeCell<StringPool> = UnsafeCell::new(StringPool::new());
}

#[inline(always)]
pub fn intern(s: String) -> &'static str {
    unsafe { STRING_POOL.with(|pool| (*pool.get()).intern(s)) }
}