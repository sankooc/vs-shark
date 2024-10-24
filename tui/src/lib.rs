use crossterm::event::Event;

pub mod ui;
pub mod loading;
pub mod table;
pub mod theme;
pub mod overview;
pub mod panel;

pub type Result<T> = color_eyre::Result<T>;


pub trait ControlPanel {
    fn control(&mut self, event: &Event);
}