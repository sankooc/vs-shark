use std::{
    sync::{mpsc::Receiver, mpsc::Sender},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use frames::SelectPanel;
use window::MainUI;

use crate::engine::{PcapCommand, PcapEvent};

// use crate::loading;

mod frames;
mod hex;
mod popup;
mod stack;
mod store;
mod window;

pub struct UI {
    sender: Sender<PcapCommand>,
    receiver: Receiver<PcapEvent>,
}

impl UI {
    pub fn new(sender: Sender<PcapCommand>, receiver: Receiver<PcapEvent>) -> Self {
        Self { sender, receiver }
    }
    pub fn run(&self) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();
        let mut store = store::Store::default();
        let mut quiting = false;
        loop {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                        if shift_pressed {
                            match key.code {
                                KeyCode::Up => {
                                    store.select_panel(SelectPanel::LIST);
                                }
                                _ => {}
                            }
                        } else {
                            match key.kind {
                                KeyEventKind::Press => match key.code {
                                    KeyCode::Char('q') | KeyCode::Esc => {
                                        self.sender.send(PcapCommand::Quit).unwrap();
                                        break;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }

                            if let Some(mut sel) = store.selection() {
                                let cmd = sel.as_mut().control(shift_pressed, key);
                                match &cmd {
                                    PcapCommand::None => {}
                                    _ => {
                                        self.sender.send(cmd).unwrap();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match self.receiver.try_recv() {
                Ok(event) => {
                    if let PcapEvent::Quit = event {
                        quiting = true;
                        // println!("failed to parse file");
                        break;
                    }
                    let cmd = store.update(event);
                    match &cmd {
                        PcapCommand::None => {}
                        _ => {
                            self.sender.send(cmd).unwrap();
                        }
                    }
                }
                _ => {
                    // println!("no event");
                }
            }
            let mut app = MainUI::from(&mut store);
            terminal.draw(|f| f.render_widget(&mut app, f.area())).unwrap();
        }
        while quiting {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        match key.kind {
                            KeyEventKind::Press => match key.code {
                                _ => {
                                    let _ = self.sender.send(PcapCommand::Quit);
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
