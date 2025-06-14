
use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, text::Text, widgets::{Block, Padding, Paragraph, Widget}};

use crate::theme::{panel_color, POSITIVE_STYLE};

pub fn line(text: &str, area: Rect, buf: &mut Buffer) {
    let text = Text::from(text);
    let paragraph = Paragraph::new(text).alignment(Alignment::Center).style(panel_color());
    paragraph.render(area, buf);
}

pub fn main_block(_area: Rect, buf: &mut Buffer) -> Rect {
    let block = Block::new().padding(Padding::new(0, 0, 0, 0))
        .style(POSITIVE_STYLE);
    let area = block.inner(_area);
    block.render(_area, buf);
    area
}