use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::ops::Range;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pcap::common::concept::{
    ConversationCriteria, Criteria, Field, FrameIndex, FrameInfo, HttpMessageDetail, Language, ListResult, ProgressStatus, VConnection, VConversation, VHttpConnection,
};
use pcap::common::io::DataSource;
use pcap::common::{Instance, ResourceLoader};
use std::sync::mpsc::Sender;
use util::{file_seek, file_seeks};

// use crate::MAX_CONTENT_SIZE;

pub enum PcapEvent {
    Quit,
    ProgressStatus(ProgressStatus),
    Init,
    FrameList(ListResult<FrameInfo>),
    FrameData(Vec<Field>, Vec<DataSource>),
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
    HttpDetail(usize),
}

pub struct LocalResource {
    filepath: String,
}

impl ResourceLoader for LocalResource {
    fn load(&self, range: &Range<usize>) -> anyhow::Result<Vec<u8>> {
        file_seek(&self.filepath, range)
    }
    fn loads(&self, ranges: &[Range<usize>]) -> anyhow::Result<Vec<u8>> {
        file_seeks(&self.filepath, ranges)
    }
}

impl LocalResource {
    fn new(filepath: String) -> Self {
        LocalResource { filepath }
    }
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

impl From<&HttpMessageDetail> for HttpMessageWrap {
    fn from(value: &HttpMessageDetail) -> Self {
        let mime = value.text_type();
        let parsed_content = value.get_text_content();
        let headers = value.headers.clone();
        HttpMessageWrap::new(headers, mime, parsed_content)
    }
}

pub struct Service {
    file: File,
    fname: String,
    sender: Sender<PcapEvent>,
    receiver: Receiver<PcapUICommand>,
}

impl Service {
    pub fn new(fname: String, sender: Sender<PcapEvent>, receiver: Receiver<PcapUICommand>) -> Self {
        let file = File::open(fname.clone()).unwrap();
        Self { fname, file, sender, receiver }
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let batch_size = 1024 * 256;
        let loader = LocalResource::new(self.fname.clone());
        let mut ins = Instance::new(batch_size, loader);
        let mut reader = BufReader::new(&mut self.file);
        let mut pos = 0;
        let mut buffer = vec![0; batch_size];
        'main: loop {
            let start_loop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            if let Ok(cmd) = self.receiver.try_recv() {
                match cmd {
                    PcapUICommand::Quit => break,
                    PcapUICommand::FrameList(start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.frames_by(cri);
                        self.sender.send(PcapEvent::FrameList(result_list)).unwrap();
                    }
                    PcapUICommand::FrameData(index) => {
                        if let Some(frame) = ins.frame(index as usize) {
                            if let Some(_range) = frame.range() {
                                if let Some((rs, datasources)) = ins.select_frame(index as usize) {
                                    // let ds = if let Some(_source) = source {
                                    //     let range = _range2.unwrap();
                                    //     let data_source = DataSource::create(_source, range);
                                    //     Some(data_source)
                                    // } else {
                                    //     None
                                    // };
                                    self.sender.send(PcapEvent::FrameData(rs, datasources)).unwrap();
                                }
                            }
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
                        let result_list = ins.http_connections(cri, None, true);
                        self.sender.send(PcapEvent::HttpConnectionList(result_list)).unwrap();
                    }
                    PcapUICommand::HttpDetail(index) => {
                        if let Some(rs) = ins.http_detail(index) {
                            let request = if !rs.is_empty() {
                                let item = rs.first().unwrap();
                                if item.is_request {
                                    Some(item.into())
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            let response: Option<HttpMessageWrap> = if !rs.is_empty() {
                                let item = rs.last().unwrap();
                                if item.is_request {
                                    None
                                } else {
                                    Some(item.into())
                                }
                            } else {
                                None
                            };
                            self.sender.send(PcapEvent::HttpContent(request, response)).unwrap();
                        }
                    }

                    // PcapUICommand::HttpContent(http_connection) => {
                    //     if let Ok(mut file) = File::open(&self.fname) {
                    //         let request = if let Some(req) = &http_connection.request {
                    //             let header = concat_data(&mut file, http_connection.request_headers, None).unwrap_or_default();
                    //             let entity = concat_data(&mut file, http_connection.request_body, Some(MAX_CONTENT_SIZE)).unwrap_or_default();
                    //             Some(parse_http_message(req, header, entity))
                    //         } else {
                    //             None
                    //         };
                    //         let response = if let Some(res) = &http_connection.response {
                    //             let header = concat_data(&mut file, http_connection.response_headers, None).unwrap_or_default();
                    //             let entity = concat_data(&mut file, http_connection.response_body, Some(MAX_CONTENT_SIZE)).unwrap_or_default();
                    //             Some(parse_http_message(res, header, entity))
                    //         } else {
                    //             None
                    //         };
                    //         self.sender.send(PcapEvent::HttpContent(request, response)).unwrap();
                    //     }
                    // }
                    _ => {}
                }
            }
            let metadata = reader.get_ref().metadata()?;
            let new_len = metadata.len();
            if new_len == 0 {
                self.sender.send(PcapEvent::Quit).unwrap();
                break 'main;
            }
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
