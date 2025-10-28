use js_sys::Uint8Array;
use pcap::common::trim_data;
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Conf {
    id: String,
    resolve_all: bool,
    batch_size: usize,
}
#[wasm_bindgen]
impl Conf {
    #[wasm_bindgen]
    pub fn new(id: String, resolve_all: bool, batch_size: usize) -> Self {
        Self { id, resolve_all, batch_size }
    }
    #[wasm_bindgen]
    pub fn resolve_all(&self) -> bool {
        self.resolve_all
    }
    #[wasm_bindgen]
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[wasm_bindgen]
impl Range {
    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }
    #[wasm_bindgen]
    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

impl From<&std::ops::Range<usize>> for Range {
    fn from(value: &std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[wasm_bindgen]
pub struct FrameResult {
    list: String,
    extra: Option<Vec<u8>>,
    source: Option<Vec<u8>>,
    range: Option<std::ops::Range<usize>>
}

impl FrameResult {
    pub fn new(list: String, source: Option<Vec<u8>>, extra: Option<Vec<u8>>, range: Option<std::ops::Range<usize>>) -> Self {
        Self { list, source, extra, range }
    }
}

#[wasm_bindgen]
impl FrameResult {

    pub fn empty() -> Self {
        Self {
            list: "{}".into(),
            source: None,
            extra: None,
            range: None,
        }
    }
    fn to_uint8(data: &Option<Vec<u8>>) -> Uint8Array {
        if let Some(v) = data {
            Uint8Array::from(v.as_slice())
        } else {
            Uint8Array::from(JsValue::null())
        }
    }
    #[wasm_bindgen]
    pub fn list(&self) -> String {
        self.list.clone()
    }
    #[wasm_bindgen]
    pub fn source(&self) -> Uint8Array {
        FrameResult::to_uint8(&self.source)
    }
    #[wasm_bindgen]
    pub fn extra(&self) -> Uint8Array {
        FrameResult::to_uint8(&self.extra)
    }

    pub fn range(&self) -> Option<Range> {
        self.range.as_ref().map(|f|f.into())
    }
}

#[wasm_bindgen]
pub struct FrameRange {
    pub frame: Range,
    pub data: Range,
}

#[wasm_bindgen]
impl FrameRange {
    pub fn new() -> Self {
        Self {
            frame: Range::empty(),
            data: Range::empty(),
        }
    }
    #[wasm_bindgen]
    pub fn compact(&self) -> bool {
        self.frame.start == self.data.start && self.frame.end == self.data.end
    }
}

impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Serialize, Clone)]
// #[wasm_bindgen]
pub enum Language {
    Text,
    Json,
    JavaScript,
    Css,
    Html,
    Xml,
    Csv,
    Yaml,
    Binary,
}

#[derive(Serialize, Clone)]
pub enum HttpEncoding {
    None,
    Gzip,
    Deflate,
    Brotli,
    Zstd,
}

// #[wasm_bindgen]
#[derive(Serialize)]
pub struct HttpMessageWrap {
    pub headers: Vec<String>,
    pub mime: Language,
    pub parsed_content: Option<String>,
}

impl HttpMessageWrap {
    pub fn new(headers: Vec<String>, mime: Language, parsed_content: Option<String>) -> Self {
        Self { headers, mime, parsed_content }
    }
}

fn parse_content_type(content_type_str: &str) -> Language {
    let main_type = content_type_str.to_lowercase();

    if main_type.is_empty() {
        return Language::Binary;
    }
    if main_type.contains("/json") {
        return Language::Json;
    }
    if main_type.contains("/javascript") {
        return Language::JavaScript;
    }
    if main_type.contains("/css") {
        return Language::Css;
    }
    if main_type.contains("/html") {
        return Language::Html;
    }
    if main_type.contains("/xml") {
        return Language::Xml;
    }
    if main_type.contains("/csv") {
        return Language::Csv;
    }
    if main_type.contains("/yaml") {
        return Language::Yaml;
    }
    if main_type.contains("text/") {
        return Language::Text;
    }
    Language::Binary
}

pub fn parse_http_message(head: &str, header: Vec<u8>, entity: Option<Vec<u8>>) -> HttpMessageWrap {
    let (mut headers, mime, encoding) = parse_header_content(header);
    headers.insert(0, head.to_string());
    let body = if let Some(content) = entity {
        parse_body_with_mime(content, &mime, encoding)
    } else {
        None
    };
    HttpMessageWrap::new(headers, mime, body)
}

pub fn parse_header_content(header_raw: Vec<u8>) -> (Vec<String>, Language, HttpEncoding) {
    if header_raw.is_empty() {
        return (vec![], Language::Binary, HttpEncoding::None);
    }
    let text = String::from_utf8_lossy(&header_raw);
    let headers: Vec<&str> = text.split("\r\n").collect();
    let mut content_type = Language::Binary;
    let mut encoding = HttpEncoding::None;
    let mut rs = vec![];
    for head in headers.into_iter() {
        if head.chars().count() == 0 {
            continue;
        }
        rs.push(head.into());
        if head.starts_with("Content-Type: ") || head.starts_with("content-type: ") {
            content_type = parse_content_type(&head[14..]);
        }
        if head.starts_with("Content-Encoding: ") || head.starts_with("content-encoding: ") {
            let _type = trim_data(&head.as_bytes()[18..]);
            match _type {
                b"gzip" => {
                    encoding = HttpEncoding::Gzip;
                }
                b"deflate" => {
                    encoding = HttpEncoding::Deflate;
                }
                b"br" => {
                    encoding = HttpEncoding::Brotli;
                }
                b"zstd" => {
                    encoding = HttpEncoding::Zstd;
                }
                _ => {}
            }
            // encoding = HttpEncoding::Gzip;
        }
    }
    (rs, content_type, encoding)
}

fn parse_body_with_mime(body_raw: Vec<u8>, mime: &Language, encoding: HttpEncoding) -> Option<String> {
    match &mime {
        Language::Binary => return None,
        _ => {}
    }
    let decoded_data = match encoding {
        HttpEncoding::None => body_raw,
        HttpEncoding::Gzip => {
            use flate2::read::GzDecoder;
            use std::io::Read;
            let mut decoder = GzDecoder::new(&body_raw[..]);
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw,
            }
        }
        HttpEncoding::Deflate => {
            use flate2::read::DeflateDecoder;
            use std::io::Read;
            let mut decoder = DeflateDecoder::new(&body_raw[..]);
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw,
            }
        }
        HttpEncoding::Brotli => {
            use brotli::Decompressor;
            use std::io::Read;
            let mut decoded = Vec::new();
            match Decompressor::new(&body_raw[..], 4096).read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw,
            }
        }
        HttpEncoding::Zstd => {
            use std::io::Read;
            use zstd::stream::read::Decoder;
            let Ok(mut decoder) = Decoder::new(&body_raw[..]) else {
                return Some(String::from_utf8_lossy(&body_raw).to_string());
            };
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw,
            }
        }
    };
    let plain = String::from_utf8(decoded_data).unwrap_or_default();
    Some(plain)
}
