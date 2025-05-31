
use crossterm::event::KeyEvent;
use pcap::common::concept::ProgressStatus;

use crate::engine::{PcapCommand, PcapEvent};

use super::{frames::{FrameState, SelectPanel}, stack::StackState};


#[derive(Default)]
pub struct Store {
    pub progress: Option<ProgressStatus>,
    pub init: bool,
    pub frame_data: Option<FrameState>
}

pub trait ControlState {
    fn control(&mut self, shift_pressed: bool, event: KeyEvent) -> PcapCommand;
}

impl Store {
    pub fn frame_data(&mut self) -> Option<&mut FrameState>{
        self.frame_data.as_mut()
    }
    pub fn update(&mut self, event: PcapEvent) -> PcapCommand {
        match event {
            PcapEvent::ProgressStatus(status) => {
                if let None = self.progress {
                    self.progress = Some(status);
                    PcapCommand::FrameList(0, 100)
                } else {
                    self.progress = Some(status);
                    PcapCommand::None
                }
                // self.progress = Some(status);
            },
            PcapEvent::FrameList(list) => {
                // let index = list.items.get(0).unwrap().index;
                self.frame_data = Some(FrameState::new(list));
                PcapCommand::None
            },
            PcapEvent::FrameData(fields, ds,  extra) => {
                if let Some(frame_data) = &mut self.frame_data {
                    frame_data.field = Some(StackState::new(fields, ds, extra));
                }
                PcapCommand::None
            },
            _ => PcapCommand::None
        }
    }
    pub fn selection(&mut self) -> Option<Box<&mut dyn ControlState>> {
        if let Some(frame_data) = &mut self.frame_data {
            match frame_data.cursor {
                SelectPanel::LIST => {
                    return Some(Box::new(frame_data));
                },
                SelectPanel::STACK => {
                    if let Some(field) = &mut frame_data.field {
                        return Some(Box::new(field));
                    }
                }
            }
        }
        None
    }
    pub fn select_panel(&mut self, panel: SelectPanel) {
        if let Some(frame_data) = &mut self.frame_data {
            frame_data.cursor = panel;
        }
    }
}