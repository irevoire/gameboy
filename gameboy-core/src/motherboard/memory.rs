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

pub struct Memory {
    // echo of 8kb internal RAM
    internal_ram: [u8; 8 * 1024],   // 8kB
    switchable_ram: [u8; 8 * 1024], // 8kB
    video_ram: [u8; 8 * 1024],      // 8kB
    cartridge: Option<Cartridge>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            internal_ram: [0; 8 * 1024],
            switchable_ram: [0; 8 * 1024],
            video_ram: [0; 8 * 1024],
            cartridge: None,
        }
    }

    pub fn insert_cartridge(mut self, cartridge: Cartridge) -> Self {
        self.cartridge = Some(cartridge);
        self
    }

    pub fn from_cartridge(cartridge: Cartridge) -> Self {
        Self::new().insert_cartridge(cartridge)
    }
}
