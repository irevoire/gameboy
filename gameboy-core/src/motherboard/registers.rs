#[allow(non_snake_case)]

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
}
