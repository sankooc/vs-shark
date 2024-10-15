use js_sys::Uint8Array;
// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;



#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Field {
    pub start: usize,
    pub size: usize,
    data: Uint8Array,
}
impl Field {
    pub fn empty() -> Self {
        Field{ start: 0, size: 0, data: Uint8Array::new(&JsValue::undefined()) }
    }
    pub fn convert(embed: &core::common::concept::Field) -> Self {
        let (start, size);
        core::common::concept::Field { start, size, .. } = *embed;
        let a: &[u8] = embed.data.as_ref();
        let data: Uint8Array = a.into();
        Field {
            start,
            size,
            data,
        }
    }
}
#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Uint8Array {
        self.data.clone()
    }
}

#[wasm_bindgen]
pub struct WFileInfo {
    pub link_type: u32,
    file_type: String,
    pub start_time: u64,
    version: String,
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
    // fn new(info: FileInfo) -> WFileInfo {
    //     WFileInfo {
    //         link_type: info.link_type,
    //         file_type: format!("{:?}", info.file_type),
    //         start_time: info.start_time,
    //         version: info.version.clone(),
    //     }
    // }
}
