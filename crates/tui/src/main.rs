use std::{fs::{self}, sync::mpsc};

use clap::Parser;
use pcaps::{engine::{PcapCommand, PcapEvent, Service}, ui};




/// ----- pcapviewer ------
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let fname = args.file;
    if !fs::exists(&fname).unwrap() {
        eprintln!("File [{}] not exists", fname);
        std::process::exit(1);
    }
    start(&fname)
}

fn start(_fname: &str) -> anyhow::Result<()> {
    let (etx, erx) = mpsc::channel::<PcapEvent>();
    let (ptx, prx) = mpsc::channel::<PcapCommand>();
    let ui = ui::UI::new(ptx, erx);
    let mut engine = Service::new(_fname.to_string(), etx, prx);
    let logic_handle = std::thread::spawn(move || {
        engine.run().unwrap();
    });
    ui.run()?;
    logic_handle.join().unwrap();
    Ok(())
}
