use std::{cmp, rc::Rc};

use crate::{theme::{get_frame_color, get_header_style, get_select}, ControlPanel};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Margin, Rect}, style::{self, Color, Style, Stylize}, text::Text, widgets::{Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget}
};
use shark::common::{base::Instance, concept::{Criteria, FrameInfo}};
use style::palette::tailwind;

const INFO_TEXT: [&str; 2] = ["(Esc) quit | (↑) scroll up | (↓) scroll down | (←) prev page | (→) next page", "(Shift + →) next color | (Shift + ←) previous color"];

const ITEM_HEIGHT: usize = 1;

const BUFFER_BG: Color = tailwind::SLATE.c950;
const ROW_FG: Color = tailwind::SLATE.c200;
const FOOTER_BORDER_COLOR: Color = tailwind::BLUE.c400;

pub struct App {
    state: TableState,
    scroll_state: ScrollbarState,
    instance: Rc<Instance>,
    _start: usize,
    pub frames: Vec<FrameInfo>,
}
const STEP: usize = 50;

impl App {
    pub fn new(instance: Rc<Instance>) -> Self {
        let mut _self = Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(0),
            instance,
            frames: Vec::new(),
            _start: 0,
        };
        _self.set_data();
        _self
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
        self.state.select(Some(i));
        let inx = i * ITEM_HEIGHT;
        self.scroll_state = self.scroll_state.position(inx);
        self.scroll_state;
        self.scroll_state;
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
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn set_data(&mut self){
        let start = self._start;
        let total = self.instance.get_frames().len();
        let end = cmp::min(STEP + self._start, total);
        let size = end - start;
        let items = self.instance.get_frames_by(Criteria{start, size, criteria: "".into()});
        self.frames = items.items;
    }

    pub fn next_column(&mut self) {
        let total = self.instance.get_frames().len();
        if self._start + STEP >= total {
            return;
        }
        self._start += STEP;
        self.set_data();
    }

    pub fn previous_column(&mut self) {
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

    pub fn next_color(&mut self) {}

    pub fn previous_color(&mut self) {}


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
            .row_highlight_style(select_row_style)
            // .column_highlight_style(selected_col_style)
            // .highlight_symbol(Text::from(vec!["".into(), bar.into(), bar.into(), "".into()]))
            .bg(BUFFER_BG)
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(t, area, buf, &mut self.state);
        // t.render(area, buf, &mut self.state);
        // frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, area: Rect, buf: &mut Buffer,) {
        let scroll = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight).begin_symbol(None).end_symbol(None);
        let _area = area.inner(Margin { vertical: 1, horizontal: 1 });
        // StatefulWidget::render(scroll, _area, buf, &mut self.scroll_state);
        scroll.render(_area, buf, &mut self.scroll_state);
        // frame.render_stateful_widget(Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight).begin_symbol(None).end_symbol(None), area.inner(Margin { vertical: 1, horizontal: 1 }), &mut self.scroll_state);
        
    }

    fn render_footer(&self, buf: &mut Buffer, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(Style::new().fg(ROW_FG).bg(BUFFER_BG))
            .centered()
            .block(Block::bordered().border_type(BorderType::Double).border_style(Style::new().fg(FOOTER_BORDER_COLOR)));
        info_footer.render(area, buf);
        // frame.render_widget(info_footer, area);
    }
}

impl ControlPanel for App {
    fn control(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                // let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                    KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                    _ => {}
                }
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {

        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(area);
        self.render_table(buf, rects[0]);
        self.render_scrollbar(rects[0], buf);
        self.render_footer(buf, rects[1]);
    }
}
