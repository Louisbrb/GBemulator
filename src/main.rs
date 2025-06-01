use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
struct Args {
    rom: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut rom_data = Vec::new();
    File::open(&args.rom)
        .expect("Could not open ROM")
        .read_to_end(&mut rom_data)
        .expect("Could not read ROM");

    println!("Loaded ROM: {} ({} bytes)", args.rom.display(), rom_data.len());
}

