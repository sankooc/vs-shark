use std::time::Duration;
use super::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{ Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Gauge, Padding, Widget},
    DefaultTerminal,
};

const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
pub struct App {
    state: AppState,
    progress4: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    // Running,
    #[default]
    Started,
    Quitting,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state != AppState::Quitting {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }
        Ok(())
    }

    pub fn update(&mut self, _: u16) {
        if self.state != AppState::Started {
            return;
        }
        self.progress4 = (self.progress4 + 1.0).clamp(0.0, 100.0);
    }

    pub fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') | KeyCode::Enter => self.start(),
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn start(&mut self) {
        self.state = AppState::Started;
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
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
        let label = format!("{:.1}%", self.progress4);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress4 / 100.0)
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