/// This file will handle the memory of the gameboy
/// Here is the organisation of the memory
///
/// Interrupt Enable Register
/// --------------------------- FFFF
/// Internal RAM
/// --------------------------- FF80
/// Empty but unusable for I/O
/// --------------------------- FF4C
/// I/O ports
/// --------------------------- FF00
/// Empty but unusable for I/O
/// --------------------------- FEA0
/// Sprite Attrib Memory (OAM)
/// --------------------------- FE00
/// Echo of 8kB Internal RAM
/// --------------------------- E000
/// 8kB Internal RAM
/// --------------------------- C000
/// 8kB switchable RAM bank
/// --------------------------- A000
/// 8kB Video RAM
/// --------------------------- 8000 --
/// 16kB switchable ROM bank          |
/// --------------------------- 4000  | = 32kB Cartrigbe
/// 16kB ROM bank #0                  |
/// --------------------------- 0000 --
use super::cartridge::Cartridge;
use std::ops::Index;
use std::ops::IndexMut;

pub struct Memory {
    // echo of 8kb internal RAM
    internal_ram: [u8; 8 * 1024],   // 8kB
    switchable_ram: [u8; 8 * 1024], // 8kB
    video_ram: [u8; 8 * 1024],      // 8kB
    cartridge: Cartridge,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            internal_ram: [0; 8 * 1024],
            switchable_ram: [0; 8 * 1024],
            video_ram: [0; 8 * 1024],
            cartridge: Cartridge::new(),
        }
    }

    pub fn insert_cartridge(mut self, cartridge: Cartridge) -> Self {
        self.cartridge = cartridge;
        self
    }

    pub fn from_cartridge(cartridge: Cartridge) -> Self {
        Self::new().insert_cartridge(cartridge)
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0x0000...0x7FFF => &self.cartridge[i],
            0x8000...0x9FFF => &self.video_ram[i - 0x8000],
            0xA000...0xBFFF => &self.switchable_ram[i - 0xA000],
            0xC000...0xDFFF => &self.internal_ram[i - 0xC000],
            0xE000...0xFDFF => &self.internal_ram[i - 0xE000],
            _ => unimplemented!(),
        }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        match i {
            0x0000...0x7FFF => &mut self.cartridge[i],
            0x8000...0x9FFF => &mut self.video_ram[i - 0x8000],
            0xA000...0xBFFF => &mut self.switchable_ram[i - 0xA000],
            0xC000...0xDFFF => &mut self.internal_ram[i - 0xC000],
            0xE000...0xFDFF => &mut self.internal_ram[i - 0xE000],
            _ => unimplemented!(),
        }
    }
}
