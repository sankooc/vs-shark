use std::{cmp, io::BufRead, rc::Rc};

use ratatui::{
    style::Styled, text::{Line, Span, Text}, widgets::{Block, Padding, Paragraph, Widget}
};

use crate::theme::get_protocol_color;

pub struct HexView {
    data: Option<(usize, usize, Rc<Vec<u8>>)>
}

impl HexView {
    pub fn new() -> Self {
        Self {data: None}
    }
}

pub fn _convert_field() {
    let start = 1;
    let size = 3;
    let data: Vec<u8> = vec![12, 32, 3, 2, 2, 1, 3, 4, 5, 4, 6, 4, 3, 3, 3, 4, 5, 3, 3, 4, 5, 54, 3, 3, 2, 23, 4, 5, 4, 3, 45, 54, 6, 2];
    let len = data.len();
    if len <= 0 {
        return;
    }
    let lines: usize = (len - 1) / 16 + 1;
}
impl Widget for &mut HexView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {

        let data: Vec<u8> = vec![12, 32, 3, 2, 2, 1, 3, 4, 5, 4, 6, 4, 3, 3, 3, 4, 5, 3, 3, 4, 5, 54, 3, 3, 2, 23, 4, 5, 4, 3, 45, 54, 6, 2];
        let len = data.len();
        let line_count: usize = (len - 1) / 16 + 1;
        let mut lines = Vec::new();
        let mut _cursor = 0;
        for inx in 0..line_count {
            let index:Span = format!("  {:#07x}0  ", inx).into();
            let mut ll = vec![index.style(get_protocol_color("tcp"))];
            let size = cmp::min(8, len - _cursor);
            let _data = &data[_cursor.._cursor + size];
            let left = _data.iter().map(|f| format!("{:02x}", *f)).collect::<Vec<_>>().join(" ");
            _cursor += 8;
            let hex_style = get_protocol_color("tls");
            ll.push(Span::from(left).style(hex_style.clone()));
            if _cursor + 1 < len {
                ll.push(Span::from("  ").style(hex_style.clone()));
                let size = cmp::min(8, len - _cursor);
                let _data = &data[_cursor.._cursor + size];
                let right = _data.iter().map(|f| format!("{:02x}", *f)).collect::<Vec<_>>().join(" ");
                ll.push(Span::from(right).style(hex_style.clone()));
                _cursor += 8;
            }

            lines.push(Line::from(ll));
            // let txt = Text::from(ll);
        }
        let _top = Paragraph::new(lines).block(Block::bordered().padding(Padding::ZERO));
        _top.render(area, buf);
    }
}
