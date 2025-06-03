use crate::bus::Bus;

pub struct CPU {
    pub reg_a: u8,
    pub reg_f: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_h: u8,
    pub reg_l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool, 
    pub halted: bool,
    pub bus: Bus,
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        Self {
            reg_a: 0x01,
            reg_f: 0xB0, 
            reg_b: 0x00,
            reg_c: 0x13,
            reg_d: 0x00,
            reg_e: 0xD8,
            reg_h: 0x01,
            reg_l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100, 
            ime: false,
            halted: false,
            bus,
        }
    }

    
    fn af(&self) -> u16 { ((self.reg_a as u16) << 8) | (self.reg_f as u16 & 0xF0) }
    fn bc(&self) -> u16 { ((self.reg_b as u16) << 8) | self.reg_c as u16 }
    fn de(&self) -> u16 { ((self.reg_d as u16) << 8) | self.reg_e as u16 }
    fn hl(&self) -> u16 { ((self.reg_h as u16) << 8) | self.reg_l as u16 }

    fn set_af(&mut self, val: u16) {
        self.reg_a = (val >> 8) as u8;
        self.reg_f = val as u8 & 0xF0; 
    }
    fn set_bc(&mut self, val: u16) {
        self.reg_b = (val >> 8) as u8;
        self.reg_c = val as u8;
    }
    fn set_de(&mut self, val: u16) {
        self.reg_d = (val >> 8) as u8;
        self.reg_e = val as u8;
    }
    fn set_hl(&mut self, val: u16) {
        self.reg_h = (val >> 8) as u8;
        self.reg_l = val as u8;
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        match opcode {
            0x00 => {
                // NOP
            }
            0x3E => {
                // LD A, d8
                let val = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.reg_a = val;
            }
            0x06 => {
                // LD B, d8
                let val = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.reg_b = val;
            }
            0x04 => {
                // INC B
                self.reg_b = self.reg_b.wrapping_add(1);
                self.set_flags(self.reg_b == 0, false, (self.reg_b & 0x0F) == 0x00, self.flag_c());
            }
            0xC3 => {
                // JP a16
                let lo = self.bus.read(self.pc) as u16;
                let hi = self.bus.read(self.pc + 1) as u16;
                self.pc = (hi << 8) | lo;
            }
            _ => {
                println!("PANIC !!!!!!");
            }
        }
    }

    fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.reg_f = 0;
        if z { self.reg_f |= 0x80; }
        if n { self.reg_f |= 0x40; }
        if h { self.reg_f |= 0x20; }
        if c { self.reg_f |= 0x10; }
    }

    fn flag_c(&self) -> bool {
        self.reg_f & 0x10 != 0
    }
}
