use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{
    concept::{VConnection, VConversation},
    util::format_bytes_single_unit_int,
};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Rect}, widgets::{Block, Widget}
};

use crate::{
    engine::{PcapEvent, PcapUICommand}, ui::{loading, render_table, ControlState, CustomTableState, TableStyle}
};

#[derive(Default)]
pub struct Conversation {
    state: CustomTableState<VConversation>,
    detail: Option<(usize, String, CustomTableState<VConnection>)>,
}
pub struct ConversationStyle;
impl TableStyle<VConversation> for ConversationStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_header_style()
    }

    fn get_row_style(&self, _: &VConversation, status: usize) -> ratatui::prelude::Style {
        match status {
            1 => crate::theme::BLANK_FROZEN,
            _ => crate::theme::BLANK
        }
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["", "Count", "Sender", "Receiver", "Packets", "Bytes", "TX Packets", "RX Packets", "TX Bytes", "RX Bytes"]
    }

    fn get_row(&self, data: &VConversation, selected: bool) -> Vec<String> {
        let tx_p = data.receiver_packets;
        let tx_b = data.receiver_bytes;
        let rx_p = data.sender_packets;
        let rx_b = data.sender_bytes;
        vec![
            if selected { "⏎".into() } else { "".into() },
            format!("{}", data.connects),
            data.sender.clone(),
            data.receiver.clone(),
            format!("{}", tx_p + rx_p),
            format_bytes_single_unit_int((tx_b + rx_b) as usize),
            format!("{}", tx_p),
            format!("{}", rx_p),
            format_bytes_single_unit_int(tx_b as usize),
            format_bytes_single_unit_int(rx_b as usize),
        ]
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(1),
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
    fn get_block(&self) -> Option<Block> {
        None
    }
}

pub struct ConnectionStyle;
impl TableStyle<VConnection> for ConnectionStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_header_style()
    }

    fn get_row_style(&self, _: &VConnection, _: usize) -> ratatui::prelude::Style {
        crate::theme::DNS_BG.into()
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["", "Protocol", "S-port", "R-port", "TX-Packets", "TX-Bytes", "TX-Used", "RX-Packets", "RX-Bytes", "RX-Used"]
    }

    fn get_row(&self, data: &VConnection, selected: bool) -> Vec<String> {
        vec![
            if selected { "⌫".into() } else { "".into() },
            data.protocol.clone(),
            format!("{}", data.primary.port),
            format!("{}", data.second.port),
            format!("{}", data.primary.statistic.count),
            format_bytes_single_unit_int(data.primary.statistic.throughput as usize),
            format_bytes_single_unit_int(data.primary.statistic.clean_throughput as usize),
            format!("{}", data.second.statistic.count),
            format_bytes_single_unit_int(data.second.statistic.throughput as usize),
            format_bytes_single_unit_int(data.second.statistic.clean_throughput as usize),
        ]
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(2),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Min(10),
            Constraint::Min(12),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(12),
            Constraint::Min(12),
        ]
    }
    fn get_block(&self) -> Option<ratatui::widgets::Block> {
        Some(super::block::content_border_low())
    }
}

impl Widget for &mut Conversation {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.loading {
            loading::line(area, buf);
            return;
        }
        if let Some((_,_, state)) = &self.detail {
            let rects = ratatui::layout::Layout::vertical([Constraint::Length(8), Constraint::Min(10)]).split(area);
            render_table(ConversationStyle, &self.state, rects[0], buf,1);
            render_table(ConnectionStyle, state, rects[1], buf, 0);
            return;
        }
        render_table(ConversationStyle, &self.state, area, buf, 0);
    }
}

const PAGE_SIZE: usize = 100;

impl ControlState for Conversation {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.state.loading {
            return PcapUICommand::None;
        }
        
        if let Some((key,_, state)) = &mut self.detail {
            let index = *key;
            match event.code {
                KeyCode::Backspace => {
                    self.detail = None;
                }
                KeyCode::Down => {
                    state.to_next();
                }
                KeyCode::Up => {
                    state.previous();
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
                        return PcapUICommand::ConnectionList(index, start, len);
                    }
                    return PcapUICommand::None;
                }
                KeyCode::Left => {
                    if self.state.list.start == 0 {
                        return PcapUICommand::None;
                    }
                    let _pre = std::cmp::min(self.state.list.start, PAGE_SIZE);
                    let start = self.state.list.start - _pre;
                    return PcapUICommand::ConnectionList(index, start, PAGE_SIZE);
                }
                _ => {
                    return PcapUICommand::None;
                }
            }
            return PcapUICommand::Refresh;
        }
        
        match event.code {
            KeyCode::Enter => {
                if let Some(item) = self.state.list.items.get(self.state.select) {
                    let key = item.key;
                    let title = format!("{} -> {}", item.sender, item.receiver);
                    self.detail = Some((key, title, CustomTableState::default()));
                    return PcapUICommand::ConnectionList(key, 0, PAGE_SIZE);
                }
            }
            KeyCode::Down => {
                self.state.to_next();
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
            PcapEvent::Init => PcapUICommand::ConversationList(0, PAGE_SIZE),
            PcapEvent::ConversationList(list) => {
                self.state.update(list);
                PcapUICommand::Refresh
            }
            PcapEvent::ConnectionList(list) => {
                if let Some((key, title, mut state)) = self.detail.take() {
                    state.update(list);
                    self.detail = Some((key, title, state));
                    return PcapUICommand::Refresh;
                }
                PcapUICommand::None
                // let key = self.detail.as_ref().unwrap().0;
                // self.detail.as_mut().unwrap().1.update(list);
                // PcapUICommand::Refresh
            }
            _ => PcapUICommand::None,
        }
    }
}
