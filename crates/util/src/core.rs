use std::{
    ops::Range,
    path::Path,
    thread::{self},
    time::Duration,
};

use anyhow::bail;
use pcap::common::{
    Instance, ResourceLoader, concept::{ConversationCriteria, Criteria, DNSRecord, DNSResponse, Field, FrameIndex, FrameInfo, HttpCriteria, HttpMessageDetail, ListResult, TLSConversation, TLSItem, UDPConversation, VConnection, VConversation, VHttpConnection}, io::DataSource
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::{FileBatchReader, file_seek, file_seeks};

pub enum UICommand {
    Quit,
    None,
    OpenFile(oneshot::Sender<Result<(), String>>, String),
    Frames(oneshot::Sender<ListResult<FrameInfo>>, Criteria),
    Frame(oneshot::Sender<FrameResult>, FrameIndex),
    List(oneshot::Sender<String>, String),
    Stat(oneshot::Sender<String>, String),
    TCPList(oneshot::Sender<ListResult<VConversation>>, Criteria, ConversationCriteria),
    TCPConvList(oneshot::Sender<ListResult<VConnection>>, usize, Criteria),
    UDPList(oneshot::Sender<ListResult<UDPConversation>>, Criteria, Option<String>, bool),
    TLSList(oneshot::Sender<ListResult<TLSConversation>>, Criteria),
    TLSDetail(oneshot::Sender<ListResult<TLSItem>>, usize, Criteria),
    DNSRecors(oneshot::Sender<ListResult<DNSResponse>>, Criteria, bool),
    DNSRecord(oneshot::Sender<ListResult<DNSRecord>>, usize, Criteria),
    HTTPList(oneshot::Sender<ListResult<VHttpConnection>>, Criteria, Option<HttpCriteria>, bool),
    HTTPDetail(oneshot::Sender<Option<Vec<HttpMessageDetail>>>, usize),
}

pub enum EngineCommand {
    Quit,
    None,
    FileInfo(String),
}

pub enum ResourceCommand {
    NoFile,
    Open(String),
    Close(String),
    Error(String),
    Data(Vec<u8>),
}

#[derive(Deserialize, Serialize)]
pub struct FrameResult {
    pub fields: Vec<Field>,
    pub datasource: Vec<DataSource>,
}

impl FrameResult {
    pub fn new(fields: Vec<Field>, datasource: Vec<DataSource>) -> Self {
        FrameResult { fields, datasource }
    }
    pub fn empty() -> Self {
        FrameResult {
            fields: vec![],
            datasource: vec![],
        }
    }
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

fn create_instance(fname: String, batch_size: usize) -> Instance<LocalResource> {
    let loader = LocalResource::new(fname);
    Instance::new(batch_size, loader)
}

fn readfile(filepath: String) -> anyhow::Result<Instance<LocalResource>> {
    let path = Path::new(&filepath);
    if !path.exists() {
        bail!("no file")
    }
    let batch_size = 1024 * 1024 * 4;
    let mut ins = create_instance(filepath.clone(), batch_size);
    let mut batcher = FileBatchReader::new(filepath.to_string(), batch_size as u64);
    let mut finish = false;
    while !finish {
        finish = if let Ok((left, data)) = batcher.read() {
            ins.update(data)?;
            left == 0
        } else {
            true
        }
    }
    Ok(ins)
}

fn jsonlize<T>(data: &T) -> Option<String>
where
    T: Serialize,
{
    serde_json::to_string(&data).ok()
}

fn stat_str(tp: &str, instance: &Instance<LocalResource>) -> Option<String> {
    let items = match tp {
        "http_host" => instance.stat_http_host(),
        "ip4" => instance.stat_ip4(),
        "ip6" => instance.stat_ip6(),
        "http_data" => {
            let rs = instance.stat_http();
            return jsonlize(&rs);
        }
        "frame" => {
            let rs = instance.stat_frame();
            return jsonlize(&rs);
        }
        "ip_address" => instance.stat_ipaddress_distribute(),
        _ => {
            return None;
        }
    };
    jsonlize(&items)
}

pub struct Engine {
    ins: Option<Instance<LocalResource>>,
    gui_rx: mpsc::Receiver<UICommand>,
    // rr_rx: mpsc::Receiver<ResourceCommand>,
}

impl Engine {
    pub fn new(gui_rx: mpsc::Receiver<UICommand>) -> Self {
        Engine { gui_rx, ins: None }
    }
}

impl Engine {
    async fn handle_gui(&mut self, cmd: UICommand) {
        if let Some(instance) = &self.ins {
            match cmd {
                UICommand::Frames(tx, cri) => {
                    let rs = instance.frames_by(cri);
                    let _ = tx.send(rs);
                }
                UICommand::Frame(tx, index) => {
                    let _ = if let Some((list, datasource)) = instance.select_frame(index as usize) {
                        let rs = FrameResult::new(list, datasource);
                        tx.send(rs)
                    } else {
                        tx.send(FrameResult::empty())
                    };
                }
                UICommand::Stat(tx, tp) => {
                    let _ = if let Some(str) = stat_str(&tp, instance) {
                        tx.send(str)
                    } else {
                        tx.send("[]".to_string())
                    };
                }
                UICommand::TCPList(tx, criteria, filter) => {
                    let rs = instance.conversations(criteria, filter);
                    let _ = tx.send(rs);
                }
                UICommand::TCPConvList(tx, index, criteria) => {
                    let rs = instance.connections(index, criteria);
                    let _ = tx.send(rs);
                }
                UICommand::UDPList(tx, cri, filter, asc) => {
                    let rs = instance.udp_conversations(cri, filter, asc);
                    let _ = tx.send(rs);
                }
                UICommand::TLSList(tx, cri) => {
                    let rs = instance.tls_connections(cri);
                    let _ = tx.send(rs);
                }
                UICommand::TLSDetail(tx, index, criteria) => {
                    let rs = instance.tls_conv_list(index, criteria);
                    let _ = tx.send(rs);
                }
                UICommand::DNSRecors(tx, cri, asc) => {
                    let rs = instance.dns_records(cri, asc);
                    let _ = tx.send(rs);
                }
                UICommand::DNSRecord(tx, index, cri) => {
                    let rs = instance.dns_record(index, cri);
                    let _ = tx.send(rs);
                }
                UICommand::HTTPList(tx, cri, filter, asc) => {
                    let rs = instance.http_connections(cri, filter, asc);
                    let _ = tx.send(rs);
                }
                UICommand::HTTPDetail(tx, index) => {
                    let rs = instance.http_detail(index);
                    let _ = tx.send(rs);
                }

                _ => {}
            }
        } else {
            match cmd {
                UICommand::OpenFile(tx, filepath) => {
                    println!("Open file command received for file: {}", filepath);
                    if let Ok(ins) = readfile(filepath.clone()) {
                        self.ins = Some(ins);
                        println!("File {} opened successfully.", filepath);
                        let _ = tx.send(Ok(()));
                    } else {
                        self.ins = None;
                        let _ = tx.send(Err("load failed".to_string()));
                    }
                }
                _ => {
                    println!("No instance loaded. Command cannot be processed.");
                }
            }
        };
    }
    async fn _handle_resource(_cmd: &ResourceCommand) {}
    // todo!();

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.gui_rx.recv() => {
                    let _response = self.handle_gui(msg).await;
                }
                // Some(msg) = self.rr_rx.recv() => {
                //     let response = Self::handle_resource(&msg).await;
                // }

                else => thread::sleep(Duration::from_millis(50)),
            }
        }
    }
}

