use ratatui::widgets::{Block, Padding, Paragraph, Widget};


pub struct HexView {
}

impl HexView {
    pub fn new() -> Self {
        Self{}
    }
}

impl Widget for &mut HexView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
            let block = Block::bordered().padding(Padding::ZERO);
            let ext = block.inner(area);
            block.render(area, buf);
            let _top = Paragraph::new("Doadls");
            _top.render(ext, buf);
    }
}