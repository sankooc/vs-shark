use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs::File, io::BufReader};

use pcap::common::concept::{ConversationCriteria, Criteria, Field, FrameIndex, FrameInfo, ListResult, ProgressStatus, VConnection, VConversation, VHttpConnection};
use pcap::common::io::DataSource;
use pcap::common::{trim_data, Instance};
use std::sync::mpsc::Sender;

use crate::MAX_CONTENT_SIZE;

pub enum PcapEvent {
    Quit,
    ProgressStatus(ProgressStatus),
    Init,
    FrameList(ListResult<FrameInfo>),
    FrameData(Vec<Field>, Option<DataSource>, Option<Vec<u8>>),
    ConversationList(ListResult<VConversation>),
    ConnectionList(ListResult<VConnection>),
    HttpConnectionList(ListResult<VHttpConnection>),
    HttpContent(Option<HttpMessageWrap>, Option<HttpMessageWrap>),
}

pub enum PcapUICommand {
    Quit,
    None,
    Refresh,
    FrameList(usize, usize),
    FrameData(FrameIndex),
    ConversationList(usize, usize),
    ConnectionList(usize, usize, usize),
    HttpConnectionList(usize, usize),
    HttpContent(VHttpConnection),
}


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

pub struct Service {
    file: File,
    fname: String,
    sender: Sender<PcapEvent>,
    receiver: Receiver<PcapUICommand>,
}
#[derive(Clone, Copy)]
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

pub fn seek2(fname: &str, range: Range<usize>) -> anyhow::Result<Vec<u8>> {
    let offset = range.start as u64;
    let size = range.end - range.start;
    let mut file = File::open(fname).unwrap();
    file.seek(SeekFrom::Start(offset))?;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}
pub fn concat_data(file: &mut File, ranges: Vec<(usize, usize)>, len: Option<usize>) -> anyhow::Result<Vec<u8>> {
    if ranges.is_empty() {
        return Ok(vec![]);
    }
    let max = if let Some(length) = len {
        length
    } else {
        ranges.iter().map(|(start, end)| end - start).sum()
    };
    let mut rs = Vec::with_capacity(max);
    for (start, end) in ranges {
        file.seek(SeekFrom::Start(start as u64))?;
        let left = max - rs.len();
        let _size = end - start;
        let size = std::cmp::min(left, _size);
        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer)?;
        rs.extend_from_slice(&buffer);
        if rs.len() >= max {
            break;
        }
    }

    Ok(rs)
}

impl Service {
    pub fn new(fname: String, sender: Sender<PcapEvent>, receiver: Receiver<PcapUICommand>) -> Self {
        let file = File::open(fname.clone()).unwrap();
        Self { fname, file, sender, receiver }
    }
    pub fn seek(&mut self, range: Range<usize>) -> anyhow::Result<Vec<u8>> {
        let offset = range.start as u64;
        let size = range.end - range.start;
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; size];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let batch_size = 1024 * 256;
        let mut ins = Instance::new(batch_size);
        let mut reader = BufReader::new(&mut self.file);
        let mut pos = 0;
        let mut buffer = vec![0; batch_size];
        'main: loop {
            let start_loop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            match self.receiver.try_recv() {
                Ok(cmd) => match cmd {
                    PcapUICommand::Quit => break,
                    PcapUICommand::FrameList(start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.frames_by(cri);
                        self.sender.send(PcapEvent::FrameList(result_list)).unwrap();
                    }
                    PcapUICommand::FrameData(index) => {
                        if let Some(frame) = ins.frame(index as usize) {
                            if let Some(range) = frame.range() {
                                let data = seek2(&self.fname, range)?;
                                if let Some((rs, source, extra)) = ins.select_frame(index as usize, data) {
                                    let ds = if let Some(_source) = source {
                                        let range = frame.frame_range().unwrap();
                                        let data_source = DataSource::create(_source, range);
                                        Some(data_source)
                                    } else {
                                        None
                                    };
                                    self.sender.send(PcapEvent::FrameData(rs, ds, extra)).unwrap();
                                }
                            }
                            // self.sender.send(PcapEvent::FrameData(frame)).unwrap();
                        }
                    }
                    PcapUICommand::ConversationList(start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.conversations(cri, ConversationCriteria::default());
                        self.sender.send(PcapEvent::ConversationList(result_list)).unwrap();
                    }
                    PcapUICommand::ConnectionList(key, start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.connections(key, cri);
                        self.sender.send(PcapEvent::ConnectionList(result_list)).unwrap();
                    }
                    PcapUICommand::HttpConnectionList(start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.http_connections(cri, None);
                        self.sender.send(PcapEvent::HttpConnectionList(result_list)).unwrap();
                    }
                    PcapUICommand::HttpContent(http_connection) => {
                        if let Ok(mut file) = File::open(&self.fname) {
                            let request = if let Some(req) = &http_connection.request {
                                let header = concat_data(&mut file, http_connection.request_headers, None).unwrap_or_default();
                                let entity = concat_data(&mut file, http_connection.request_body, Some(MAX_CONTENT_SIZE)).unwrap_or_default();
                                Some(parse_http_message(req, header, entity))
                            } else {
                                None
                            };
                            let response = if let Some(res) = &http_connection.response {
                                let header = concat_data(&mut file, http_connection.response_headers, None).unwrap_or_default();
                                let entity = concat_data(&mut file, http_connection.response_body, Some(MAX_CONTENT_SIZE)).unwrap_or_default();
                                Some(parse_http_message(res, header, entity))
                            } else {
                                None
                            };
                            self.sender.send(PcapEvent::HttpContent(request, response)).unwrap();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            let metadata = reader.get_ref().metadata()?;
            let new_len = metadata.len();
            let mut _next = 0;
            let mut _pro = None;
            while pos < new_len {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    // time::sleep(Duration::from_millis(100)).await;
                    break;
                }
                pos += n as u64;

                if let Ok(_rs) = ins.update(buffer[..n].to_vec()) {
                    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    if _next < timestamp {
                        // _pro = Some(_rs);
                        self.sender.send(PcapEvent::ProgressStatus(_rs)).unwrap();
                        _next = timestamp + 450;
                    } else {
                        _pro = Some(_rs);
                    }
                } else {
                    self.sender.send(PcapEvent::Quit).unwrap();
                    break 'main;
                }
                // let _rs = ins.update(buffer[..n].to_vec()).unwrap();
            }
            if let Some(rs) = _pro.take() {
                self.sender.send(PcapEvent::ProgressStatus(rs)).unwrap();
            }
            let _next_loop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            if start_loop + 166 > _next_loop {
                thread::sleep(Duration::from_millis((166 + start_loop - _next_loop) as u64));
            }
        }
        Ok(())
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

fn parse_http_message(head: &str, header: Vec<u8>, entity: Vec<u8>) -> HttpMessageWrap {
    let (mut headers, mime, encoding) = parse_header_content(header);
    headers.insert(0, head.to_string());
    let body = parse_body_with_mime(entity, &mime, encoding);
    HttpMessageWrap::new(headers, mime,body)
}

fn parse_header_content(header_raw: Vec<u8>) -> (Vec<String>, Language, HttpEncoding) {
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

enum HttpEncoding {
    None,
    Gzip,
    Deflate,
    Brotli,
    Zstd,
}

fn parse_body_with_mime(body_raw: Vec<u8>, mime: &Language, encoding: HttpEncoding) -> Option<String> {
    if let Language::Binary = &mime { return None }
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