pub struct UIEngine {
    gui_tx: mpsc::Sender<UICommand>,
    rx: mpsc::Receiver<EngineCommand>,
}

impl UIEngine {
    pub fn new(gui_tx: mpsc::Sender<UICommand>, rx: mpsc::Receiver<EngineCommand>) -> Self {
        UIEngine { gui_tx, rx }
    }
    pub async fn open_file(&self, filepath: String) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::OpenFile(tx, filepath)).await;
        rx.await.map_err(|e| e.to_string())?
    }
    pub async fn get_list(&self) -> String {
        "list".to_string()
    }
    pub async fn frames(&self, cri: Criteria) -> ListResult<FrameInfo> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::Frames(tx, cri)).await;
        rx.await.unwrap()
    }
    pub async fn frame(&self, index: FrameIndex) -> FrameResult {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::Frame(tx, index)).await;
        rx.await.unwrap()
    }
    pub async fn stat(&self, tp: String) -> String {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::Stat(tx, tp)).await;
        rx.await.unwrap()
    }
    pub async fn conversations(&self, cri: Criteria, filter: ConversationCriteria) -> ListResult<VConversation> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::TCPList(tx, cri, filter)).await;
        rx.await.unwrap()
    }
    pub async fn connections(&self, index: usize, cri: Criteria) -> ListResult<VConnection> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::TCPConvList(tx, index, cri)).await;
        rx.await.unwrap()
    }

    pub async fn udp_list(&self, cri: Criteria, filter: Option<String>, asc: bool) -> ListResult<UDPConversation> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::UDPList(tx, cri, filter, asc)).await;
        rx.await.unwrap()
    }

    pub async fn tls_list(&self, cri: Criteria) -> ListResult<TLSConversation> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::TLSList(tx, cri)).await;
        rx.await.unwrap()
    }

    pub async fn tls_detail(&self, index: usize, cri: Criteria) -> ListResult<TLSItem> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::TLSDetail(tx, index, cri)).await;
        rx.await.unwrap()
    }

    pub async fn dns_records(&self, cri: Criteria, asc: bool) -> ListResult<DNSResponse> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::DNSRecors(tx, cri, asc)).await;
        rx.await.unwrap()
    }

    pub async fn dns_record(&self, index: usize, cri: Criteria) -> ListResult<DNSRecord> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::DNSRecord(tx, index, cri)).await;
        rx.await.unwrap()
    }

    pub async fn http_list(&self, cri: Criteria, filter: Option<HttpCriteria>, asc: bool) -> ListResult<VHttpConnection> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::HTTPList(tx, cri, filter, asc)).await;
        rx.await.unwrap()
    }

    pub async fn http_detail(&self, index: usize) -> Option<Vec<HttpMessageDetail>> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::HTTPDetail(tx, index)).await;
        rx.await.unwrap()
    }

    pub async fn run(&mut self) {
        loop {
            if let Some(msg) = self.rx.recv().await {
                match msg {
                    EngineCommand::Quit => break,
                    EngineCommand::None => {}
                    EngineCommand::FileInfo(_) => {}
                }
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

pub fn build_engine() -> (UIEngine, Engine) {
    let (gui_tx, grx) = mpsc::channel::<UICommand>(10);
    let (_rui_tx, _rrx) = mpsc::channel::<ResourceCommand>(10);
    let (_e_tx, erx) = mpsc::channel::<EngineCommand>(10);
    let ui_engine = UIEngine::new(gui_tx.clone(), erx);
    let engine = Engine::new(grx);
    (ui_engine, engine)
}
