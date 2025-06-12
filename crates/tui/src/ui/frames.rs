use crate::{
    engine::{PcapEvent, PcapUICommand},
    theme::{get_frame_color, get_header_style, get_select, ACTIVE_TAB_COLOR, ICMPV6_FG},
    ui::{loading, stack::StackView, ControlState},
};

use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{
    concept::{FrameInfo, ListResult},
    util::date_sim_str,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Stylize},
    symbols,
    text::Text,
    widgets::{Block, Cell, HighlightSpacing, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget},
};

const ITEM_HEIGHT: usize = 1;
// const FOOTER_BORDER_COLOR: Color = tailwind::BLUE.c400;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SelectPanel {
    LIST,
    STACK,
}

pub struct App {
    loading: bool,
    list: ListResult<FrameInfo>,
    cursor: SelectPanel,
    select: usize,
    view: StackView,
}

impl App {
    pub fn new() -> Self {
        Self {
            loading: true,
            list: ListResult::empty(),
            cursor: SelectPanel::LIST,
            select: 0,
            view: StackView::default(),
        }
    }
    fn render_loading(&self, area: Rect, buf: &mut Buffer) {
        loading::line("Loading frame data...", area, buf);
    }
    pub fn get_selection(&self) -> TableState {
        TableState::default().with_selected(self.select)
    }
    pub fn scroll_state(&self) -> ScrollbarState {
        let ss = ScrollbarState::new(self.list.items.len() * ITEM_HEIGHT);
        ss.position(self.select * ITEM_HEIGHT)
    }
    pub fn next(&mut self) -> usize {
        if self.select < self.list.items.len() - 1 {
            self.select += 1;
        }
        self.select
    }
    pub fn previous(&mut self) -> usize {
        if self.select > 0 {
            self.select -= 1;
        }
        self.select
    }
    fn render_table(&mut self, buf: &mut Buffer, area: Rect) {
        let header_style = get_header_style();
        let cols = ["Index", "Time", "Source", "Target", "Protocol", "Length", "Info"];
        let header = cols.into_iter().map(Cell::from).collect::<Row>().style(header_style).height(1);
        let frames = &self.list.items;
        let rows = frames.iter().map(|data| {
            let mut rs: Vec<Cell> = Vec::new();
            rs.push(format!("{}", data.index + 1).into());
            rs.push(date_sim_str(data.time).into());
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
                Constraint::Length(10),
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
        StatefulWidget::render(t, t_area, buf, &mut self.get_selection());

        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scroll.render(area, buf, &mut self.scroll_state());
    }
}

use Constraint::{Length, Min};
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.loading {
            self.render_loading(area, buf);
            return;
        }
        let vertical = Layout::vertical([Constraint::Min(4), Constraint::Min(5)]);
        let rects = vertical.split(area);
        {
            let _area = get_erea(buf, rects[0], self.cursor == SelectPanel::LIST);
            self.render_table(buf, _area);
        }

        if self.cursor == SelectPanel::STACK {
            let _area = get_erea(buf, rects[1], self.cursor == SelectPanel::STACK);
            self.view.render(_area, buf);
        } else {
            let _area = get_erea(buf, rects[1], false);
            let layout = Layout::vertical([Length(2), Min(0)]);
            let [_, _area] = layout.areas(_area);
            let text = Text::from("Press <Enter> to Detail");
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            paragraph.render(_area, buf);
        }
    }
}

impl ControlState for App {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.loading {
            return PcapUICommand::None;
        }
        if self.cursor == SelectPanel::STACK {
            if let KeyCode::Backspace = event.code {
                self.cursor = SelectPanel::LIST;
                return PcapUICommand::Refresh;
            }
            return self.view.control(false, event);
        }
        match event.code {
            KeyCode::Enter => {
                self.cursor = SelectPanel::STACK;
                if let Some(finfo) = self.list.items.get(self.select) {
                    return PcapUICommand::FrameData(finfo.index);
                }
                return PcapUICommand::None;
            }
            KeyCode::Down => {
                self.next();
                // self.field = None;
            }
            KeyCode::Up => {
                self.previous();
                // self.field = None;
            }
            KeyCode::Right => {
                let total = self.list.total;
                let start = (self.list.items.last().unwrap().index + 1) as usize;
                if start < total {
                    let len = std::cmp::min(total - start, 100);
                    return PcapUICommand::FrameList(start, len);
                }
            }
            KeyCode::Left => {
                let start = self.list.items.first().unwrap().index as usize;
                if start > 0 {
                    let _start = std::cmp::max(100, start);
                    return PcapUICommand::FrameList(_start - 100, 100);
                }
            }
            _ => {}
        }
        PcapUICommand::Refresh
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::Init => {
                PcapUICommand::FrameList(0, 100)
            }
            PcapEvent::FrameList(list) => {
                self.list = list;
                self.cursor = SelectPanel::LIST;
                self.select = 0;
                self.loading = false;
                PcapUICommand::Refresh
            }
            _ => match self.cursor {
                SelectPanel::LIST => PcapUICommand::None,
                _ => self.view.update(event),
            },
        }
    }
}

fn get_erea(buf: &mut Buffer, area: Rect, active: bool) -> Rect {
    if active {
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .padding(Padding::new(0, 0, 0, 0))
            .border_style(ACTIVE_TAB_COLOR);
        let inner_area = block.inner(area);
        block.render(area, buf);
        inner_area
    } else {
        let block = Block::bordered()
            .border_set(symbols::border::PLAIN)
            .padding(Padding::new(0, 0, 0, 0))
            .border_style(ICMPV6_FG);
        let inner_area = block.inner(area);
        block.render(area, buf);
        inner_area
    }
}
