use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{concept::VConversation, util::format_bytes_single_unit_int};
use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, widgets::Widget};

use crate::{engine::{PcapEvent, PcapUICommand}, ui::{loading, render_table, ControlState, CustomTableState, TableStyle}};


pub struct Conversation {
    state: CustomTableState<VConversation>,    
}
pub struct ConversationStyle;
impl TableStyle<VConversation> for ConversationStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_header_style()
    }

    fn get_row_style(&self, _: &VConversation) -> ratatui::prelude::Style {
        crate::theme::DNS_BG.into()
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["Count", "Sender", "Receiver", "Packets", "Bytes", "TX Packets", "RX Packets", "TX Bytes", "RX Bytes"]
    }

    fn get_row(&self, data: &VConversation) -> Vec<String> {
        let tx_p = data.receiver_packets;
        let tx_b = data.receiver_bytes;
        let rx_p = data.sender_packets;
        let rx_b = data.sender_bytes;
        vec![format!("{}", data.connects),
        data.sender.clone(), data.receiver.clone(), 
        format!("{}", tx_p + rx_p), format_bytes_single_unit_int((tx_b + rx_b) as usize), 
        format!("{}", tx_p), format!("{}", rx_p), 
        format_bytes_single_unit_int(tx_b as usize), format_bytes_single_unit_int(rx_b as usize)]
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(6),
            Constraint::Min(15),
            Constraint::Min(15),
            Constraint::Min(10),
            Constraint::Min(12),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(12),
            Constraint::Min(12),
        ]
    }
}

impl Conversation {
    pub fn new() -> Self {
        Self {state: CustomTableState::new()}
    }
}

impl Widget for &mut Conversation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.loading {
            loading::line("Loading conversation...", area, buf);
            return;
        }
        render_table(ConversationStyle, &self.state, area, buf);
    }
}

const PAGE_SIZE: usize = 10;

impl ControlState for Conversation {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.state.loading {
            return PcapUICommand::None;
        }
        match event.code {
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
                    return PcapUICommand::ConversationList(start, len);
                }
                return PcapUICommand::None;
            }
            KeyCode::Left => {
                if self.state.list.start == 0 {
                    return PcapUICommand::None;
                }
                let _pre = std::cmp::min(self.state.list.start, PAGE_SIZE);
                let start = self.state.list.start - _pre;
                return PcapUICommand::ConversationList(start, PAGE_SIZE);
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
            PcapEvent::Init => {
                PcapUICommand::ConversationList(0, PAGE_SIZE)
            }
            PcapEvent::ConversationList(list) => {
                self.state.update(list);
                PcapUICommand::Refresh
            }
            _ => PcapUICommand::None,
        }
    }
}