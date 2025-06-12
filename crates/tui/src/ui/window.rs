use std::fmt::Display;

use crate::{
    engine::{PcapEvent, PcapUICommand},
    theme::{get_active_tab_color, get_color, ACTIVE_TAB_COLOR, GRUVBOX_FG}, ui::{conversation, loading, ControlState, TabContainer},
};
use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{concept::ProgressStatus, util::format_bytes_single_unit_int};
use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Style, Styled}, symbols, text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph, Tabs, Widget}
};

use super::frames;

const TAB_NAMES: [&str; 2] = ["Frame", "Conversation"];

pub struct MainUI {
    progress: Option<ProgressStatus>,
    container: TabContainer,
    active_tab: usize,
}

impl Widget for &mut MainUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let [tab_area,inner_area, footer_area] = Layout::vertical([Length(2), Min(0), Length(1)]).areas(area);
        
        let block = Block::new().borders(Borders::BOTTOM).border_set(symbols::border::QUADRANT_INSIDE).padding(Padding::ZERO).border_style(ACTIVE_TAB_COLOR);
        let _inner_area = block.inner(tab_area);
        block.render(tab_area, buf);
        self.render_tab_view(_inner_area, buf);
        self.render_main_view(inner_area, buf);
        self.render_status_bar(footer_area, buf);
    }
}
fn create_tab_title(title: impl Display) -> Line<'static> {
    format!("  {}  ", title).set_style(get_color("tab")).into()
}

impl MainUI {
    pub fn new() -> Self {
        Self {
            container: TabContainer::Frame(frames::App::new()),
            active_tab: 0,
            progress: None,
        }
    }
    fn render_tab_view(&mut self, area: Rect, buf: &mut Buffer){
        let titles = TAB_NAMES.iter().map(create_tab_title);
        let selected_tab_index = self.active_tab;
        Tabs::new(titles).highlight_style(get_active_tab_color()).select(selected_tab_index).padding("", "").divider(" ").render(area, buf);
    }
    fn render_main_view(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(_) = &self.progress {
            self.container.do_render(area, buf);
            return;
        }
        self.render_loading(area, buf);
    }
    fn render_status_bar(&self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let horizontal = Layout::horizontal([Min(0), Length(45)]);
        let [left_area, right_area] = horizontal.areas(area);

        let left_text = vec![Span::styled(
            "◄ ► to change page | SHIFT+(◄ ►) to change tab | Enter/Backspace to Detail/Back | Press q or ESC to quit",
            Style::default().fg(Color::Green),
        )];

        let left_paragraph = Paragraph::new(Line::from(left_text)).block(Block::default()).alignment(Alignment::Left);
        let right_text = match &self.progress {
            Some(progress) => {
                let str = format!(
                    "{}/{} total: {}",
                    format_bytes_single_unit_int(progress.cursor),
                    format_bytes_single_unit_int(progress.total),
                    progress.count
                );
                vec![Span::styled(str, Style::default().fg(GRUVBOX_FG))]
            }
            None => {
                vec![Span::styled("", Style::default().fg(GRUVBOX_FG))]
            }
        };

        let right_paragraph = Paragraph::new(Line::from(right_text)).block(Block::default()).alignment(Alignment::Right);

        left_paragraph.render(left_area, buf);
        right_paragraph.render(right_area, buf);
    }
    fn render_loading(&self, area: Rect, buf: &mut Buffer) {
        loading::line("Waiting for parsing...", area, buf);
    }

    fn tab_select(&mut self, active_tab: usize) -> PcapUICommand {
        match active_tab {
            0 => {
                self.active_tab = active_tab;
                self.container = TabContainer::Frame(frames::App::new());
                self.container.update(PcapEvent::Init)
            },
            1 => {
                self.active_tab = active_tab;
                self.container = TabContainer::Conversation(conversation::Conversation::new());
                self.container.update(PcapEvent::Init)
            },
            _ => {
                PcapUICommand::None
            }
        }
    }
}

impl ControlState for MainUI {
    fn control(&mut self, shift_pressed: bool, event: KeyEvent) -> PcapUICommand {
        if shift_pressed {
            match event.code {
                KeyCode::Left => {
                    if self.active_tab > 0 {
                        return self.tab_select(self.active_tab - 1);
                    }
                }
                KeyCode::Right => {
                    let len = TAB_NAMES.len();
                    if self.active_tab < len - 1 {
                        return self.tab_select(self.active_tab + 1);
                    }
                }
                _ => {
                    return PcapUICommand::None;
                }
            }
            PcapUICommand::Refresh
        } else {
            self.container.control(shift_pressed, event)
        }
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }
    fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::ProgressStatus(status) => {
                if let None = self.progress {
                    self.progress = Some(status);
                    self.container.update(PcapEvent::Init)
                } else {
                    self.progress = Some(status);
                    PcapUICommand::None
                }
            },
            _ => self.container.update(event)
        }
    }
}
