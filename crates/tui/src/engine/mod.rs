use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs::File, io::BufReader};

use pcap::common::concept::{Criteria, Field, FrameIndex, FrameInfo, ListResult, ProgressStatus};
use pcap::common::io::DataSource;
use pcap::common::Instance;
use std::sync::mpsc::Sender;

pub enum PcapEvent {
    Quit,
    ProgressStatus(ProgressStatus),
    FrameList(ListResult<FrameInfo>),
    FrameData(Vec<Field>, Option<DataSource>, Option<Vec<u8>>),
}

pub enum PcapUICommand {
    Quit,
    None,
    Refresh,
    FrameList(usize, usize),
    FrameData(FrameIndex)
}

pub struct Service {
    file: File,
    fname: String,
    sender: Sender<PcapEvent>,
    receiver: Receiver<PcapUICommand>,
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
    pub fn new(fname: String, sender: Sender<PcapEvent>, receiver: Receiver<PcapUICommand>) -> Self {
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
