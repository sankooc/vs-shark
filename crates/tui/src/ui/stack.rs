use std::{cell::{RefCell, RefMut}, ops::DerefMut, rc::Rc};

use ratatui::{
    layout::{Constraint, Layout, Rect}, style::Modifier, widgets::{Block, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget}
};
// use shark::common::{base::Instance, concept::Field};
use tui_tree_widget::{Tree, TreeItem, TreeState};
use pcap::common::concept::Field;


use crate::{theme::get_active_tab_color};

pub struct StackState {
    data: Option<Vec<u8>>,
    items: Vec<Field>,
    tree_state: RefCell<TreeState<(usize, usize, usize)>>,
}

impl StackState {
    pub fn new(items: Vec<Field>) -> Self {
        Self {
            data: None,
            items,
            tree_state: RefCell::new(TreeState::default()),
        }
    }
    pub fn tree_state(&self) -> RefMut<TreeState<(usize, usize, usize)>>{
        self.tree_state.borrow_mut()
        // todo!()
        // &mut self.tree_state
    }
    
    pub fn items(&self) -> Vec<TreeItem<'static, (usize, usize, usize)>>{
        convert_fields(&self.items)
    }
}


pub struct StackView<'a> {
    state: &'a StackState,
    // hex: HexView,
    // instance: Rc<Instance>,
    // state: TreeState<u16>,
    // items: Vec<TreeItem<'static, u16>>,
    // frame_index: usize,
}
// impl StackView<'_> {
//     // pub fn new() -> Self {
//         Self {
//             // instance,
//             // hex: HexView::new(),
//             // state: TreeState::default(),
//             // items: vec![],
//             // frame_index: 0,
//         }
//     }
//     // pub fn set_items(&mut self, index: usize, items: Vec<TreeItem<'static, u16>>){
//     //     // self.frame_index = index;
//     //     // self.items = items;
//     //     // self.state = TreeState::default();
//     //     // self.hex.set_data(None);
//     // }

//     pub fn get_field(&mut self) -> Option<(usize, usize, Rc<Vec<u8>>)>{
//         // let sel = self.state.selected();
//         // if sel.len() > 0 {
//             // let binding = self.instance.get_frames();
//             // if let Some(f) = binding.get(self.frame_index) {
//             //     let list = f.get_fields();
//             //     let mut _list:&[Field] = &list;
//             //     for index in 0..sel.len() {
//             //         let _sel = sel[index] as usize;
//             //         if index >= sel.len() - 1 {
//             //             if let Some(_field) = _list.get(_sel)  {
//             //                 return Some((_field.start, _field.size, _field.data.clone()));
//             //             } else {
//             //                 break;
//             //             }
//             //         }
//             //         if let Some(_field) = _list.get(_sel)  {
//             //             _list = _field.children();
//             //         }
//             //     }

//             // }
//         // }
//         None
//     }
// }

impl Widget for &mut StackView<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {

        let ch:[Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);

        let list = self.state.items();
        let widget = Tree::new(&list)
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
            .highlight_style(get_active_tab_color().add_modifier(Modifier::BOLD))
            .highlight_symbol("");
        let mut reff = self.state.tree_state();
        StatefulWidget::render(widget, ch[0], buf, reff.deref_mut());
        // self.hex.render(ch[1], buf);
    }
}

// impl ControlPanel for StackView {
//     fn control(&mut self, event: &Event) {
//         if let Event::Key(k) = event {
//             match &k.code {
//                 KeyCode::Down => self.state.key_down(),
//                 KeyCode::Up => self.state.key_up(),
//                 KeyCode::Left => self.state.key_left(),
//                 KeyCode::Right => self.state.key_right(),
//                 _ => true,
//             };
//             let hex_data = self.get_field();
//             self.hex.set_data(hex_data);
//         }
//     }
    
// }


impl<'a> From<&'a StackState> for StackView<'a> {
    fn from(state: &'a StackState) -> Self {
        Self { state }
    }
}

fn convert_fields(list: &[Field]) -> Vec<TreeItem<'static, (usize, usize, usize)>>{
    let mut rs = Vec::new();
    let mut count = 0;
    for item in list {
        let start = item.start;
        let size = item.size;
        if let Some(children) = &item.children {
            let child = convert_fields(children);
            let it = TreeItem::new((count, start, size), item.summary.clone(), child).expect("need unique id");
            rs.push(it);
        
        } else {
            rs.push(TreeItem::new_leaf((count, start, size), item.summary.clone()));
        }
        count += 1;
    }
    rs
}