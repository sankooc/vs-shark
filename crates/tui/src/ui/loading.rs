
use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, text::Text, widgets::{Paragraph, Widget}};

use crate::theme::panel_color;

pub fn line(text: &str, area: Rect, buf: &mut Buffer) {
    let text = Text::from(text);
    let paragraph = Paragraph::new(text).alignment(Alignment::Center).style(panel_color());
    paragraph.render(area, buf);
}