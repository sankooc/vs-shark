// use rustc_hash::FxHasher;
// use std::cell::UnsafeCell;
// use std::collections::HashMap;
// use std::hash::BuildHasherDefault;
// use std::hash::Hash;
// // use std::collections::hash_map::DefaultHasher;
// use std::hash::Hasher;

// use crate::common::io::{Reader, IO, IP6};
// use crate::common::NString;

// type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

// pub fn hash_ipv6(ip: &[u8]) -> u64 {
//     let mut hasher = FxHasher::default();
//     ip.hash(&mut hasher);
//     hasher.finish()
// }
// // #[no_mangle]
// // #[warn(improper_ctypes_definitions)]
// // pub extern "C" fn hash_ipv6_fxhash(ip: &[u8]) -> u64 {
// //     let mut hasher = FxHasher::default();
// //     hasher.write(ip);
// //     hasher.finish()
// // }

// pub struct StringPool {
//     map: FastHashMap<String, NString>,
//     ip4_map: FastHashMap<u32, NString>,
//     mac_map: FastHashMap<u64, NString>,
//     ip6_map: FastHashMap<u64, IP6>,
// }

// impl StringPool {
//     pub fn new() -> Self {
//         Self {
//             map: FastHashMap::default(),
//             mac_map: FastHashMap::default(),
//             ip4_map: FastHashMap::default(),
//             ip6_map: FastHashMap::default(),
//         }
//     }

//     #[inline(always)]
//     pub fn intern(&mut self, s: String) -> NString {
//         if let Some(v) = self.map.get(&s) {
//             return *v;
//         }
//         let key = s.clone();
//         let static_ref: NString = Box::leak(s.into_boxed_str());
//         // let static_ref: NString = unsafe { std::mem::transmute(s.as_str()) };
//         self.map.insert(key, static_ref);
//         static_ref
//     }
//     #[inline(always)]
//     pub fn intern_ip4(&mut self, reader: &mut Reader) -> anyhow::Result<NString> {
//         let data = reader.slice(4, true)?;
//         let key = IO::read32(data, false)?;
//         if !self.ip4_map.contains_key(&key) {
//             let ip = format!("{}.{}.{}.{}", data[0], data[1], data[2], data[3]);
//             let static_ref: NString = Box::leak(ip.into_boxed_str());
//             self.ip4_map.insert(key, static_ref);
//             return Ok(static_ref);
//         }
//         Ok(self.ip4_map.get(&key).unwrap())
//     }
//     // #[inline(always)]
//     // pub fn intern_ip6(&mut self, reader: &mut Reader) -> anyhow::Result<&IP6> {
//     //     let data = reader.slice(16, true)?;
//     //     let key = hash_ipv6(data);
//     //     if !self.ip6_map.contains_key(&key) {
//     //         let mut args: [u16; 8] = [0; 8];
//     //         for inx in 0..8 {
//     //             let _inx = (inx * 2) as usize;
//     //             args[inx] = ((data[_inx] as u16) * 0x0100) + (data[_inx + 1] as u16);
//     //         }
//     //         let ip = Ipv6Addr::new(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
//     //         self.ip6_map.insert(key, ip.into());
//     //     }

//     //     Ok(self.ip6_map.get(&key).unwrap())
//     // }
//     #[inline(always)]
//     pub fn intern_mac(&mut self, data: &[u8]) -> anyhow::Result<NString> {
//         let key = hash_ipv6(data);
//         if !self.mac_map.contains_key(&key) {
//             let str = format!("{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}", data[0],data[1],data[2],data[3],data[4],data[5]);
//             let static_ref: NString = Box::leak(str.into_boxed_str());
//             self.mac_map.insert(key, static_ref);
//         }
//         Ok(self.mac_map.get(&key).unwrap())
//     }
// }

// thread_local! {
//     static STRING_POOL: UnsafeCell<StringPool> = UnsafeCell::new(StringPool::new());
// }

// // #[inline(always)]
// // pub fn intern(s: String) -> NString {
// //     unsafe { STRING_POOL.with(|pool| (*pool.get()).intern(s)) }
// // }
// // #[inline(always)]
// // pub fn intern_ip4(reader: &mut Reader) -> anyhow::Result<NString> {
// //     unsafe { STRING_POOL.with(|pool| (*pool.get()).intern_ip4(reader)) }
// // }
// // #[inline(always)]
// // pub fn intern_ip6<'a>(reader: &mut Reader) -> anyhow::Result<&'a IP6> {
// //     unsafe { STRING_POOL.with(|pool| (*pool.get()).intern_ip6(reader)) }
// // }
// // #[inline(always)]
// // pub fn intern_mac<'a>(data: &'a [u8]) -> anyhow::Result<NString> {
// //     unsafe { STRING_POOL.with(|pool| (*pool.get()).intern_mac(data)) }
// // }