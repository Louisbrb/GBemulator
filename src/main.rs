mod bus;

use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};
use bus::Bus;

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
    let mut bus = Bus::new(rom_data);

    println!(" initialised test read from 0x100: {:#X}", bus.read(0x100));

}

