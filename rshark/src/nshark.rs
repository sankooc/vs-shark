use crate::common::FileInfo;
use crate::entry::*;
use crate::files::CContext;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WContext {
    ctx: Box<CContext>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct WFileInfo {
    pub link_type: u16,
    pub file_type: String,
    pub start_time: u64,
    pub version:  String,
}
#[wasm_bindgen]
impl WFileInfo {
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self.version.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn file_type(&self) -> String {
        self.file_type.clone()
    }
}
impl WFileInfo {
    fn new(info: &FileInfo) -> WFileInfo{
        WFileInfo{link_type: info.link_type, file_type:  format!("{:?}",info.file_type), start_time: info.start_time, version: info.version.clone() }
    }
}

// #[wasm_bindgen]
// impl WContext {
//     pub fn get_file_info(&self) -> WFileInfo {
//         WFileInfo::new(self.get_file_info())
//     }
// }

#[wasm_bindgen]
impl WContext {
    #[wasm_bindgen(constructor)]
    pub fn new (s: &Uint8Array) -> WContext {
        let mut slice = vec![0; s.length() as usize];
        s.copy_to(&mut slice[..]);
        WContext {
            ctx: Box::new(load_data(&slice).unwrap()),
        }
    }
    #[wasm_bindgen]
    pub fn get_info(&mut self) -> WFileInfo {
        WFileInfo::new(self.ctx.get_info())
    }
}


#[wasm_bindgen]
pub fn load(s: &Uint8Array) -> WContext {
    WContext::new(s)
    // WContext {
    //     ctx: Box::new(load_data(s).unwrap()),
    // }
}
