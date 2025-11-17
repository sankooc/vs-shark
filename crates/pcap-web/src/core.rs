use std::{
    ops::Range, path::Path, thread::{self}, time::Duration
};

use pcap::common::{Instance, ResourceLoader};
use tokio::sync::{mpsc, oneshot};
use util::{file_seek, file_seeks};

pub enum UICommand {
    Quit,
    None,
    OpenFile(oneshot::Sender<String>, String),
    List(oneshot::Sender<String>, String),
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

fn create_instance(fname: String) -> Instance<LocalResource> {
    let batch_size = 1024 * 1024 * 4;
    let loader = LocalResource::new(fname);
    Instance::new(batch_size as usize, loader)
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
        println!("Handling GUI command:");
        match cmd {
            UICommand::Quit => {
                println!("Quit command received.");
            }
            UICommand::None => {
                println!("No operation command received.");
            }
            UICommand::OpenFile(tx, filepath) => {
                println!("Open file command received for file: {}", filepath);
                let path = Path::new(&filepath);
                if !path.exists() {
                let _ = tx.send(format!("No File {} ", filepath));
                    return;
                }
                self.ins = Some(create_instance(filepath.clone()));
                let _ = tx.send(format!("File {} opened successfully.", filepath));
            }
            UICommand::List(tx, session_id) => {
                println!("List command received for session: {}", session_id);
                // Here you would add logic to get the list and send back a response
                let _ = tx.send(format!("List for session {}.", session_id));
            }
        }
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
    pub async fn open_file(&self, filepath: String) -> String {
        let (tx, rx) = oneshot::channel();

        let _ = self.gui_tx.send(UICommand::OpenFile(tx, filepath)).await;
        match rx.await {
            Ok(result) => result,
            Err(_) => String::new(),
        }
    }
    pub async fn get_list(&self) -> String {
        "list".to_string()
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
