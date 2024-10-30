use std::rc::Rc;

use crossterm::event::Event;
use ratatui::{layout::Constraint, widgets::{Cell, Widget}};
use shark::common::{base::Instance, concept::TCPConversation};

use crate::{control::{DataSource, UITable}, theme::{get_header_style, get_protocol_color}, ControlPanel};


struct TCPMeta {
    source: String,
    target: String,
    s_acc: f32,
    s_throughput: u32,
    t_acc: f32,
    t_throughput: u32,
    count: u16,
    throughput: u32
}
impl From<&TCPConversation> for TCPMeta {
    fn from(v: &TCPConversation) -> Self {
        Self{
            source:v.source().normal_str(), s_acc: v.source().accuracy(), s_throughput: v.source().throughput(),
            target:v.target().normal_str(), t_acc: v.target().accuracy(), t_throughput: v.target().throughput(),
            count: v.count(), throughput: v.throughput()
        }
    }
}

impl DataSource for TCPMeta {
    fn get_header_style() -> ratatui::prelude::Style {
        get_header_style()
    }

    fn select_style() -> ratatui::prelude::Style {
        get_protocol_color("arp")
    }

    fn get_cols() -> Vec<&'static str> {
        vec!["Source", "Target", "s-acc", "s-throughput", "t-acc", "t-throughput", "count", "throughput"]
    }

    fn cols_layout() -> Vec<ratatui::prelude::Constraint> {
        vec![Constraint::Min(15), Constraint::Min(15), Constraint::Length(10), Constraint::Length(8), Constraint::Length(10), Constraint::Length(8), Constraint::Length(8), Constraint::Length(8)]
    }

    fn item_style(&self) -> ratatui::prelude::Style {
        get_protocol_color("tls")
    }

    fn cell_data(&self) -> Vec<ratatui::widgets::Cell> {
        let mut rs: Vec<Cell> = Vec::new();
            rs.push(self.source.clone().into());
            rs.push(self.target.clone().into());
            rs.push(format!("{:.2}%", self.s_acc).into());
            rs.push(format!("{}", self.s_throughput).into());
            rs.push(format!("{:.2}%", self.t_acc).into());
            rs.push(format!("{}", self.t_throughput).into());
            rs.push(format!("{}", self.count).into());
            rs.push(format!("{}", self.throughput).into());
            rs
    }
}


pub struct TCPList {
    instance: Rc<Instance>,
    table: UITable<TCPMeta>,
}

impl TCPList {
    pub fn new(instance: Rc<Instance>) -> Self {
        let items = instance.context().get_conversation_items();
        let mut table = UITable::new();
        table.items(items.iter().map(TCPMeta::from).collect::<Vec<_>>());
        Self{instance, table}
    }
    
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl Widget for &mut TCPList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.table.render(area, buf);
    }
}

impl ControlPanel for TCPList {
    fn control(&mut self, event: &Event) {
        self.table.control(event);
    }
}
