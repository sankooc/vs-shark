use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{
    concept::VHttpConnection,
    util::format_bytes_single_unit_int,
};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Rect}, widgets::Widget
};

use crate::{
    engine::{PcapEvent, PcapUICommand}, ui::{loading, render_table, ControlState, CustomTableState, TableStyle}
};

pub struct Page {
    state: CustomTableState<VHttpConnection>,
}
pub struct ConversationStyle;
impl TableStyle<VHttpConnection> for ConversationStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_header_style()
    }

    fn get_row_style(&self, _: &VHttpConnection, _: usize) -> ratatui::prelude::Style {
        crate::theme::DNS_BG.into()
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["", "Status", "Method", "Host", "Length", "ContentType", "Time"]
    }

    fn get_row(&self, data: &VHttpConnection) -> Vec<String> {

        vec![
            "⏎".into(),
            data.status.clone(),
            data.method.clone(),
            data.url.clone(),
            format_bytes_single_unit_int(data.length as usize),
            data.content_type.clone(),
            data.rt.clone(),
        ]
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(1),
            Constraint::Max(5),
            Constraint::Max(10),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Max(14),
        ]
    }
}


impl Page {
    pub fn new() -> Self {
        Self {
            state: CustomTableState::new(),
        }
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.loading {
            loading::line("Loading http connections...", area, buf);
            return;
        }
        // if let Some((_,title, state)) = &self.detail {
        //     let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]);
        //     let rects = vertical.split(area);
        //     {
        //         let target = add_border(rects[0], buf);
        //         let text = Text::from(title.as_str());
        //         let paragraph = Paragraph::new(text).alignment(Alignment::Left).style(title_color());
        //         paragraph.render(target, buf);
        //     }

        //     render_table(ConnectionStyle, state, rects[1], buf);
        //     return;
        // }
        render_table(ConversationStyle, &self.state, area, buf, 0);
    }
}

const PAGE_SIZE: usize = 100;

impl ControlState for Page {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.state.loading {
            return PcapUICommand::None;
        }
        
        // if let Some((key,_, state)) = &mut self.detail {
        //     let index = *key;
        //     match event.code {
        //         // KeyCode::Backspace => {
        //         //     self.detail = None;
        //         // }
        //         KeyCode::Down => {
        //             state.next();
        //         }
        //         KeyCode::Up => {
        //             state.previous();
        //         }
        //         KeyCode::Right => {
        //             let total = self.state.list.total;
        //             let len = self.state.list.items.len();
        //             if total == 0 || len < PAGE_SIZE {
        //                 return PcapUICommand::None;
        //             }
        //             let start = self.state.list.start + len;
    
        //             if start < total {
        //                 let len = std::cmp::min(total - start, PAGE_SIZE);
        //                 return PcapUICommand::ConnectionList(index, start, len);
        //             }
        //             return PcapUICommand::None;
        //         }
        //         KeyCode::Left => {
        //             if self.state.list.start == 0 {
        //                 return PcapUICommand::None;
        //             }
        //             let _pre = std::cmp::min(self.state.list.start, PAGE_SIZE);
        //             let start = self.state.list.start - _pre;
        //             return PcapUICommand::ConnectionList(index, start, PAGE_SIZE);
        //         }
        //         _ => {
        //             return PcapUICommand::None;
        //         }
        //     }
        //     return PcapUICommand::Refresh;
        // }
        
        match event.code {
            // KeyCode::Enter => {
            //     if let Some(item) = self.state.list.items.get(self.state.select) {
            //         let key = item.key;
            //         let title = format!("{} -> {}", item.sender, item.receiver);
            //         self.detail = Some((key, title, CustomTableState::new()));
            //         return PcapUICommand::ConnectionList(key, 0, PAGE_SIZE);
            //     }
            // }
            KeyCode::Down => {
                self.state.next();
            }
            KeyCode::Up => {
                self.state.previous();
            }
            KeyCode::Right => {
                let total = self.state.list.total;
                let len = self.state.list.items.len();
                if total == 0 || len < PAGE_SIZE {
                    return PcapUICommand::None;
                }
                let start = self.state.list.start + len;

                if start < total {
                    let len = std::cmp::min(total - start, PAGE_SIZE);
                    return PcapUICommand::HttpConnectionList(start, len);
                }
                return PcapUICommand::None;
            }
            KeyCode::Left => {
                if self.state.list.start == 0 {
                    return PcapUICommand::None;
                }
                let _pre = std::cmp::min(self.state.list.start, PAGE_SIZE);
                let start = self.state.list.start - _pre;
                return PcapUICommand::HttpConnectionList(start, PAGE_SIZE);
            }
            _ => {
                return PcapUICommand::None;
            }
        }
        PcapUICommand::Refresh
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::Init => PcapUICommand::HttpConnectionList(0, PAGE_SIZE),
            PcapEvent::HttpConnectionList(list) => {
                self.state.update(list);
                PcapUICommand::Refresh
            }
            // PcapEvent::ConnectionList(list) => {
            //     if let Some((key, title, mut state)) = self.detail.take() {
            //         state.update(list);
            //         self.detail = Some((key, title, state));
            //         return PcapUICommand::Refresh;
            //     }
            //     PcapUICommand::None
            //     // let key = self.detail.as_ref().unwrap().0;
            //     // self.detail.as_mut().unwrap().1.update(list);
            //     // PcapUICommand::Refresh
            // }
            _ => PcapUICommand::None,
        }
    }
}
