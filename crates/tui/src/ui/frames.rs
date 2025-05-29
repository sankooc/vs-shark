use std::{cmp, rc::Rc};

use crate::{
    theme::{get_frame_color, get_header_style, get_select},
    ControlPanel,
};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use pcap::common::concept::{Field, FrameInfo, ListResult};
use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{Modifier, Stylize}, text::Text, widgets::{Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget}
};
use tui_tree_widget::TreeItem;

use super::stack::{StackState, StackView};

// const INFO_TEXT: [&str; 2] = ["(Esc) quit | (↑) scroll up | (↓) scroll down | (←) prev page | (→) next page", "(Shift + →) next color | (Shift + ←) previous color"];

pub struct FrameState {
    pub cursor: SelectPanel,
    pub list: ListResult<FrameInfo>,
    pub select: usize,
    pub data: Vec<FrameInfo>,
    pub field: Option<StackState>,
}
impl FrameState {
    pub fn new(list: ListResult<FrameInfo>) -> Self {
        Self {
            list,
            cursor: SelectPanel::LIST,
            select: 0,
            data: Vec::new(),
            field: None,
        }
    }
    pub fn next(&mut self) {
        if self.select < self.list.items.len() - 1 {
            self.select += 1;
        }
    }
    pub fn previous(&mut self) {
        if self.select > 0 {
            self.select -= 1;
        }
    }
    pub fn get_selection(&self) -> TableState {
        TableState::default().with_selected(self.select)
    }
    pub fn scroll_state(&self) -> ScrollbarState {
        let ss = ScrollbarState::new(self.list.items.len() * ITEM_HEIGHT);
        ss.position(self.select * ITEM_HEIGHT)
    }
}
const ITEM_HEIGHT: usize = 1;
// const FOOTER_BORDER_COLOR: Color = tailwind::BLUE.c400;

pub enum SelectPanel {
    LIST,
    STACK,
}
pub struct App<'a> {
    state: &'a  FrameState,
}

impl App<'_> {
    fn render_table(&mut self, buf: &mut Buffer, area: Rect) {
        let header_style = get_header_style();
        let cols = ["Index", "Source", "Target", "Protocol", "Length", "Info"];
        let header = cols.into_iter().map(Cell::from).collect::<Row>().style(header_style).height(1);
        let frames = &self.state.list.items;
        let rows = frames.iter().map(|data| {
            let mut rs: Vec<Cell> = Vec::new();
            rs.push(format!("{}", data.index).into());
            rs.push(data.source.clone().into());
            rs.push(data.dest.clone().into());
            rs.push(data.protocol.clone().into());
            rs.push(format!("{}", data.len).into());
            rs.push(data.info.clone().into());
            let row_style = get_frame_color(data);
            rs.into_iter().collect::<Row>().add_modifier(Modifier::BOLD).style(row_style).height(1)
        });

        let select_row_style = get_select();
        let t: Table<'_> = Table::new(
            rows,
            [
                Constraint::Length(5),
                Constraint::Max(20),
                Constraint::Max(20),
                Constraint::Max(10),
                Constraint::Max(6),
                Constraint::Min(70),
            ],
        )
        .header(header)
        .highlight_style(select_row_style)
        .highlight_spacing(HighlightSpacing::Always);
        let mut t_area = area.clone();
        t_area.width -= 1;
        StatefulWidget::render(t, t_area, buf, &mut self.state.get_selection());

        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scroll.render(area, buf, &mut self.state.scroll_state());
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Min(4), Constraint::Min(5)]);
        let rects = vertical.split(area);
        self.render_table(buf, rects[0]);
        if let Some(field) = &self.state.field {
            // let ch:[Rect; 2] = Layout::horizontal([Constraint::Min(5), Constraint::Min(5)]).areas(rects[1]);
            // self.stack_page.render(rects[1], buf);
            // super::hex::HexView::new().render(ch[1], buf);
            StackView::from(field).render(rects[1], buf);
        } else {
            let text = Text::from("No Data");
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            paragraph.render(rects[1], buf);
        }
        // let ch:[Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(rects[1]);
        // self.stack_page.render(rects[1], buf);
        // super::hex::HexView::new().render(ch[1], buf);
    }
}

impl<'a> From<&'a FrameState> for App<'a> {
    fn from(state: &'a FrameState) -> Self {
        Self { state }
    }
}
