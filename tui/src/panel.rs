use color_eyre::owo_colors::OwoColorize;
use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Padding, Paragraph, Widget}};

use crate::theme::panel_color;



pub struct Panel {
    label: String,
    val: String,
}

impl Panel {
    pub fn new(label: &str, val: &str) -> Self {
        Self{label:label.into(), val: val.into()}
    }
}

impl Widget for &mut Panel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {

            let block = Block::bordered().padding(Padding::ZERO);
            let ext = block.inner(area);
            block.render(area, buf);
            let [top, bottom] = Layout::vertical([Constraint::Length(1), Constraint::Length(2)]).areas(ext);
            let _top = Paragraph::new(format!(" {}: ",self.label));
            _ = _top.bold();
            _top.render(top, buf);
            let _bt = Paragraph::new(format!(" {}",self.val)).style(panel_color());
            _bt.render(bottom, buf);
    }
}