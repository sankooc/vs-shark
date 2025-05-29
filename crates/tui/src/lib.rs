use crossterm::event::Event;

pub mod ui;
pub mod loading;
// pub mod frames;
pub mod theme;
// // pub mod overview;
// pub mod panel;
// // pub mod stack;
// pub mod hex;
// pub mod control;
// pub mod tcp;
// pub mod base;
pub mod engine;


pub type Result<T> = anyhow::Result<T>;

pub trait ControlPanel {
    fn control(&mut self, event: &Event);
}
