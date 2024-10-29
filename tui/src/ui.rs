use std::{fmt::Display, rc::Rc};

use crate::{
    theme::{get_active_tab_color, get_color, get_protocol_color, ACTIVE_TAB_COLOR},
    ControlPanel,
};

use super::Result;

use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Styled, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Padding, Tabs, Widget},
    DefaultTerminal,
};
use shark::common::base::Instance;

pub struct MainUI {
    selected: u8,
    overview_page: super::overview::App,
    frame_page: super::table::App
    // instance: Rc<Instance>,
}

impl MainUI {
    pub fn new(instance: Rc<Instance>) -> Self {
        let frame_page = super::table::App::new(instance.clone());
        let overview_page = super::overview::App::new(instance.clone());
        // let stack_page = super::stack::StackView::new();
        Self {
            // instance,
            overview_page: overview_page,
            frame_page,
            selected: 0,
        }
    }
    pub fn update(&mut self, terminal: &mut DefaultTerminal, _event: Option<Event>) -> Result<()> {
        self.handle_events(_event)?;
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn _add_pop(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().blue();
        block.render(area, buf);
        let block = Block::bordered().padding(Padding::left(10)).border_type(BorderType::Rounded).title("Alert").style(get_protocol_color("tcp"));
        let area = _popup_area(area, 60, 20);
        block.render(area, buf);
    }
    fn handle_events(&mut self, event: Option<Event>) -> std::io::Result<()> {
        if let Some(_event) = &event {
            if let Event::Key(key) = _event {
                let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                if shift_pressed {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),
                            _ => self.frame_page.control(_event)
                        }
                    }
                    return Ok(());
                }
            }
            
            self.frame_page.control(_event);
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

impl Widget for &mut MainUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // self.add_pop(area, buf);
        // return;
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
                self.frame_page.render(_inner_area, buf);
            }
            _ => {
                self.overview_page.render(_inner_area, buf);
            }
        }
        render_footer(footer_area, buf);
    }
}

impl MainUI {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = ["Overview", "Frames"].iter().map(create_tab_title);
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

fn _popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
