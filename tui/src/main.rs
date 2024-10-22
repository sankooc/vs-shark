use tui::ui;
use tui::Result;
// use color_eyre::Result;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

fn main() {
    // let args = Args::parse();
    // println!("{}", args.file)
    start().unwrap();
}

fn start() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = tui::loading::App::default().run(terminal);
    // let app_result = ui::render(terminal);
    ratatui::restore();
    app_result
}