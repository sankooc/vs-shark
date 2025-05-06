// #[cfg(test)]
// mod tests {
//     use std::{cell::RefCell, collections::HashMap, hash::BuildHasherDefault};

//     use rustc_hash::FxHasher;

//     type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
//     #[test]
//     fn maptest() {
//         let map:FastHashMap<[u8;4], &'static str> = FastHashMap::default();
//         thread_local! {
//             static IP_CACHE: RefCell<HashMap<[u8; 4], &'static str>> = RefCell::new(HashMap::new());
//         }
        
//         // pub fn read_ip(bytes: &[u8]) -> &'static str {
//         //     let ip_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            
//         //     IP_CACHE.with(|cache| {
//         //         let mut cache = cache.borrow_mut();
//         //         cache.entry(ip_bytes).or_insert_with(|| {
//         //             let s = format!("{}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
//         //             Box::leak(s.into_boxed_str())
//         //         })
//         //     })
//         // }
//         // println!("original: {:p} - {:p}", a1.as_ptr(), a2.as_ptr());
//         // println!("reference: {:p} - {:p}", (&a1).as_ptr(), (&a2).as_ptr());
//         // let c1 = intern(a1);
//         // let c2 = intern(a2);
//         // println!("cached: {:p} - {:p}", c1.as_ptr(), c2.as_ptr());
//     }
// }
