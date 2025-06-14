use crate::{
    engine::{PcapEvent, PcapUICommand},
    theme::{get_frame_color, get_header_style, get_select},
    ui::{
        block::content_border, loading::{self}, render_table, stack::StackView, ControlState, CustomTableState, TableStyle
    },
};

use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{
    concept::FrameInfo,
    util::date_sim_str,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Text,
    widgets::{Paragraph, Widget},
};

// const ITEM_HEIGHT: usize = 1;
// const FOOTER_BORDER_COLOR: Color = tailwind::BLUE.c400;

pub struct FrameStyle;

impl TableStyle<FrameInfo> for FrameStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        get_header_style()
    }

    fn get_row_style(&self, data: &FrameInfo, _: usize) -> ratatui::prelude::Style {
        get_frame_color(data)
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["", "Index", "Time", "Source", "Target", "Protocol", "Length", "Info"]
    }

    fn get_row(&self, data: &FrameInfo) -> Vec<String> {
        let mut rs: Vec<String> = Vec::new();
        rs.push("⏎".into());
        rs.push(format!("{}", data.index + 1).into());
        rs.push(date_sim_str(data.time).into());
        rs.push(data.source.clone().into());
        rs.push(data.dest.clone().into());
        rs.push(data.protocol.clone().into());
        rs.push(format!("{}", data.len).into());
        rs.push(data.info.clone().into());
        rs
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Max(20),
            Constraint::Max(20),
            Constraint::Max(10),
            Constraint::Max(6),
            Constraint::Min(70),
        ]
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SelectPanel {
    LIST,
    STACK,
}

pub struct App {
    state: CustomTableState<FrameInfo>,
    cursor: SelectPanel,
    // select: usize,
    view: StackView,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursor: SelectPanel::LIST,
            state: CustomTableState::new(),
            view: StackView::default(),
        }
    }
    fn render_loading(&self, area: Rect, buf: &mut Buffer) {
        loading::line("Loading frame data...", area, buf);
    }
    pub fn next(&mut self) -> usize {
        self.state.next()
    }
    pub fn previous(&mut self) -> usize {
        self.state.previous()
    }
    fn render_table(&mut self, buf: &mut Buffer, area: Rect) {
        render_table(FrameStyle, &mut self.state, area, buf,0);
    }
}

use Constraint::{Length, Min};
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.loading {
            self.render_loading(area, buf);
            return;
        }
        let vertical = Layout::vertical([Constraint::Min(4), Constraint::Min(5)]);
        let rects = vertical.split(area);
        {
            // let _area = get_erea(buf, rects[0], self.cursor == SelectPanel::LIST);
            self.render_table(buf, rects[0]);
        }

        if self.cursor == SelectPanel::STACK {
            // let _area = get_erea(buf, rects[1], self.cursor == SelectPanel::STACK);
            self.view.render(rects[1], buf);
        } else {
            let block = content_border();
            let inner = block.inner(rects[1]);
            block.render(rects[1], buf);

            let layout = Layout::vertical([Length(2), Min(0)]);
            let [_, _area] = layout.areas(inner);
            let text = Text::from("Press <Enter> to Detail");
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            paragraph.render(_area, buf);
        }
    }
}

impl ControlState for App {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.state.loading {
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
                if let Some(finfo) = self.state.list.items.get(self.state.select) {
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
                let total = self.state.list.total;
                let start = (self.state.list.items.last().unwrap().index + 1) as usize;
                if start < total {
                    let len = std::cmp::min(total - start, 100);
                    return PcapUICommand::FrameList(start, len);
                }
            }
            KeyCode::Left => {
                let start = self.state.list.items.first().unwrap().index as usize;
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
            PcapEvent::Init => PcapUICommand::FrameList(0, 100),
            PcapEvent::FrameList(list) => {
                self.state.update(list);
                self.cursor = SelectPanel::LIST;
                PcapUICommand::Refresh
            }
            _ => match self.cursor {
                SelectPanel::LIST => PcapUICommand::None,
                _ => self.view.update(event),
            },
        }
    }
}

// fn get_erea(buf: &mut Buffer, area: Rect, active: bool) -> Rect {
//     if active {
//         let block = Block::bordered()
//             .border_set(symbols::border::QUADRANT_OUTSIDE)
//             .padding(Padding::new(0, 0, 0, 0))
//             .border_style(ACTIVE_TAB_COLOR);
//         let inner_area = block.inner(area);
//         block.render(area, buf);
//         inner_area
//     } else {
//         let block = Block::bordered()
//             .border_set(symbols::border::PLAIN)
//             .padding(Padding::new(0, 0, 0, 0))
//             .border_style(ACTIVE_TAB_COLOR);
//         let inner_area = block.inner(area);
//         block.render(area, buf);
//         inner_area
//     }
// }
