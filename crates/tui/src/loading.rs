use std::ops::Deref;
use super::Result;
use ratatui::{
    buffer::Buffer,
    layout::{ Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Gauge, Padding, Widget},
    DefaultTerminal,
};

const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;


#[derive(Default)]
pub enum PageState {
    #[default]
    LOADING,
    ERROR,
}
#[derive(Default)]
pub struct App {
    pub progress: f64,
    pub state: PageState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, state: PageState) -> Result<()> {
        self.progress = (self.progress + 1.0).clamp(0.0, 100.0);
        self.state = state;
        terminal.draw(move |frame| frame.render_widget(self.deref(), frame.area()))?;
        Ok(())
    }
}

impl Widget for &App {
    #[allow(clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min, Ratio};
        let layout = Layout::vertical([Length(2), Min(0), Length(1)]);
        let [_, gauge_area, _] = layout.areas(area);

        let layout = Layout::vertical([Ratio(1, 4); 4]);
        let [_, _, _, gauge4_area] = layout.areas(gauge_area);

        self.render_gauge4(gauge4_area, buf);
    }
}

impl App {

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Parsing File...");
        let label = format!("{:.1}%", self.progress);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Line::from(title).centered();
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(CUSTOM_LABEL_COLOR)
}