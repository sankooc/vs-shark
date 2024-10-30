use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState, Widget},
};

use crate::ControlPanel;

pub trait DataSource {
    fn get_header_style() -> Style;
    fn select_style() -> Style;
    fn get_cols() -> Vec<&'static str>;
    fn cols_layout() -> Vec<Constraint>;
    fn item_style(&self) -> Style;
    fn cell_data(&self) -> Vec<Cell>;
}

pub struct UITable<T>
where
    T: DataSource,
{
    state: TableState,
    scroll_state: ScrollbarState,
    items: Vec<T>,
}

impl<T> UITable<T>
where
    T: DataSource,
{
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(0),
        }
    }
    pub fn items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state.select(Some(0));
        self.scroll_state = ScrollbarState::new(self.items.len());
    }
    pub fn next_row(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    return;
                } else {
                    let inx = i + 1;
                    self.state.select(Some(inx));
                    self.scroll_state = self.scroll_state.position(inx);
                }
            }
            None => {},
        };
    }
    pub fn previous_row(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    return;
                } else {
                    let inx = i - 1;
                    self.state.select(Some(inx));
                    self.scroll_state = self.scroll_state.position(inx);
                }
            }
            None => {},
        };
    }
}

impl<T> Widget for &mut UITable<T>
where
    T: DataSource,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_style = T::get_header_style();
        let cols = T::get_cols();
        let header = cols.into_iter().map(Cell::from).collect::<Row>().style(header_style).height(1);

        let rows = self.items.iter().map(|data| {
            let rs: Vec<Cell> = data.cell_data();
            let row_style = data.item_style();
            rs.into_iter().collect::<Row>().style(row_style).height(1)
        });

        let select_row_style = T::select_style();

        let t: Table<'_> = Table::new(rows, T::cols_layout()).header(header).highlight_style(select_row_style).highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(t, area, buf, &mut self.state);

        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scroll.render(area, buf, &mut self.scroll_state);
    }
}

impl<T> ControlPanel for UITable<T>
where
    T: DataSource,
{
    fn control(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Down => self.next_row(),
                KeyCode::Up => self.previous_row(),
                _ => {}
            }
        }
    }
}
