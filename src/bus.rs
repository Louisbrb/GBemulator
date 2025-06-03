use crate::cartridge::Cartridge;
pub struct Bus {
    cartridge: Cartridge,          // Cartridge ROM
    vram: [u8; 0x2000],    // 8KB Video RAM
    eram: [u8; 0x2000],    // 8KB External RAM
    wram: [u8; 0x2000],    // 8KB Work RAM
    oam:  [u8; 0xA0],      // Sprite OAM
    hram: [u8; 0x7F],      // High RAM
    ie:   u8,              // Interrupt Enable Register
}

impl Bus {
    pub fn new(rom_data: Vec<u8>) -> Self {
        Self {
            cartridge: Cartridge::new(rom_data),
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam:  [0; 0xA0],
            hram: [0; 0x7F],
            ie: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize], // Echo
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
            _ => 0xFF, // Unmapped or unused
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = value, 
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
            _ => {} 
        }
    }
}
