pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
        pub fn new(rom_data: Vec<u8>) -> Self {
        Self { rom: rom_data }
    }
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            _ => 0xFF,
        }
    }
}
