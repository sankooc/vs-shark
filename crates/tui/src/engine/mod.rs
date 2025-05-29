use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs::File, io::BufReader};

use pcap::common::concept::{Criteria, Field, FrameInfo, ListResult, ProgressStatus};
use pcap::common::Instance;
use std::sync::mpsc::Sender;

pub enum PcapEvent {
    Quit,
    ProgressStatus(ProgressStatus),
    FrameList(ListResult<FrameInfo>),
    FrameData(Vec<Field>),
}

pub enum PcapCommand {
    Quit,
    FrameList(usize, usize),
    FrameData(u32)
}

pub struct Service {
    file: File,
    fname: String,
    sender: Sender<PcapEvent>,
    receiver: Receiver<PcapCommand>,
}


pub fn seek2(fname: &str, range: Range<usize>) -> anyhow::Result<Vec<u8>>{
    let offset = range.start as u64;
    let size = range.end - range.start;
    let mut file = File::open(fname).unwrap();
    file.seek(SeekFrom::Start(offset))?;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

impl Service {
    pub fn new(fname: String, sender: Sender<PcapEvent>, receiver: Receiver<PcapCommand>) -> Self {
        let file = File::open(fname.clone()).unwrap();
        Self { fname, file, sender, receiver }
    }
    pub fn seek(&mut self, range: Range<usize>) -> anyhow::Result<Vec<u8>>{
        let offset = range.start as u64;
        let size = range.end - range.start;
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; size];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let batch_size = 1024 * 256 * 1;
        let mut ins = Instance::new(batch_size);
        let mut reader = BufReader::new(&mut self.file);
        let mut pos = 0;
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0);
        loop {
            match self.receiver.try_recv() {
                Ok(cmd) => match cmd {
                    PcapCommand::Quit => break,
                    PcapCommand::FrameList(start, size) => {
                        let cri = Criteria { start, size };
                        let result_list = ins.frames_by(cri);
                        self.sender.send(PcapEvent::FrameList(result_list)).unwrap();
                    }
                    PcapCommand::FrameData(index) => {
                        if let Some(frame) = ins.frame(index as usize) {
                            if let Some(range) = frame.range() {
                                let data = seek2(&self.fname, range)?;
                                if let Some(rs) = ins.select_frame(index as usize, data) {
                                    self.sender.send(PcapEvent::FrameData(rs)).unwrap();
                                }
                            }
                            // self.sender.send(PcapEvent::FrameData(frame)).unwrap();
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

                let _rs = ins.update(buffer[..n].to_vec()).unwrap();
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                if _next < timestamp {
                    // _pro = Some(_rs);
                    self.sender.send(PcapEvent::ProgressStatus(_rs)).unwrap();
                    _next = timestamp + 450;
                } else {
                    _pro = Some(_rs);
                }
            }
            if let Some(rs) = _pro.take() {
                self.sender.send(PcapEvent::ProgressStatus(rs)).unwrap();
            }
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}
