use crossterm::event::{KeyCode, KeyEvent};
use pcap::common::{
    concept::{VHttpConnection},
    util::format_bytes_single_unit_int,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::{Block, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget},
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::{
    engine::{HttpMessageWrap, PcapEvent, PcapUICommand},
    theme::get_active_tab_color,
    ui::{block::{content_border_low, content_border_right}, code::CodeView, loading::{self}, render_table, ControlState, CustomTableState, TableStyle},
};

pub struct Page {
    state: CustomTableState<VHttpConnection>,
    detail: Option<HttpHeadersView>,
}
pub struct ConversationStyle;
impl TableStyle<VHttpConnection> for ConversationStyle {
    fn get_header_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_header_style()
    }

    fn get_row_style(&self, _: &VHttpConnection, status: usize) -> ratatui::prelude::Style {
        match status {
            1 => crate::theme::BLANK_FROZEN,
            _ => crate::theme::DNS_BG.into()
        }
    }

    fn get_select_style(&self) -> ratatui::prelude::Style {
        crate::theme::get_select()
    }

    fn get_cols(&self) -> Vec<&str> {
        vec!["", "Status", "Method", "Host", "Length", "ContentType", "Time"]
    }

    fn get_row(&self, data: &VHttpConnection, selected: bool) -> Vec<String> {
        vec![
            if selected { "⏎".into() } else { "".into() },
            data.status().to_string(),
            data.method().to_string(),
            data.url().to_string(),
            format_bytes_single_unit_int(data.length),
            data.content_type.clone(),
            data.latency.clone(),
        ]
    }

    fn get_row_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(1),
            Constraint::Max(5),
            Constraint::Max(10),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Max(14),
        ]
    }
    fn get_block(&self) -> Option<Block> {
        None
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            state: CustomTableState::default(),
            detail: None,
        }
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.loading {
            loading::line(area, buf);
            return;
        }
        if let Some(view) = &mut self.detail {
            let vertical = ratatui::layout::Layout::vertical([Constraint::Length(6), Constraint::Min(5)]);
            let rects = vertical.split(area);
            render_table(ConversationStyle, &self.state, rects[0], buf, 1);
            view.render(rects[1], buf);
            return;
        }
        render_table(ConversationStyle, &self.state, area, buf, 0);
    }
}

const PAGE_SIZE: usize = 100;

impl ControlState for Page {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        if self.state.loading {
            return PcapUICommand::None;
        }
        if let Some(view) = &mut self.detail {
            return match &event.code {
                KeyCode::Backspace => {
                    self.detail = None;
                    return PcapUICommand::Refresh;
                }
                _ => view.control(false, event)
            }
        }

        match event.code {
            KeyCode::Enter => {
                if let Some(item) = self.state.list.items.get(self.state.select) {
                    // let index = item.index;
                    return PcapUICommand::HttpDetail(item.index);
                    // return PcapUICommand::HttpContent(item.clone());
                }
            }
            KeyCode::Down => {
                self.state.next();
            }
            KeyCode::Up => {
                self.state.previous();
            }
            KeyCode::Right => {
                let total = self.state.list.total;
                let len = self.state.list.items.len();
                if total == 0 || len < PAGE_SIZE {
                    return PcapUICommand::None;
                }
                let start = self.state.list.start + len;

                if start < total {
                    let len = std::cmp::min(total - start, PAGE_SIZE);
                    return PcapUICommand::HttpConnectionList(start, len);
                }
                return PcapUICommand::None;
            }
            KeyCode::Left => {
                if self.state.list.start == 0 {
                    return PcapUICommand::None;
                }
                let _pre = std::cmp::min(self.state.list.start, PAGE_SIZE);
                let start = self.state.list.start - _pre;
                return PcapUICommand::HttpConnectionList(start, PAGE_SIZE);
            }
            _ => {
                return PcapUICommand::None;
            }
        }
        PcapUICommand::Refresh
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, event: PcapEvent) -> PcapUICommand {
        match event {
            PcapEvent::Init => PcapUICommand::HttpConnectionList(0, PAGE_SIZE),
            PcapEvent::HttpConnectionList(list) => {
                self.state.update(list);
                PcapUICommand::Refresh
            }
            PcapEvent::HttpContent(request, response) => {
                let mut rs = vec![];
                if let Some(req) = request {
                    rs.push(req);
                }
                if let Some(res) = response {
                    rs.push(res);
                }
                self.detail = Some(HttpHeadersView::new(rs));
                PcapUICommand::Refresh
            }

            // PcapEvent::ConnectionList(list) => {
            //     if let Some((key, title, mut state)) = self.detail.take() {
            //         state.update(list);
            //         self.detail = Some((key, title, state));
            //         return PcapUICommand::Refresh;
            //     }
            //     PcapUICommand::None
            //     // let key = self.detail.as_ref().unwrap().0;
            //     // self.detail.as_mut().unwrap().1.update(list);
            //     // PcapUICommand::Refresh
            // }
            _ => PcapUICommand::None,
        }
    }
}

