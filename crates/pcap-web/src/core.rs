use std::{ops::Range, thread::{self, Thread}, time::Duration};

use pcap::common::{Instance, ResourceLoader};
use tokio::sync::mpsc;
use util::{file_seek, file_seeks};

pub enum UICommand {
    Quit,
    None,
    OpenFile(String),
}

pub enum PcapEngineCommand {
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

pub struct Engine {
    // ins: Instance<LocalResource>,
    gui_rx: mpsc::Receiver<UICommand>,
    rr_rx: mpsc::Receiver<ResourceCommand>,
}

impl Engine {
    pub fn new(
        // ins: Instance<LocalResource>,
        gui_rx: mpsc::Receiver<UICommand>,
        rr_rx: mpsc::Receiver<ResourceCommand>,
    ) -> Self {
        Engine {
            gui_rx,
            rr_rx,
        }
    }
}

impl Engine {
    async fn handle_gui(cmd: &UICommand) {
        // todo!();
    }
    async fn handle_resource(cmd: &ResourceCommand) {}
        // todo!();

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(msg) = self.gui_rx.recv() => {
                    let response = Self::handle_gui(&msg).await;
                    
                }
                Some(msg) = self.rr_rx.recv() => {
                    let response = Self::handle_resource(&msg).await;
                }

                else => thread::sleep(Duration::from_millis(50)),
            }
        }
    }
}


pub trait UIEngine {
    
}


pub fn build_engine() -> Engine {
    let (gui_rx, grx) = mpsc::channel::<UICommand>(10);
    let (rui_rx, rrx) = mpsc::channel::<ResourceCommand>(10);

    Engine::new(
        grx,
        rrx,
    )
}