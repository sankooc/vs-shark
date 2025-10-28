use std::cmp;

use ratatui::{
    style::{Modifier, Stylize}, text::{Line, Span}, widgets::{Paragraph, Widget}
};

use crate::{theme::get_protocol_color, ui::block::{content_border_right}};

pub struct HexState<'a> {
    start: usize,
    size: usize,
    data: &'a [u8]
}

impl <'a>HexState<'a> {
    pub fn new(start: usize, size: usize, data: &'a [u8]) -> Self {
        Self { start, size, data }
    }
}



pub struct HexView<'a> {
    state: &'a HexState<'a>,
    // data: Option<(usize, usize, Rc<Vec<u8>>)>
}

impl<'a> From<&'a HexState<'a>> for HexView<'a> {
    fn from(state: &'a HexState<'a>) -> Self {
        Self { state }
    }
}

impl Widget for &mut HexView<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // if let None = self.state {
        //     return;
        // }
        let data = self.state.data;
        let start = self.state.start;
        let size = self.state.size;
        let range = start..start+size;
        let len = data.len();
        if len == 0 {
            return;
        }
        let line_count: usize = (len - 1) / 16 + 1;
        let mut lines = Vec::new();
        let mut _cursor = 0;
        let mut head = None;
        for inx in 0..line_count {
            let index:Span = format!("  {:#07x}0  ", inx).into();
            let mut ll = vec![index.style(get_protocol_color("tcp"))];
            let size: usize = cmp::min(8, len - _cursor);
            // let _data_range = _cursor.._cursor + size;
            let _data = &data[_cursor.._cursor + size];
            if  (_cursor.._cursor + 16).contains(&range.start){
                if let None = &head {
                    head = Some(inx);
                }
            }

            // let mut hex_style = get_protocol_color("tls");
            let get_style = |s| {
                // if s < 1 {
                //     return get_protocol_color("tls");
                // }
                if range.contains(&s) {
                    return get_protocol_color("dns");
                } else {
                    return get_protocol_color("tls");
                }
            };
            let mut _style = get_style(_cursor);
            let mut left = _data.iter().map(|f| {
                _style = get_style(_cursor);
                _cursor+=1;
                Span::from(format!("{:02x} ", *f)).add_modifier(Modifier::BOLD).style(_style)
            }).collect::<Vec<Span>>();

            ll.append(&mut left);
            if _cursor < len {
                _style = get_style(_cursor);
                ll.push(Span::from("  ").style(_style));
                let size = cmp::min(8, len - _cursor);
                let _data = &data[_cursor.._cursor + size];
                let mut right = _data.iter().map(|f| {
                    _style = get_style(_cursor);
                    _cursor+=1;
                    Span::from(format!("{:02x} ", *f)).add_modifier(Modifier::BOLD).style(_style)
                }).collect::<Vec<_>>();
                ll.append(&mut right);
            }

            lines.push(Line::from(ll));
            // let txt = Text::from(ll);
        }
        let mut _top = Paragraph::new(lines).block(content_border_right());
        if let Some(offset) = head {
            let _offset = std::cmp::max(2, offset) as u16;
            _top.scroll((_offset - 2, 0)).render(area, buf);
        } else {
            _top.render(area, buf);
        }
        
    }
}
