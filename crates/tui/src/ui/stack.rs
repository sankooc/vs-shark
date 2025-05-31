
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Scrollbar, ScrollbarOrientation, StatefulWidget, Widget},
};
use pcap::common::{concept::Field, io::DataSource};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{engine::PcapCommand, theme::get_active_tab_color};

use super::{hex::{HexState, HexView}, store::ControlState};


type Indendity = (usize, usize, usize, u8);


pub struct StackState {
    data_source: Option<DataSource>,
    extra: Option<Vec<u8>>,
    items: Vec<Field>,
    tree_state: TreeState<Indendity>,
}

impl StackState {
    pub fn new(items: Vec<Field>, data_source: Option<DataSource>, extra: Option<Vec<u8>>) -> Self {
        Self {
            data_source,
            extra,
            items,
            tree_state: TreeState::default(),
        }
    }

    pub fn items(&self) -> Vec<TreeItem<'static, Indendity>> {
        convert_fields(&self.items)
    }
}

impl ControlState for StackState {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapCommand {
        let state = &mut self.tree_state;
        match event.code {
            KeyCode::Down => {state.key_down();},
            KeyCode::Up => {state.key_up();},
            KeyCode::Left => {state.key_left();},
            KeyCode::Right => {state.key_right();},
            _ => {},
        };
        
        PcapCommand::None
    }
}

pub struct StackView<'a> {
    state: &'a mut StackState,
}

impl Widget for StackView<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ch: [Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);

        let list = self.state.items();
        let widget = Tree::new(&list)
            .expect("all item identifiers are unique")
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(None).track_symbol(None).end_symbol(None),
            ))
            .highlight_style(get_active_tab_color().add_modifier(Modifier::BOLD))
            .highlight_symbol("");
        StatefulWidget::render(widget, ch[0], buf, &mut self.state.tree_state);
        let selected = self.state.tree_state.selected();
        if selected.len() > 0 {
            let (_, start,size,source) = selected.last().unwrap().clone();
            if source == 0 {
                if let Some(ds) = &self.state.data_source {
                    let _start = start - std::cmp::min(start, ds.range().start);
                    let state = HexState::new(_start, size, &ds.data);
                    let mut _hex = HexView::from(&state);
                    _hex.render(ch[1], buf);
                }
            } else {
                if let Some(extra) = &self.state.extra {
                    let state = HexState::new(start, size, extra);
                    let mut _hex = HexView::from(&state);
                    _hex.render(ch[1], buf);
                }
            }
            
        }
    }
}

impl<'a> From<&'a mut StackState> for StackView<'a> {
    fn from(state: &'a mut StackState) -> Self {
        Self { state }
    }
}

fn convert_fields(list: &[Field]) -> Vec<TreeItem<'static, Indendity>> {
    let mut rs = Vec::new();
    let mut count = 0;
    for item in list {
        let start = item.start;
        let size = item.size;
        let source = item.source;
        let key = (count, start, size, source);
        if let Some(children) = &item.children {
            let child = convert_fields(children);
            let it = TreeItem::new(key, item.summary.clone(), child).expect("need unique id");
            rs.push(it);
        } else {
            rs.push(TreeItem::new_leaf(key, item.summary.clone()));
        }
        count += 1;
    }
    rs
}
