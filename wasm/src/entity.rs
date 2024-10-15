use js_sys::Uint8Array;
// use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;



#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct Field {
    pub start: usize,
    pub size: usize,
    summary: String,
    children: Vec<core::common::base::Field>,
    data: Uint8Array,
}
impl Field {
    pub fn convert(embed: &core::common::base::Field) -> Self {
        let (start, size);
        core::common::base::Field { start, size, .. } = *embed;
        let summary = embed.summary.clone();
        let a: &[u8] = embed.data.as_ref();
        let data: Uint8Array = a.into();
        let children = embed.children.clone();
        Field {
            start,
            size,
            summary,
            data,
            children,
        }
    }
}
#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(getter)]
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn children(&self) -> Vec<Field> {
        let mut children = Vec::new();
        for c in self.children.iter() {
            children.push(Field::convert(c));
        }
        children
    }
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
