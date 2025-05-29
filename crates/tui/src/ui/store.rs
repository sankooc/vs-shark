use std::sync::mpsc::Sender;

use pcap::common::{concept::{Field, FrameInfo, ListResult, ProgressStatus}, Frame};

use crate::engine::{PcapCommand, PcapEvent};

use super::{frames::FrameState, stack::StackState};


pub struct Store<'a> {
    pub sender: &'a Sender<PcapCommand>,
    pub progress: Option<ProgressStatus>,
    pub init: bool,
    pub frame_data: Option<FrameState>
}


impl Store<'_> {
    pub fn new<'a>(sender: &'a Sender<PcapCommand>) -> Store<'a> {
        Store {
            sender,
            progress: None,
            init: false,
            frame_data: None
        }
    }
    pub fn select(&mut self){
        if let Some(frame_data) = &self.frame_data {
            let index = frame_data.select;
            if let Some(frame) = frame_data.list.items.get(index) {
                self.sender.send(PcapCommand::FrameData(frame.index)).unwrap();
            }
        }
    }
    pub fn update(&mut self, event: PcapEvent) {
        match event {
            PcapEvent::ProgressStatus(status) => {
                if let None = self.progress {
                    self.sender.send(PcapCommand::FrameList(0, 100)).unwrap();
                }
                self.progress = Some(status);
            },
            PcapEvent::FrameList(list) => {
                self.frame_data = Some(FrameState::new(list));
                self.select();
            },
            PcapEvent::FrameData(fields) => {
                if let Some(frame_data) = &mut self.frame_data {
                    frame_data.field = Some(StackState::new(fields));
                }
            },
            _ => {}
        }
    }
}