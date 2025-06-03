mod bus;
mod cartridge;
mod cpu;

use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};
use crate::{bus::Bus, cpu::CPU};

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
    let bus = Bus::new(rom_data);
    let mut cpu = CPU::new(bus);
    for _ in 0..10 {
        cpu.step();
    }

}

