use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Widget};

#[derive(Default)]
pub struct Modal {}

use Constraint::{Length, Min};
impl Widget for Modal {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().on_blue();
        let inner_area = block.inner(area);
        block.render(area, buf);

        let layout = Layout::vertical([Length(5), Min(0)]);
        let [_, _area] = layout.areas(inner_area);
        let text = "Failed to parse file, press any key to quit";
        let paragraph = Paragraph::new(text.slow_blink()).alignment(Alignment::Center);
        paragraph.render(_area, buf);
    }
}