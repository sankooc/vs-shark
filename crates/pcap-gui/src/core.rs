use std::ops::Range;

use pcap::common::{Instance, ResourceLoader};
use tokio::sync::mpsc;
use util::{file_seek, file_seeks};

pub enum PcapUICommand {
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
    ins: Instance<LocalResource>,
    gui_rx: mpsc::Receiver<PcapUICommand>,
    rr_rx: mpsc::Receiver<ResourceCommand>,
}

impl Engine {
    async fn handle_gui(cmd: &PcapUICommand) {
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

                else => break,
            }
        }
    }
}