#[derive(Default, Clone, Eq, Hash, PartialEq)]
pub enum HeaderKey {
    #[default]
    None,
    Message(usize),
    Head(usize, usize),
    Content(usize),
}

#[derive(Default)]
pub struct HttpHeadersView {
    items: Vec<HttpMessageWrap>,
    tree_state: TreeState<HeaderKey>,
}

impl HttpHeadersView {
    pub fn new(items: Vec<HttpMessageWrap>) -> Self {
        Self {
            items,
            tree_state: TreeState::default(),
        }
    }
    pub fn add_header(key: HeaderKey, header: &str) -> TreeItem<HeaderKey> {
        TreeItem::new_leaf(key, header)
    }
    pub fn state(&mut self) -> &mut TreeState<HeaderKey> {
        &mut self.tree_state
    }
    pub fn wrap(items: &[HttpMessageWrap]) -> Vec<TreeItem<HeaderKey>> {
        let mut rs = vec![];
        // let mut index = 0;
        for (index, item) in items.iter().enumerate() {
            let len = item.headers.len();
            let mut child = vec![];
            for inx in 1..len {
                let str = item.headers.get(inx).unwrap();
                child.push(HttpHeadersView::add_header(HeaderKey::Head(index, inx), str));
            }

            if let Some(content) = &item.parsed_content {
                if !content.is_empty() {
                    child.push(TreeItem::new_leaf(HeaderKey::Content(index), format!("Entity: {}", format_bytes_single_unit_int(content.len()))));
                }
            }
            let it = TreeItem::new(HeaderKey::Message(index), item.headers.first().unwrap().clone(), child).expect("need unique id");
            rs.push(it);
        }
        rs
    }
}

impl Widget for &mut HttpHeadersView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // let ch: [Rect; 2] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);

        let list = HttpHeadersView::wrap(&self.items);
        let state = &mut self.tree_state;

        let widget = Tree::new(&list)
            .expect("all item identifiers are unique")
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(None).track_symbol(None).end_symbol(None),
            ))
            .highlight_style(get_active_tab_color().add_modifier(Modifier::BOLD))
            .highlight_symbol("");
        let mut _top = widget.block(content_border_low());

        let selected = state.selected();
        if selected.len() > 1 {
            if let Some(_sel) = selected.last() {
                match _sel {
                    HeaderKey::Content(index) => {
                        if let Some(item) = self.items.get(*index) {
                            let content = item.parsed_content.clone().unwrap_or("".into());
                            let codeview = CodeView::new(content, item.mime);
                            
                            let ch: [Rect; 2] = ratatui::layout::Layout::horizontal([Constraint::Min(5), Constraint::Min(5)]).areas(area);
                            
                            StatefulWidget::render(_top, ch[0], buf, state);
                            let block = content_border_right();
                            let in_area = block.inner(ch[1]);
                            block.render(ch[1], buf);
                            codeview.render(in_area, buf);
                            return;
                        }
                    }
                    _ => {}
                }
            };
        }
        StatefulWidget::render(_top, area, buf, state);
    }
}

impl ControlState for HttpHeadersView {
    fn control(&mut self, _: bool, event: KeyEvent) -> PcapUICommand {
        let state = &mut self.tree_state;
        match event.code {
            KeyCode::Down => {
                state.key_down();
            }
            KeyCode::Up => {
                state.key_up();
            }
            KeyCode::Left => {
                state.key_left();
            }
            KeyCode::Right => {
                state.key_right();
            }
            _ => return PcapUICommand::None,
        };

        PcapUICommand::Refresh
    }

    fn do_render(&mut self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, _: PcapEvent) -> PcapUICommand {
        PcapUICommand::None
    }
}
