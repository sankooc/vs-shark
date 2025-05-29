use std::{fs::{self, File}, sync::mpsc};

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
    // let args = Args::parse();
    // let fname = args.file;
    // let fname = "./sandbox/11.pcapng";
    let fname = "/home/sankooc/repo/pcapview/crates/tui/sandbox/ww.pcap";
    // let fname = "/home/sankooc/repo/pcaps/big-2.pcap";
    // let fname = "/home/sankooc/repo/pcapview/crates/tui/sandbox/11.pcapng";
    if !fs::exists(fname).unwrap() {
        eprintln!("File [{fname}] not exists");
        std::process::exit(1);
    }
    start(fname)
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
