use std::{cmp, rc::Rc};

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::{self},
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition, Widget}
};
use shark::common::{
    base::Instance,
    concept::{LineData, Lines},
};

use crate::{panel::Panel, theme::reverse_protocol, ControlPanel};

pub struct App {
    instance: Rc<Instance>,
    frame_stat: Option<Lines>,
}

impl App {
    pub fn new(instance: Rc<Instance>) -> Self {
        let frame_stat = instance.statistic_frames().ok();
        Self { instance, frame_stat }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {

        let [top, bottom] = Layout::vertical([Constraint::Length(6), Constraint::Fill(12)]).areas(area);


        let _info = self.instance.pcap_info();
        let pinfos = vec![
            ("Filetype", _info.file_type), 
            ("Frames",format!("{}",_info.frame_count)),
            ("TCP",format!("{}",_info.tcp_count)),
            ("DNS Record",format!("{}",_info.dns_count)),
            ("HTTP Message",format!("{}",_info.http_count)),
            ("Parse Cost",format!("{}",_info.cost)),
        ];

        let ch:[Rect; 6] = Layout::horizontal([Constraint::Fill(1); 6]).areas(top);

        for i in 0..6 {
            let (label, val) = pinfos.get(i).unwrap();
            let _area = ch[i];
            let mut panel = Panel::new(*label, val.as_str());
            panel.render(_area, buf);
        }
        if let Some(f_state) = &self.frame_stat {
            render_line_chart(buf, bottom, f_state);
        }
    }
}


impl ControlPanel for App {
    fn control(&mut self, _: &Event) {
    }
    
}


fn render_line_chart(buf: &mut Buffer, area: Rect, state: &Lines) {
    let mut max = 0;
    // let set = state.get_y();
    // let kind = set.len();
    let count = state.get_x().len() as f64;

    for line_data in state.data().iter() {
        max = cmp::max(max, *(line_data.data().iter().max().unwrap()));
    }
    let _conv = |link: &LineData| -> (String, Vec<(f64, f64)>) {
        let mut _index: f64 = -1.0;
        let data = link
            .data()
            .iter()
            .map(move |f| {
                _index += 1.0;
                return (_index as f64, *f as f64);
            })
            .collect::<Vec<(f64, f64)>>();
        (link.name(), data)
    };
    let mut la = Vec::new();
    for i in 0..state.get_x().len() {
        la.push(format!("{}", i));
    }
    let labels = (&la).iter().map(|f| Line::from(f.clone()));

    let temp = &state.data().iter().map(_conv).collect::<Vec<(String, Vec<(f64, f64)>)>>();
    let data_set = temp.iter().map(|f| Dataset::default().marker(symbols::Marker::Braille).style(reverse_protocol(f.0.as_str())).graph_type(GraphType::Line).data(&f.1));
    let datasets = data_set.collect::<Vec<Dataset>>();


    let chart = Chart::new(datasets)
        .block(Block::bordered())
        .x_axis(Axis::default().style(Style::default().gray()).bounds([0.0, count]).labels(labels))
        .y_axis(Axis::default().style(Style::default().gray()).bounds([0.0, max as f64]).labels(["0".bold(), format!("{}", max).into()]))
        .legend_position(Some(LegendPosition::TopLeft));
    chart.render(area, buf);
}
