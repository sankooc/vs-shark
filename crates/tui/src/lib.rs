use crossterm::event::Event;

pub mod ui;
pub mod theme;
pub mod engine;


pub type Result<T> = anyhow::Result<T>;

pub trait ControlPanel {
    fn control(&mut self, event: &Event);
}
