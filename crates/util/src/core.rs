use std::{ops::Range, path::Path, sync::Arc, time::Duration};

use anyhow::bail;
use pcap::common::{
    concept::{
        ConversationCriteria, Criteria, DNSRecord, DNSResponse, Field, FrameIndex, FrameInfo, HttpCriteria, HttpMessageDetail, ListResult, ProgressStatus, TLSConversation,
        TLSItem, UDPConversation, VConnection, VConversation, VHttpConnection,
    },
    io::DataSource,
    Instance, ResourceLoader,
};
use serde::{Deserialize, Serialize};
use tokio::{
    io::AsyncReadExt,
    sync::{mpsc, oneshot, Mutex},
    task::AbortHandle,
    time::sleep,
};

use crate::{file_seek, file_seeks};

pub enum UICommand {
    Quit,
    None,
    OpenFile(oneshot::Sender<Result<(), String>>, String),
    CloseFile(oneshot::Sender<Result<(), String>>),
    Frames(oneshot::Sender<ListResult<FrameInfo>>, Criteria),
    Frame(oneshot::Sender<FrameResult>, FrameIndex),
    List(oneshot::Sender<String>, String),
    Stat(oneshot::Sender<String>, String),
    TCPList(oneshot::Sender<ListResult<VConversation>>, Criteria, ConversationCriteria),
    TCPConvList(oneshot::Sender<ListResult<VConnection>>, usize, Criteria),
    UDPList(oneshot::Sender<ListResult<UDPConversation>>, Criteria, Option<String>, bool),
    TLSList(oneshot::Sender<ListResult<TLSConversation>>, Criteria),
    TLSDetail(oneshot::Sender<ListResult<TLSItem>>, usize, Criteria),
    DNSRecords(oneshot::Sender<ListResult<DNSResponse>>, Criteria, bool),
    DNSRecord(oneshot::Sender<ListResult<DNSRecord>>, usize, Criteria),
    HTTPList(oneshot::Sender<ListResult<VHttpConnection>>, Criteria, Option<HttpCriteria>, bool),
    HTTPDetail(oneshot::Sender<Option<Vec<HttpMessageDetail>>>, usize),
}

pub enum EngineCommand {
    Quit,
    None,
    Error(String),
    Progress(ProgressStatus),
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
    pub fn new(filepath: String) -> Self {
        LocalResource { filepath }
    }
}

fn create_instance(fname: String, batch_size: usize) -> Arc<Mutex<Instance<LocalResource>>> {
    let loader = LocalResource::new(fname);
    let ins = Instance::new(batch_size, loader);
    Arc::new(Mutex::new(ins))
}

fn jsonlize<T>(data: &T) -> Option<String>
where
    T: Serialize,
{
    serde_json::to_string(&data).ok()
}

async fn stat_str(tp: &str, instance: &Arc<Mutex<Instance<LocalResource>>>) -> Option<String> {
    let items = match tp {
        "http_host" => instance.lock().await.stat_http_host(),
        "ip4" => instance.lock().await.stat_ip4(),
        "ip6" => instance.lock().await.stat_ip6(),
        "http_data" => {
            let rs = instance.lock().await.stat_http();
            return jsonlize(&rs);
        }
        "frame" => {
            let rs = instance.lock().await.stat_frame();
            return jsonlize(&rs);
        }
        "ip_address" => instance.lock().await.stat_ipaddress_distribute(),
        _ => {
            return None;
        }
    };
    jsonlize(&items)
}

pub struct Engine {
    ins: Option<Arc<Mutex<Instance<LocalResource>>>>,
    gui_rx: mpsc::Receiver<UICommand>,
    engine_tx: mpsc::Sender<EngineCommand>,
    handler: Option<AbortHandle>,
    watch: bool,
}

