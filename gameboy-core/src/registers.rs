#![allow(non_snake_case)]
use super::memory::Memory;
use std::ops::Index;
use std::ops::IndexMut;
use std::sync::Arc;

pub struct Registers {
    /// general purpose registers
    A: u8, // acc / arg
    F: u8, // flags

    B: u8,
    C: u8,

    D: u8,
    E: u8,

    H: u8, // addr
    L: u8, // addr

    /// special purpose registers
    sp: u16, // stack pointer
    pc: u16, // program pointer
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0,
            F: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            sp: 0,
            pc: 0x100,
        }
    }

    /// Z: Math operation resulted in zero
    pub fn zero(&self) -> bool {
        (self.F >> 7) == 1
    }

    /// N: Math operation used substraction
    pub fn substraction(&self) -> bool {
        ((self.F >> 6) & 1) == 1
    }

    /// H: Math operation raised half-carry
    pub fn half_carry(&self) -> bool {
        ((self.F >> 5) & 1) == 1
    }

    /// C: Math operation raised carry
    pub fn carry(&self) -> bool {
        ((self.F >> 4) & 1) == 1
    }

    pub fn HL(&self) -> u16 {
        ((self.H as u16) << 8) | (self.L as u16)
    }
}

impl Index<usize> for Registers {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0x7 | 0xF => &self.A,
            0x0 | 0x8 => &self.B,
            0x1 | 0x9 => &self.C,
            0x2 | 0xA => &self.D,
            0x3 | 0xB => &self.E,
            0x4 | 0xC => &self.H,
            0x5 | 0xD => &self.L,
            index => panic!("Access to an unknow register {}", index),
        }
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        match i {
            0x7 | 0xF => &mut self.A,
            0x0 | 0x8 => &mut self.B,
            0x1 | 0x9 => &mut self.C,
            0x2 | 0xA => &mut self.D,
            0x3 | 0xB => &mut self.E,
            0x4 | 0xC => &mut self.H,
            0x5 | 0xD => &mut self.L,
            index => panic!("Access to an unknow register {}", index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Registers {
        Registers::new(Arc::new(Memory::new()))
    }

    #[test]
    fn test_init() {
        let r = init();
        assert_eq!(r.pc, 0x100);
    }
}
