use crossterm::event::{Event, KeyCode};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::ControlPanel;

pub struct StackView {
    state: TreeState<u16>,
    items: Vec<TreeItem<'static, u16>>,
}
impl StackView {
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: vec![],
        }
    }
    pub fn set_items(&mut self, items: Vec<TreeItem<'static, u16>>){
        self.items = items;
        self.state = TreeState::default();
    }
}

impl Widget for &mut StackView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {

        let widget = Tree::new(&self.items)
            .expect("all item identifiers are unique")
            .block(
                Block::bordered()
            )
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" ");
        StatefulWidget::render(widget, area, buf, &mut self.state);
    }
}

impl ControlPanel for StackView {
    fn control(&mut self, event: &Event) {
        if let Event::Key(k) = event {
            match &k.code {
                // KeyCode::Enter | KeyCode::Char(' ') => self.state.toggle_selected(),
                KeyCode::Down | KeyCode::Char('j') => self.state.key_down(),
                KeyCode::Up | KeyCode::Char('k') => self.state.key_up(),
                KeyCode::Left | KeyCode::Char('h') => self.state.key_left(),
                KeyCode::Right | KeyCode::Char('l') => self.state.key_right(),
                _ => true,
            };
        }
        let sel = self.state.selected();
        println!("{:?}", sel);
    }
}
