use crate::{
    engine::{PcapEvent, PcapUICommand},
    theme::GRUVBOX_FG, ui::{loading, ControlState, TabContainer},
};
use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::concept::ProgressStatus;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use super::frames;

pub struct MainUI {
    progress: Option<ProgressStatus>,
    container: TabContainer,
    active_tab: usize,
}

impl Widget for &mut MainUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let [inner_area, footer_area] = Layout::vertical([Min(0), Length(1)]).areas(area);

        self.render_main_view(inner_area, buf);

        self.render_status_bar(footer_area, buf);
    }
}

impl MainUI {
    pub fn new() -> Self {
        Self {
            container: TabContainer::Frame(frames::App::new()),
            active_tab: 0,
            progress: None,
            // loaded: false,
        }
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

}

impl ControlState for MainUI {
    fn control(&mut self, shift_pressed: bool, event: KeyEvent) -> PcapUICommand {
        if shift_pressed {
            match event.code {
                KeyCode::Left => {
                    if self.active_tab < 1 {
                        self.active_tab = 0;
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
                    PcapUICommand::FrameList(0, 100)
                } else {
                    self.progress = Some(status);
                    PcapUICommand::None
                }
                // self.progress = Some(status);
            },
            // PcapEvent::FrameList(list) => {
            //     // let index = list.items.get(0).unwrap().index;
            //     // self.frame_data = Some(FrameState::new(list));
            //     PcapUICommand::None
            // },
            // PcapEvent::FrameData(fields, ds,  extra) => {
            //     if let Some(frame_data) = &mut self.frame_data {
            //         frame_data.field = Some(StackState::new(fields, ds, extra));
            //     }
            //     PcapUICommand::None
            // },
            _ => self.container.update(event)
        }
    }
}

fn format_bytes_single_unit_int(bytes: usize) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes;
    let mut low = 0;
    let mut unit_index = 0;

    while size >= 1024 && unit_index < UNITS.len() - 1 {
        low = size % 1024;
        size /= 1024;
        unit_index += 1;
    }

    format!("{}.{} {}", size, low, UNITS[unit_index])
}
