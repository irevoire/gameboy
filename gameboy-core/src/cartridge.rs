use std::ops::Index;
use std::ops::IndexMut;

pub struct Cartridge {
    rom_bank_0: [u8; 16 * 1024],
    switchable_rom: [u8; 16 * 1024],
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            rom_bank_0: [0; 16 * 1024],
            switchable_rom: [0; 16 * 1024],
        }
    }
}

impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0x0000...0x3FFF => &self.rom_bank_0[i],
            0x4000...0x7FFF => &self.switchable_rom[i - 0x4000],
            _ => unimplemented!(),
        }
    }
}

impl IndexMut<usize> for Cartridge {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        match i {
            0x0000...0x3FFF => &mut self.rom_bank_0[i],
            0x4000...0x7FFF => &mut self.switchable_rom[i - 0x4000],
            _ => unimplemented!(),
        }
    }
}
