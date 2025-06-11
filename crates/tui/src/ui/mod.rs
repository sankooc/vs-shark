use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use enum_dispatch::enum_dispatch;
use ratatui::{buffer::Buffer, layout::Rect};
use window::MainUI;

use crate::{
    engine::{PcapEvent, PcapUICommand},
};

// use crate::loading;

mod frames;
mod hex;
mod popup;
mod stack;
mod window;
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
    Frame(frames::App)
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
