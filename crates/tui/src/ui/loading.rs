
use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, text::Line, widgets::{Paragraph, Widget}};

use crate::ui::block::content_border;

pub fn line(area: Rect, buf: &mut Buffer) {
    let lines = vec![
        Line::from(r"  _                            _         _       _             "),
        Line::from(r" | |      ___    __ _    __| |  ___  | |_    (_)  _ __     __ _ "),
        Line::from(r" | |     / _ \  / _` |  / _` | / _ \ | __|   | | | '_ \   / _` |"),
        Line::from(r" | |___ |  __/ | (_| | | (_| ||  __/ | |_    | | | | | | | (_| |"),
        Line::from(r" |_____| \___|  \__,_|  \__,_| \___|  \__|  _/ | |_| |_|  \__, |"),
        Line::from(r"                                         |__/            |___/ "),
    ];
    // lines.push(Line::from(r"  _                           _   _                 "));
    // lines.push(Line::from(r" | |       ___     __ _    __| | (_)  _ __     __ _ "));
    // lines.push(Line::from(r" | |      / _ \   / _` |  / _` | | | | '_ \   / _` |"));
    // lines.push(Line::from(r" | |___  | (_) | | (_| | | (_| | | | | | | | | (_| |"));
    // lines.push(Line::from(r" |_____|  \___/   \__,_|  \__,_| |_| |_| |_|  \__, |"));
    // lines.push(Line::from(r"                                              |___/ "));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),   
            Constraint::Length(lines.len() as u16), 
            Constraint::Fill(1),
        ])
        .split(area);
    Paragraph::new(lines).centered().render(chunks[1], buf);
}

pub fn main_block(_area: Rect, buf: &mut Buffer) -> Rect {
    let block = content_border();
    let area = block.inner(_area);
    block.render(_area, buf);
    area
}