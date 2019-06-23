use super::registers::Registers;

pub struct Cpu {
    reg: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
        }
    }
}
