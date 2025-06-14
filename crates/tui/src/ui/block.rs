use ratatui::widgets::{Block, Padding};

use crate::theme::{BLANK, REVERT_STYLE};

const CONTENT_BORDER: Block = Block::bordered()
.padding(Padding::new(0, 0, 0, 0))
.border_set(ratatui::symbols::border::QUADRANT_OUTSIDE);

pub fn content_border() -> Block<'static> {
    CONTENT_BORDER.border_style(REVERT_STYLE).style(BLANK)
}

pub fn content_inner_border() -> Block<'static> {
    CONTENT_BORDER.border_style(REVERT_STYLE).style(BLANK)
    // Block::new().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT).padding(Padding::new(0, 0, 0, 0)).border_set(ratatui::symbols::border::QUADRANT_OUTSIDE).border_style(REVERT_STYLE).style(BLANK)
}
// pub fn content_border_frozen() -> Block<'static> {
//     CONTENT_BORDER.border_style(REVERT_STYLE).style(BLANK_FROZEN)
// }

// pub fn content_inner_border() -> Block<'static> {
//     CONTENT_BORDER.border_style(REVERT_STYLE).style(BLANK)
//     // Block::new().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT).padding(Padding::new(0, 0, 0, 0)).border_set(ratatui::symbols::border::QUADRANT_OUTSIDE).border_style(REVERT_STYLE).style(BLANK)
// }