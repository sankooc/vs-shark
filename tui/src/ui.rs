use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    theme::{get_active_tab_color, get_color, ACTIVE_TAB_COLOR},
    ControlPanel,
};

use super::Result;

use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Styled,
    symbols,
    text::Line,
    widgets::{Block, Padding, Tabs, Widget},
    DefaultTerminal,
};
use shark::common::base::Instance;

pub struct MainUi {
    selected: u8,
    overview_page: RefCell<super::overview::App>,
    frame_page: RefCell<super::table::App>,
    // instance: Rc<Instance>,
}

impl MainUi {
    pub fn new(instance: Rc<Instance>) -> Self {
        let frame_page = super::table::App::new(instance.clone());
        let overview_page = super::overview::App::new(instance.clone());
        Self {
            // instance,
            overview_page: RefCell::new(overview_page),
            frame_page: RefCell::new(frame_page),
            selected: 0,
        }
    }
    pub fn update(&mut self, terminal: &mut DefaultTerminal, _event: Option<Event>) -> Result<()> {
        self.handle_events(_event)?;
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> std::io::Result<()> {
        if let Some(_event) = &event {
            if let Event::Key(key) = _event {
                let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                if shift_pressed {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                            KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                            _ => {}
                        }
                    }
                    return Ok(());
                }
            }
            let mut page = self.frame_page.borrow_mut();
            page.control(_event);
            drop(page);
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        if self.selected < 1 {
            self.selected += 1;
        }
    }

    pub fn previous_tab(&mut self) {
        if self.selected >= 1 {
            self.selected -= 1;
        }
    }

    pub fn quit(&mut self) {}
}

impl Widget for &mut MainUi {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, _] = horizontal.areas(header_area);

        self.render_tabs(tabs_area, buf);
        let block = Block::bordered().border_set(symbols::border::QUADRANT_OUTSIDE).padding(Padding::ZERO).border_style(ACTIVE_TAB_COLOR);
        let _inner_area = block.inner(inner_area);
        block.render(inner_area, buf);

        match self.selected {
            1 => {
                let mut page = self.frame_page.borrow_mut();
                page.render(_inner_area, buf);
                drop(page);
            }
            _ => {
                let mut page = self.overview_page.borrow_mut();
                page.render(_inner_area, buf);
                drop(page);
            }
        }
        render_footer(footer_area, buf);
    }
}

impl MainUi {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = ["Overview", "Frames", "Conversation"].iter().map(create_tab_title);
        let selected_tab_index = self.selected as usize;
        Tabs::new(titles).highlight_style(get_active_tab_color()).select(selected_tab_index).padding("", "").divider(" ").render(area, buf);
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("SHIFT+(◄ ►) to change tab | Press q or ESC to quit").centered().render(area, buf);
}

fn create_tab_title(title: impl Display) -> Line<'static> {
    format!("  {}  ", title).set_style(get_color("tab")).into()
}
