use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use enum_dispatch::enum_dispatch;
use pcap::common::concept::ListResult;
use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::{Modifier, Style, Stylize}, symbols, widgets::{Block, Cell, HighlightSpacing, Padding, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget}};
use window::MainUI;

use crate::{
    engine::{PcapEvent, PcapUICommand}, theme::ICMPV6_FG,
};

// use crate::loading;

mod frames;
mod hex;
mod popup;
mod stack;
mod window;
mod conversation;
mod loading;

pub struct UI {
    sender: Sender<PcapUICommand>,
    receiver: Receiver<PcapEvent>,
}

fn try_handle_event(app: &mut MainUI) -> PcapUICommand {
    if event::poll(Duration::from_millis(10)).unwrap() {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                match key.kind {
                    KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return PcapUICommand::Quit;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                return app.control(shift_pressed, key);
            }
        }
    }
    PcapUICommand::None
}


#[enum_dispatch]
pub enum TabContainer {
    Frame(frames::App),
    Conversation(conversation::Conversation)
}

#[enum_dispatch(TabContainer)]
pub trait ControlState {
    fn control(&mut self, shift_pressed: bool, event: KeyEvent) -> PcapUICommand;
    fn do_render(&mut self, area: Rect, buf: &mut Buffer);
    fn update(&mut self, event: PcapEvent) -> PcapUICommand;
}

impl UI {
    pub fn new(sender: Sender<PcapUICommand>, receiver: Receiver<PcapEvent>) -> Self {
        Self { sender, receiver }
    }
    pub fn run(&self) -> anyhow::Result<()> {
        let mut app = MainUI::new();
        let mut terminal = ratatui::init();
        let mut quiting = false;
        terminal.draw(|f| f.render_widget(&mut app, f.area())).unwrap();
        loop {
            let cmd = try_handle_event(&mut app);
            match &cmd {
                PcapUICommand::Quit => {
                    self.sender.send(cmd).unwrap();
                    break;
                }
                PcapUICommand::None => {
                }
                PcapUICommand::Refresh => {
                    terminal.draw(|f| f.render_widget(&mut app, f.area())).unwrap();
                    continue;
                }
                _ => {
                    terminal.draw(|f| f.render_widget(&mut app, f.area())).unwrap();
                    self.sender.send(cmd).unwrap();
                    continue;
                }
            }

            let react = match self.receiver.try_recv() {
                Ok(event) => {
                    if let PcapEvent::Quit = event {
                        quiting = true;
                        // println!("failed to parse file");
                        break;
                    }
                    app.update(event)
                }
                _ => PcapUICommand::None,
            };
            match &react {
                PcapUICommand::Quit => {
                    self.sender.send(react).unwrap();
                    break;
                }
                PcapUICommand::Refresh => {
                    terminal.draw(|f| f.render_widget(&mut app, f.area())).unwrap();
                }
                PcapUICommand::None => {
                    continue;
                }
                _ => {
                    self.sender.send(react).unwrap();
                    continue;
                }
            }
        }
        while quiting {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        match key.kind {
                            KeyEventKind::Press => match key.code {
                                _ => {
                                    let _ = self.sender.send(PcapUICommand::Quit);
                                    break;
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            let modal = popup::Modal::default();
            terminal.draw(|f| f.render_widget(modal, f.area())).unwrap();
        }
        ratatui::restore();
        Ok(())
    }
}



const ITEM_HEIGHT: usize = 1;
pub struct CustomTableState<T> {
    loading: bool,
    list: ListResult<T>,
    select: usize,
}
impl<T> CustomTableState<T> {
    pub fn new() -> Self {
        Self {
            loading: true,
            list: ListResult::empty(),
            select: 0,
        }
    }
    pub fn update(&mut self, list: ListResult<T>){
        self.list = list;
        self.select = 0;
        self.loading = false;
    }
    pub fn get_selection(&self) -> TableState {
        TableState::default().with_selected(self.select)
    }
    pub fn scroll_state(&self) -> ScrollbarState {
        let ss = ScrollbarState::new(self.list.items.len() * ITEM_HEIGHT);
        ss.position(self.select * ITEM_HEIGHT)
    }
    pub fn next(&mut self) -> usize {
        if self.list.items.len() > 0 && self.select < self.list.items.len() - 1 {
            self.select += 1;
        }
        self.select
    }
    pub fn previous(&mut self) -> usize {
        if self.list.items.len() > 0 && self.select > 0 {
            self.select -= 1;
        }
        self.select
    }
}
    


pub trait TableStyle<T> {
    fn get_header_style(&self) -> Style;
    fn get_row_style(&self, data: &T) -> Style;
    fn get_select_style(&self) -> Style;
    fn get_cols(&self) -> Vec<&str>;
    fn get_row(&self, data: &T) -> Vec<String>;
    fn get_row_width(&self) -> Vec<Constraint>;
}
pub fn render_table<T>(ts: impl TableStyle<T>,state: &CustomTableState<T>, _area: Rect, buf: &mut Buffer) {
    
    let block = Block::bordered()
    .border_set(symbols::border::PLAIN)
    .padding(Padding::new(0, 0, 0, 0))
    .border_style(ICMPV6_FG);
    let area = block.inner(_area);
    block.render(_area, buf);

    let header_style = ts.get_header_style();
    let cols = ts.get_cols();
    let header = cols.into_iter().map(Cell::from).collect::<Row>().style(header_style).height(1);
    let frames = &state.list.items;
    let rows = frames.iter().map(|data| {
        let rs: Vec<Cell> = ts.get_row(data).iter().map(|s| s.clone().into()).collect();
        let row_style = ts.get_row_style(data);
        rs.into_iter().collect::<Row>().add_modifier(Modifier::BOLD).style(row_style).height(1)
    });

    let select_row_style = ts.get_select_style();
    let t: Table<'_> = Table::new(
        rows,
        ts.get_row_width(),
    )
    .header(header)
    .highlight_style(select_row_style)
    .highlight_spacing(HighlightSpacing::Always);
    let mut t_area = area.clone();
    t_area.width -= 1;
    StatefulWidget::render(t, t_area, buf, &mut state.get_selection());
    let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    scroll.render(area, buf, &mut state.scroll_state());
}
    
