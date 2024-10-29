use std::{cmp, rc::Rc};

use crate::{
    theme::{get_frame_color, get_header_style, get_select},
    ControlPanel,
};

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget},
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
        // let identity = format!("{}-{}", pre, count);
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
        self.stack_page.set_items(inx, stack_items);
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
            rs.push(Cell::from(Text::from(format!("{}", data.index))));
            rs.push(Cell::from(Text::from(data.source.clone())));
            rs.push(Cell::from(Text::from(data.dest.clone())));
            rs.push(Cell::from(Text::from(data.protocol.clone())));
            rs.push(Cell::from(Text::from(format!("{}", data.len))));
            rs.push(Cell::from(Text::from(data.info.clone())));
            let row_style = get_frame_color(data);
            rs.into_iter().collect::<Row>().style(row_style).height(1)
        });

        let select_row_style = get_select();
        // let bar = " █ ";
        let t: Table<'_> = Table::new(rows, [Constraint::Length(5), Constraint::Max(20), Constraint::Max(20), Constraint::Max(6), Constraint::Max(6), Constraint::Min(70)])
            .header(header)
            .highlight_style(select_row_style)
            // .row_highlight_style(select_row_style)
            // .column_highlight_style(selected_col_style)
            // .highlight_symbol(Text::from(vec!["".into(), bar.into(), bar.into(), "".into()]))
            // .bg(BUFFER_BG)
            .highlight_spacing(HighlightSpacing::Always);
        // Widget::render(t, area, buf);
        StatefulWidget::render(t, area, buf, &mut self.state);
    
        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scroll.render(area, buf, &mut self.scroll_state);
    }

    // fn render_footer(&self, buf: &mut Buffer, area: Rect) {
    //     let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
    //         .style(Style::new().fg(ROW_FG).bg(BUFFER_BG))
    //         .centered()
    //         .block(Block::bordered().border_type(BorderType::Double).border_style(Style::new().fg(FOOTER_BORDER_COLOR)));
    //     info_footer.render(area, buf);
    //     // frame.render_widget(info_footer, area);
    // }
}

impl ControlPanel for App {
    fn control(&mut self, event: &Event) {
        
        // let shift_pressed = _event.modifiers.contains(KeyModifiers::SHIFT);
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
                            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                            KeyCode::Char('l') | KeyCode::Right => self.next_page(),
                            KeyCode::Char('h') | KeyCode::Left => self.previous_page(),
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
    where
        Self: Sized,
    {
        let vertical = Layout::vertical([Constraint::Min(5), Constraint::Min(5)]);
        let rects = vertical.split(area);
        self.render_table(buf, rects[0]);
        // let ch:[Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(rects[1]);
        self.stack_page.render(rects[1], buf);
        // super::hex::HexView::new().render(ch[1], buf);
    }
}