impl Engine {
    pub fn new(gui_rx: mpsc::Receiver<UICommand>, engine_tx: mpsc::Sender<EngineCommand>) -> Self {
        Engine {
            gui_rx,
            engine_tx,
            ins: None,
            handler: None,
            watch: true,
        }
    }
}

impl Engine {
    async fn start_read(&mut self, filepath: &str) -> anyhow::Result<()> {
        let buf_size = 5 * 1024 * 1024;
        let engine_tx = self.engine_tx.clone();
        let path = Path::new(filepath);
        if !path.exists() {
            let _ = engine_tx.send(EngineCommand::Error("file not exist".to_string())).await;
            bail!("no file")
        }
        if self.ins.is_some() {
            let _ = engine_tx.send(EngineCommand::Error("file already opened".to_string())).await;
            bail!("instance exists")
        }
        // println!("start parse");
        let instance = create_instance(filepath.to_string(), buf_size);
        let instance_clone = Arc::downgrade(&instance);
        self.ins = Some(instance);
        let watch = self.watch;
        let fname = filepath.to_string();
        let handle = tokio::spawn(async move {
            // println!("start thread");
            let path = Path::new(&fname);
            if !path.exists() {
                let _ = engine_tx.send(EngineCommand::Error("file not exist".to_string())).await;
                bail!("no file")
            }
            let mut file = match tokio::fs::File::open(&path).await {
                Ok(f) => f,
                Err(_e) => {
                    let _ = engine_tx.send(EngineCommand::Error("failed to open file".to_string())).await;
                    bail!("failed to open file")
                }
            };

            let mut cursor = 0;
            let mut last_modify = 0;
            // println!("start load");
            loop {
                if let Some(ins_arc) = instance_clone.upgrade() {
                    let (total, _last_modify) = if let Ok(metadata) = file.metadata().await {
                        if let Ok(modi) = metadata.modified() {
                            if let Ok(duration) = modi.duration_since(std::time::UNIX_EPOCH) {
                                (metadata.len(), duration.as_secs())
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    };
                    if last_modify > 0 && _last_modify == last_modify {
                        if watch {
                            sleep(Duration::from_millis(1000)).await;
                            continue;
                        } else {
                            break;
                        }
                    }
                    let mut buffer = vec![0; buf_size];
                    if let Ok(read_size) = file.read(&mut buffer).await {
                        if read_size > 0 {
                            cursor += read_size;
                            let pf = { ins_arc.lock().await.update_slice(&buffer[..read_size]).ok() };
                            if let Some(mut prog) = pf {
                                prog.total = total as usize;
                                prog.cursor = cursor;
                                println!("get instance start read from {prog:?}");
                                let _ = engine_tx.send(EngineCommand::Progress(prog)).await;
                            }
                            continue;
                        } else {
                            // println!("read finished");
                            last_modify = _last_modify;
                            if watch {
                                sleep(Duration::from_millis(1000)).await;
                            } else {
                                break;
                            }
                        }
                    } else {
                        println!("read failed");
                        break;
                    }
                } else {
                    println!("instance dropped");
                    break;
                };
            }
            Ok(())
        });
        self.handler = Some(handle.abort_handle());
        Ok(())
    }
    async fn handle_gui(&mut self, cmd: UICommand) {
        if let Some(instance) = &self.ins {
            match cmd {
                UICommand::CloseFile(tx) => {
                    self.ins = None;
                    if let Some(handle) = self.handler.take() {
                        handle.abort();
                    }
                    let _ = tx.send(Ok(()));
                }
                UICommand::Frames(tx, cri) => {
                    let rs = instance.lock().await.frames_by(cri);
                    let _ = tx.send(rs);
                }
                UICommand::Frame(tx, index) => {
                    let _ = if let Some((list, datasource)) = instance.lock().await.select_frame(index as usize) {
                        let rs = FrameResult::new(list, datasource);
                        tx.send(rs)
                    } else {
                        tx.send(FrameResult::empty())
                    };
                }
                UICommand::Stat(tx, tp) => {
                    let _ = if let Some(str) = stat_str(&tp, instance).await {
                        tx.send(str)
                    } else {
                        tx.send("[]".to_string())
                    };
                }
                UICommand::TCPList(tx, criteria, filter) => {
                    let rs = instance.lock().await.conversations(criteria, filter);
                    let _ = tx.send(rs);
                }
                UICommand::TCPConvList(tx, index, criteria) => {
                    let rs = instance.lock().await.connections(index, criteria);
                    let _ = tx.send(rs);
                }
                UICommand::UDPList(tx, cri, filter, asc) => {
                    let rs = instance.lock().await.udp_conversations(cri, filter, asc);
                    let _ = tx.send(rs);
                }
                UICommand::TLSList(tx, cri) => {
                    let rs = instance.lock().await.tls_connections(cri);
                    let _ = tx.send(rs);
                }
                UICommand::TLSDetail(tx, index, criteria) => {
                    let rs = instance.lock().await.tls_conv_list(index, criteria);
                    let _ = tx.send(rs);
                }
                UICommand::DNSRecords(tx, cri, asc) => {
                    let rs = instance.lock().await.dns_records(cri, asc);
                    let _ = tx.send(rs);
                }
                UICommand::DNSRecord(tx, index, cri) => {
                    let rs = { instance.lock().await.dns_record(index, cri) };
                    let _ = tx.send(rs);
                }
                UICommand::HTTPList(tx, cri, filter, asc) => {
                    let rs = { instance.lock().await.http_connections(cri, filter, asc) };
                    let _ = tx.send(rs);
                }
                UICommand::HTTPDetail(tx, index) => {
                    let rs = { instance.lock().await.http_detail(index) };
                    let _ = tx.send(rs);
                }

                _ => {}
            }
        } else {
            match cmd {
                UICommand::OpenFile(tx, filepath) => {
                    if self.start_read(filepath.as_str()).await.is_ok() {
                        let _ = tx.send(Ok(()));
                    } else {
                        self.ins = None;
                        if let Some(handle) = self.handler.take() {
                            handle.abort();
                        }
                        let _ = tx.send(Err("load failed".to_string()));
                    }
                }
                _ => {
                    // println!("No instance loaded. Command cannot be processed.");
                }
            }
        };
    }
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.gui_rx.recv() => {
                    let _response = self.handle_gui(msg).await;
                }
            }
        }
    }
}

