use shark::common::base::Instance;
use shark::entry::load_data;
use pcaps::ui::MainUI;
use std::fs;
use std::process;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use pcaps::Result;
// use color_eyre::Result;
use clap::Parser;

/// ----- pcapviewer ------
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> { 
    let args = Args::parse();
    let fname = args.file;
    // let fname = "./sandbox/11.pcapng"; 
    // let fname = "./sandbox/dns.pcapng";
    if !fs::exists(fname.clone()).unwrap() {
        eprintln!("File [{fname}] not exists");
        process::exit(1);
    }
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


fn show (instance: Instance) {
    let mut ui = MainUI::new(Rc::new(instance));
    ui.run();
}


fn show_error() {

}
