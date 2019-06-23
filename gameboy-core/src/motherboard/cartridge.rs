pub struct Cartridge {
    switchable_rom: [u8; 16 * 1024],
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            switchable_rom: [0; 16 * 1024],
        }
    }
}
