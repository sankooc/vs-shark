use crossterm::event::Event;

pub mod ui;
pub mod theme;
pub mod engine;


pub const MAX_CONTENT_SIZE: usize = 0xffff;

pub type Result<T> = anyhow::Result<T>;

pub trait ControlPanel {
    fn control(&mut self, event: &Event);
}
