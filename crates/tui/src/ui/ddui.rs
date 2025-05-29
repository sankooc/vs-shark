use std::{fmt::Display, time::Duration};

use crate::{
    theme::{get_active_tab_color, get_color, get_protocol_color, ACTIVE_TAB_COLOR},
    ControlPanel,
};

use super::Result;

use crossterm::event::{self, KeyModifiers};
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
// use shark::common::base::Instance;

enum AppState {
    RUNNING,
    QUIT,
}

pub struct MainUI {
    selected: u8,
    state: AppState,
    // overview_page: super::overview::App,
    frame_page: super::frames::App,
    // tcp_page: super::tcp::TCPList,
}

impl MainUI {
    pub fn new() -> Self {
        let frame_page = super::frames::App::new();
        // let overview_page = super::overview::App::new(instance.clone());
        // let tcp_page = super::tcp::TCPList::new(instance.clone());
        // let stack_page = super::stack::StackView::new();
        Self {
            // instance,
            state: AppState::RUNNING,
            // tcp_page,
            // overview_page,
            frame_page,
            selected: 0,
        }
    }
    pub fn update(&mut self, terminal: &mut DefaultTerminal, _event: Option<Event>) -> Result<()> {
        self.handle_events(_event)?;
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }
    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        loop {
            let next_event = self.get_event();
            if let AppState::QUIT = self.state {
                break;
            }
            self.update(&mut terminal, next_event).unwrap();
        }
        ratatui::restore();
    }
    pub fn get_event(&mut self) -> Option<Event> {
        let timeout = Duration::from_secs_f32(1.0 / 10.0);
        let mut _event: Option<Event> = None;
        if let Ok(_get) = event::poll(timeout) {
            if _get {
                if let Ok(_key) = event::read() {
                    if let Event::Key(key) = &_key {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::QUIT,
                                _ => {}
                            }
                        }
                    }
                    _event = Some(_key);
                }
            }
        }
        _event
    }
    
    fn get_view(&mut self) -> &mut dyn ControlPanel {
        match self.selected {
            _ => &mut self.frame_page,
            // 2 => &mut self.tcp_page,
            // _ => &mut self.overview_page,
        }
    }

    // fn get_wigit(&mut self) -> Box<&dyn Widget> {
    //     match self.selected {
    //         1 => Box::new(&mut self.frame_page),
    //         2 => Box::new(&mut self.tcp_page),
    //         _ => Box::new(&mut self.overview_page)
    //     }
    // }

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
                            _ => self.frame_page.control(_event),
                        }
                    }
                    return Ok(());
                }
                self.get_view().control(_event);
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        if self.selected < 2 {
            self.selected += 1;
        }
    }

    pub fn previous_tab(&mut self) {
        if self.selected >= 1 {
            self.selected -= 1;
        }
    }

    pub fn quit(&mut self) {}
    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        match self.selected {
            1 => Line::raw("◄ ► to change page | SHIFT+(▲ ▼) to change panel | Press q or ESC to quit").centered().render(area, buf),
            // 2 => Line::raw("◄ ► to change page | SHIFT+(▲ ▼) to change panel | Press q or ESC to quit").centered().render(area, buf),
            _ => Line::raw("SHIFT+(◄ ►) to change tab | Press q or ESC to quit").centered().render(area, buf),
        }
    }
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
            _ => {
                self.frame_page.render(_inner_area, buf);
            }
            // 2 => {
            //     self.tcp_page.render(_inner_area, buf);
            // }
            // _ => {
            //     self.overview_page.render(_inner_area, buf);
            // }
        }
        self.render_footer(footer_area, buf);
    }
}

impl MainUI {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = ["Overview", "Frames", "TCP"].iter().map(create_tab_title);
        let selected_tab_index = self.selected as usize;
        Tabs::new(titles).highlight_style(get_active_tab_color()).select(selected_tab_index).padding("", "").divider(" ").render(area, buf);
    }
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
