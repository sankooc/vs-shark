use ratatui::widgets::{Block, Borders, Padding};

use crate::theme::{BLANK, REVERT_STYLE};

const CONTENT_BORDER: Block = Block::bordered()
.padding(Padding::new(0, 0, 0, 0))
.border_set(ratatui::symbols::border::QUADRANT_OUTSIDE);

pub fn content_border() -> Block<'static> {
    CONTENT_BORDER.border_style(REVERT_STYLE).style(BLANK)
}

pub fn content_border_low() -> Block<'static> {
    Block::new().borders(Borders::TOP)
        .padding(Padding::new(0, 0, 0, 0))
        .border_set(ratatui::symbols::border::PLAIN)
        .border_style(REVERT_STYLE).style(BLANK)
}
pub fn content_border_right() -> Block<'static> {
    Block::new().borders(Borders::TOP | Borders::LEFT)
        .padding(Padding::new(0, 0, 0, 0))
        .border_set(ratatui::symbols::border::PLAIN)
        .border_style(REVERT_STYLE).style(BLANK)
}