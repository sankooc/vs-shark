
use crossterm::event::KeyEvent;
use pcap::common::concept::ProgressStatus;

use crate::{engine::{PcapEvent, PcapUICommand}, ui::{frames, window::MainUI}};
use enum_dispatch::enum_dispatch;

use super::{frames::{FrameState, SelectPanel}, stack::StackState};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};


#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum UITab {
    #[default]
    Frames = 0,
    Conversations,
}

#[derive(Default)]
pub struct Store {
    pub progress: Option<ProgressStatus>,
    pub tab: UITab,
    // pub init: bool,
    pub frame_data: Option<FrameState>
}

// #[enum_dispatch]
// pub enum TabContainer {
//     Frame(frames::App)
// }

// #[enum_dispatch(TabContainer)]
// pub trait ControlState {
//     fn control(&mut self, shift_pressed: bool, event: KeyEvent) -> PcapUICommand;
//     fn render_it(&mut self, area: Rect, buf: &mut Buffer);
// }

// #[enum_dispatch(TabContainer)]
// pub trait WidgetWrapper {
// }

// impl<T: Widget> WidgetWrapper for T {}

 
impl Store {
    pub fn frame_data(&mut self) -> Option<&mut FrameState>{
        self.frame_data.as_mut()
    }
    pub fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::ProgressStatus(status) => {
                if let None = self.progress {
                    self.progress = Some(status);
                    PcapUICommand::FrameList(0, 100)
                } else {
                    self.progress = Some(status);
                    PcapUICommand::None
                }
                // self.progress = Some(status);
            },
            PcapEvent::FrameList(list) => {
                // let index = list.items.get(0).unwrap().index;
                self.frame_data = Some(FrameState::new(list));
                PcapUICommand::None
            },
            PcapEvent::FrameData(fields, ds,  extra) => {
                if let Some(frame_data) = &mut self.frame_data {
                    frame_data.field = Some(StackState::new(fields, ds, extra));
                }
                PcapUICommand::None
            },
            _ => PcapUICommand::None
        }
    }
    // pub fn selection(&mut self) -> Option<Box<&mut dyn ControlState>> {
    //     if let Some(frame_data) = &mut self.frame_data {
    //         match frame_data.cursor {
    //             SelectPanel::LIST => {
    //                 return Some(Box::new(frame_data));
    //             },
    //             SelectPanel::STACK => {
    //                 if let Some(field) = &mut frame_data.field {
    //                     return Some(Box::new(field));
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    pub fn select_panel(&mut self, panel: SelectPanel) {
        if let Some(frame_data) = &mut self.frame_data {
            frame_data.cursor = panel;
        }
    }
}


// impl ControlState for Store {
//     fn control(&mut self, _: bool, _event: KeyEvent) -> PcapUICommand {
//         PcapUICommand::None
//     }
// }