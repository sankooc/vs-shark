use std::{cmp, rc::Rc};

use ratatui::{
    style::{Modifier, Stylize}, text::{Line, Span}, widgets::{Block, Padding, Paragraph, Widget}
};

use crate::theme::get_protocol_color;

pub struct HexView {
    data: Option<(usize, usize, Rc<Vec<u8>>)>
}

impl HexView {
    pub fn new() -> Self {
        Self {data: None}
    }
    pub fn set_data(&mut self, data: Option<(usize, usize, Rc<Vec<u8>>)>) {
        self.data = data;
    }
}


impl Widget for &mut HexView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let None = self.data {
            return;
        }
        let _data = self.data.as_ref().unwrap();
        let start = _data.0;
        let size = _data.1;
        let range = start..start+size;
        let data = _data.2.clone();
        let len = data.len();
        if len <= 0 {
            return;
        }
        let line_count: usize = (len - 1) / 16 + 1;
        let mut lines = Vec::new();
        let mut _cursor = 0;
        for inx in 0..line_count {
            let index:Span = format!("  {:#07x}0  ", inx).into();
            let mut ll = vec![index.style(get_protocol_color("tcp"))];
            let size = cmp::min(8, len - _cursor);
            let _data = &data[_cursor.._cursor + size];

            // let mut hex_style = get_protocol_color("tls");
            let get_style = |s| {
                if range.contains(&(s-1)) {
                    return get_protocol_color("dns");
                } else {
                    return get_protocol_color("tls");
                }
            };
            let mut left = _data.iter().map(|f| {
                _cursor+=1;
                Span::from(format!("{:02x} ", *f)).add_modifier(Modifier::BOLD).style(get_style(_cursor))
            }).collect::<Vec<Span>>();

            ll.append(&mut left);
            if _cursor + 1 < len {
                ll.push(Span::from("  "));
                let size = cmp::min(8, len - _cursor);
                let _data = &data[_cursor.._cursor + size];
                let mut right = _data.iter().map(|f| {
                    _cursor+=1;
                    Span::from(format!("{:02x} ", *f)).add_modifier(Modifier::BOLD).style(get_style(_cursor))
                }).collect::<Vec<_>>();
                ll.append(&mut right);
            }

            lines.push(Line::from(ll));
            // let txt = Text::from(ll);
        }
        let _top = Paragraph::new(lines).block(Block::bordered().padding(Padding::ZERO));
        _top.render(area, buf);
    }
}
