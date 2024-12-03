use std::{cmp, rc::Rc};

use crate::{
    theme::{get_frame_color, get_header_style, get_select},
    ControlPanel,
};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Modifier, Stylize}, widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget}
};
use shark::common::{
    base::Instance,
    concept::{Criteria, FrameInfo},
};
use tui_tree_widget::TreeItem;

// const INFO_TEXT: [&str; 2] = ["(Esc) quit | (↑) scroll up | (↓) scroll down | (←) prev page | (→) next page", "(Shift + →) next color | (Shift + ←) previous color"];

const ITEM_HEIGHT: usize = 1;
// const FOOTER_BORDER_COLOR: Color = tailwind::BLUE.c400;

pub enum SelectPanel {
    LIST,
    STACK,
}
pub struct App {
    cursor: SelectPanel,
    stack_page: super::stack::StackView,
    state: TableState,
    scroll_state: ScrollbarState,
    instance: Rc<Instance>,
    _start: usize,
    pub frames: Vec<FrameInfo>,
}
const STEP: usize = 100;

fn convert_fields(list: &[shark::common::concept::Field]) -> Vec<TreeItem<'static, u16>>{
    let mut rs = Vec::new();
    let mut count = 0;
    for item in list {
        if item.children().len() > 0 {
            let child = convert_fields(item.children());
            let it = TreeItem::new(count, item.summary(), child).expect("need unique id");
            rs.push(it);
        } else {
            rs.push(TreeItem::new_leaf(count, item.summary()));
        }
        count += 1;
    }
    rs
}

impl App {
    pub fn new(instance: Rc<Instance>) -> Self {
        let mut _self = Self {
            cursor: SelectPanel::LIST,
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(0),
            stack_page: super::stack::StackView::new(instance.clone()),
            instance,
            frames: Vec::new(),
            _start: 0,
        };
        _self.set_data();
        _self
    }

    fn select(&mut self, inx: usize){
        let frame_index = self._start + inx;
        self.state.select(Some(inx));
        self.scroll_state = self.scroll_state.position(inx * ITEM_HEIGHT);
        let fs = self.instance.get_frames();
        let f = &fs[frame_index];
        let stacks = f.get_fields();
        let stack_items = convert_fields(&stacks);
        self.stack_page.set_items(self._start + inx, stack_items);
    }
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.frames.len() - 1 {
                    return;
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.select(i);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    return;
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.select(i);
    }

    pub fn set_data(&mut self) {
        let start = self._start;
        let total = self.instance.get_frames().len();
        let end = cmp::min(STEP + self._start, total);
        let size = end - start;
        let items = self.instance.get_frames_by(Criteria { start, size, criteria: "".into() });
        self.frames = items.items;
        self.scroll_state = ScrollbarState::new(self.frames.len() * ITEM_HEIGHT);
        if let Some(inx) = self.state.selected() {
            self.select(inx);
        }
    }

    pub fn next_page(&mut self) {
        let total = self.instance.get_frames().len();
        if self._start + STEP >= total {
            return;
        }
        self._start += STEP;
        self.set_data();
    }

    pub fn previous_page(&mut self) {
        if self._start == 0 {
            return;
        }
        if self._start >= STEP {
            self._start -= STEP;
        } else {
            self._start = 0;
        }
        self.set_data();
    }

    fn render_table(&mut self, buf: &mut Buffer, area: Rect) {
        let header_style = get_header_style();
        let cols = ["Index", "Source", "Target", "Protocol", "Length", "Info"];
        let header = cols.into_iter().map(Cell::from).collect::<Row>().style(header_style).height(1);

        let rows = self.frames.iter().map(|data| {
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
        let t: Table<'_> = Table::new(rows, [Constraint::Length(5), Constraint::Max(20), Constraint::Max(20), Constraint::Max(6), Constraint::Max(6), Constraint::Min(70)])
            .header(header)
            .highlight_style(select_row_style)
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(t, area, buf, &mut self.state);
    
        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scroll.render(area, buf, &mut self.scroll_state);
    }
}

impl ControlPanel for App {
    fn control(&mut self, event: &Event) {
        match event {
            Event::Key(_event) => {
                let shift_pressed = _event.modifiers.contains(KeyModifiers::SHIFT);
                if shift_pressed {
                    if _event.kind == KeyEventKind::Press {
                        match &_event.code {
                            KeyCode::Up => {
                                self.cursor = SelectPanel::LIST;
                            }
                            KeyCode::Down => {
                                self.cursor = SelectPanel::STACK;
                            }
                            _ => {}
                        }
                        return;
                    }
                    
                }
            }
            _ => {}
        }
        match self.cursor {
            SelectPanel::STACK => {
                self.stack_page.control(event);
            }
            _ => {
                if let Event::Key(key) = event {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Down => self.next_row(),
                            KeyCode::Up => self.previous_row(),
                            KeyCode::Right => self.next_page(),
                            KeyCode::Left => self.previous_page(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    // fn control(&mut self, event: &Event) {
    // }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let vertical = Layout::vertical([Constraint::Min(5), Constraint::Min(5)]);
        let rects = vertical.split(area);
        self.render_table(buf, rects[0]);
        // let ch:[Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(rects[1]);
        self.stack_page.render(rects[1], buf);
        // super::hex::HexView::new().render(ch[1], buf);
    }
}
