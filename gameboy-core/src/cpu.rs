use super::memory::Memory;
use super::registers::Registers;
use std::sync::Arc;

struct Clock {
    m: u64,
    t: u64,
}

impl Clock {
    fn new() -> Self {
        Clock { m: 0, t: 0 }
    }
}

pub struct Cpu {
    reg: Registers,
    clk: Clock,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Registers::new(),
            clk: Clock::new(),
        }
    }

    /// return the number of cycle to compute the instruction
    fn LD_nn_n(&mut self) -> u8 {
        8
    }
}
