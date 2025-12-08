use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{concept::Field, io::DataSource};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Scrollbar, ScrollbarOrientation, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    engine::{PcapEvent, PcapUICommand},
    theme::get_active_tab_color,
    ui::{block::{content_border_low, content_border_right}, ControlState},
};

use super::hex::{HexState, HexView};

type Indendity = (usize, usize, usize, u8);

#[derive(Default)]
pub struct StackView {
    // data_source: Option<DataSource>,
    // extra: Option<Vec<u8>>,
    datasources: Vec<DataSource>,
    items: Vec<Field>,
    tree_state: TreeState<Indendity>,
}

impl StackView {
    pub fn items(&self) -> Vec<TreeItem<'static, Indendity>> {
        convert_fields(&self.items)
    }
    pub fn reset(&mut self, items: Vec<Field>) {
        self.items = items;
    }
}

impl Widget for &mut StackView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let ch: [Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);

        let list = self.items();
        let widget = Tree::new(&list)
            .expect("all item identifiers are unique")
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(None).track_symbol(None).end_symbol(None),
            ))
            .highlight_style(get_active_tab_color().add_modifier(Modifier::BOLD))
            .highlight_symbol("");
            let mut _top = widget.block(content_border_low());

        StatefulWidget::render(_top, ch[0], buf, &mut self.tree_state);
        

        let selected = self.tree_state.selected();
        if !selected.is_empty() {
            let (_, start, size, source) = *selected.last().unwrap();
            if let Some(datasource) = self.datasources.get(source as usize) {
                let offset = start.saturating_sub(datasource.range().start);
                let state = HexState::new(offset, size, &datasource.data);
                let mut _hex = HexView::from(&state);
                _hex.render(ch[1], buf);
            }
            // if source == 0 {
            //     if let Some(ds) = &self.data_source {
            //         let _start = start - std::cmp::min(start, ds.range().start);
            //         let state = HexState::new(_start, size, &ds.data);
            //         let mut _hex = HexView::from(&state);
            //         _hex.render(ch[1], buf);
            //     }
            // } else {
            //     if let Some(extra) = &self.extra {
            //         let state = HexState::new(start, size, extra);
            //         let mut _hex = HexView::from(&state);
            //         _hex.render(ch[1], buf);
            //     }
            // }
        } else {
            content_border_right().render(ch[1], buf);
        }
    }
}

impl ControlState for StackView {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        let state = &mut self.tree_state;
        match event.code {
            KeyCode::Down => {
                state.key_down();
            }
            KeyCode::Up => {
                state.key_up();
            }
            KeyCode::Left => {
                state.key_left();
            }
            KeyCode::Right => {
                state.key_right();
            }
            _ => {
                return PcapUICommand::None
            }
        };

        PcapUICommand::Refresh
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::FrameData(fields, datasources) => {
                self.items = fields;
                self.datasources = datasources;
                // self.data_source = ds;
                // self.extra = extra;
                self.tree_state.close_all();
                PcapUICommand::Refresh
            },
            _ => PcapUICommand::None
        }
    }
}
fn convert_fields(list: &[Field]) -> Vec<TreeItem<'static, Indendity>> {
    let mut rs = Vec::new();
    // let mut count = 0;
    for (count, item) in list.iter().enumerate() {
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
    }
    rs
}
