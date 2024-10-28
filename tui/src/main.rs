use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use shark::common::base::Instance;
use shark::entry::load_data;
use shark_tui::ui::MainUI;
use std::fs;
use std::rc::Rc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use shark_tui::Result;
// use color_eyre::Result;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    // let args = Args::parse(q); 
    // println!("{}", args.file) 
    let fname = "./sandbox/11.pcapng"; 
    // let fname = "./sandbox/dns.pcapng";
    
    let data: Vec<u8> = fs::read(fname).unwrap();
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
    if let Ok(mut _ctx) = load_data(&data) {
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        let ctx = &mut _ctx.ctx;
        ctx.cost = end - start;
        show(_ctx);
    } else {
        show_error();
    }
    Ok(()) 
}
 
enum AppState {
    NORMAL,
    QUIT,
}


struct TApplication {
    state: AppState,
    terminal: DefaultTerminal,
    ui: MainUI,
}

impl TApplication {
    fn new(_instance: Instance, terminal: DefaultTerminal) -> Self {
        let instance = Rc::new(_instance);
        let ui = MainUI::new(instance.clone());
        Self{ui, terminal, state: AppState::NORMAL}
    }
    fn run(&mut self) {
        loop {
            if let AppState::QUIT = self.state {
                break;
            }
        
            let next_event = self.handle_events();
            self.ui.update(&mut self.terminal, next_event).unwrap();
        }
    }
    pub fn handle_events(&mut self) -> Option<Event> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        let mut _event: Option<Event> = None;
        if let Ok(_get) = event::poll(timeout) {
            if _get {
                if let Ok(_key) = event::read() {
                    if let Event::Key(key) = &_key {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::QUIT,
                                _ => {}
                            }
                        }
                    }
                    _event = Some(_key);
                }
            }
        }
        _event
    }
}


fn show (instance: Instance) {
    let terminal = ratatui::init();
    let mut gui = TApplication::new(instance,terminal);
    gui.run();
    ratatui::restore();
}


fn show_error() {

}
