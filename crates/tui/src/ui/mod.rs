use std::{sync::{mpsc::Receiver, mpsc::Sender}, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use window::MainUI;

use crate::engine::{PcapCommand, PcapEvent};

// use crate::loading;

mod store;
mod window;
mod frames;
mod stack;


pub struct UI {
    sender: Sender<PcapCommand>,
    receiver: Receiver<PcapEvent>,
}


impl UI {
    pub fn new(sender: Sender<PcapCommand>, receiver: Receiver<PcapEvent>) -> Self {
        Self {sender, receiver}
    }
    pub fn run(&self) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();
        let mut store = store::Store::new(&self.sender);
        loop {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        match key.kind {
                            KeyEventKind::Press => {
                                match key.code {
                                    KeyCode::Char('q') | KeyCode::Esc => {
                                        self.sender.send(PcapCommand::Quit).unwrap();
                                        break;
                                    }
                                    KeyCode::Down => {
                                        if let Some(state) = &mut store.frame_data {
                                            state.next();
                                            store.select();
                                        }
                                    }
                                    KeyCode::Up => { 
                                        if let Some(state) = &mut store.frame_data {
                                            state.previous();
                                            store.select();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            match self.receiver.try_recv() {
                Ok(event) => {
                    store.update(event);
                }
                _ => {
                    // println!("no event");
                }
            }
            let app = MainUI::from(&store);
            terminal.draw(|f| f.render_widget(&app, f.area())).unwrap();
        }
        ratatui::restore();
        Ok(())
    }
}