pub struct UIEngine {
    gui_tx: mpsc::Sender<UICommand>,
}

impl UIEngine {
    pub fn new(gui_tx: mpsc::Sender<UICommand>) -> Self {
        UIEngine { gui_tx }
    }
    pub async fn open_file(&self, filepath: String) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::OpenFile(tx, filepath)).await;
        rx.await.map_err(|e| e.to_string())?
    }
    pub async fn close_file(&self) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        let _ = self.gui_tx.send(UICommand::CloseFile(tx)).await;
        rx.await.unwrap()
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
        let _ = self.gui_tx.send(UICommand::DNSRecords(tx, cri, asc)).await;
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

    // pub async fn run(&mut self) {
    //     loop {
    //         if let Some(msg) = self.rx.recv().await {
    //             match msg {
    //                 EngineCommand::Quit => break,
    //                 EngineCommand::None => {}
    //                 EngineCommand::Progress(_) => {}
    //             }
    //         } else {
    //             println!("waiting next");
    //             thread::sleep(Duration::from_millis(500));
    //         }
    //     }
    // }
}

pub fn build_engine() -> (UIEngine, Engine, mpsc::Receiver<EngineCommand>) {
    let (gui_tx, grx) = mpsc::channel::<UICommand>(10);
    let (etx, erx) = mpsc::channel::<EngineCommand>(10);
    let ui_engine = UIEngine::new(gui_tx);
    let engine = Engine::new(grx, etx);
    (ui_engine, engine, erx)
}
