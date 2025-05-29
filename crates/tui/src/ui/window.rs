use crate::theme::{get_active_tab_color, get_header_style, panel_color, ACTIVE_TAB_COLOR, GRUVBOX_FG};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Tabs, Widget},
};

use super::{frames, store::Store};

pub struct MainUI<'a> {
    store: &'a Store<'a>,
    active_tab: usize,
}

impl Widget for &MainUI<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Min(0), Length(1)]);
        let [inner_area, footer_area] = vertical.areas(area);

        self.render_main_view(inner_area, buf);

        self.render_status_bar(footer_area, buf);
    }
}

impl<'a> MainUI<'a> {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = vec!["概览", "数据包", "流量", "协议", "设置"];

        let vertical_padding = (area.height - 1) / 2;

        let tabs = Tabs::new(titles.iter().cloned().map(Line::from).collect::<Vec<_>>())
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::TOP)
                    .style(get_header_style())
                    .padding(Padding::new(vertical_padding, 0, 0, 0)),
            ) 
            .highlight_style(get_active_tab_color())
            .select(self.active_tab);

        tabs.render(area, buf);
    }

    fn render_main_view(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(symbols::border::ROUNDED)
            .padding(Padding::new(0, 0, 0, 0))
            .border_style(ACTIVE_TAB_COLOR);

        let inner_area = block.inner(area);
        block.render(area, buf);
        if let Some(_) = &self.store.progress {
            // loading::App::default().render(inner_area, buf);
           self.render_packets(inner_area, buf)
        } else {
           self.render_overview(inner_area, buf)
        }
    }

    fn render_status_bar(&self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let horizontal = Layout::horizontal([Min(0), Length(45)]);
        let [left_area, right_area] = horizontal.areas(area);
        
        let left_text = vec![
            Span::styled("◄ ► to change page | SHIFT+(▲ ▼) to change panel | Press q or ESC to quit", Style::default().fg(Color::Green)),
        ];

        let left_paragraph = Paragraph::new(Line::from(left_text)).block(Block::default()).alignment(Alignment::Left);
        let right_text = match &self.store.progress {
            Some(progress) => {
                let str = format!("{}/{} total: {}", format_bytes_single_unit_int(progress.cursor), format_bytes_single_unit_int(progress.total), progress.count);
                vec![Span::styled(str, Style::default().fg(GRUVBOX_FG))]
            }
            None => {
                vec![
                    Span::styled("", Style::default().fg(GRUVBOX_FG))
                    // Span::styled("q", Style::default().fg(Color::LightRed)),
                    // Span::styled(" 退出", Style::default().fg(GRUVBOX_FG)),
                ]
            }
        };

        let right_paragraph = Paragraph::new(Line::from(right_text)).block(Block::default()).alignment(Alignment::Right);

        left_paragraph.render(left_area, buf);
        right_paragraph.render(right_area, buf);
    }

    fn render_overview(&self, area: Rect, buf: &mut Buffer) {
        let text = Text::from("Loading...");
        let paragraph = Paragraph::new(text).alignment(Alignment::Center).style(panel_color());
        paragraph.render(area, buf);
    }

    fn render_packets(&self, area: Rect, buf: &mut Buffer) {
        if let Some(state) = &self.store.frame_data {
            let mut frame_view: frames::App = state.into();
            frame_view.render(area, buf);
            return;
        }
        let text = Text::from("加载中");
        let paragraph = Paragraph::new(text).alignment(Alignment::Center).style(panel_color());
        paragraph.render(area, buf);
    }

}

impl<'a> From<&'a Store<'a>> for MainUI<'a> {
    fn from(store: &'a Store<'a>) -> MainUI<'a> {
        MainUI {
            store,
            active_tab: 0,
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