use super::memory::Memory;
use super::registers::Registers;
use std::sync::Arc;

pub struct Cpu {
    reg: Registers,
    mem: Arc<Memory>,
}

impl Cpu {
    pub fn new(mem: Arc<Memory>) -> Self {
        Cpu {
            reg: Registers::new(mem.clone()),
            mem,
        }
    }

    /// return the number of cycle to compute the instruction
    fn LD_nn_n(&mut self) -> u8 {
        8
    }
}
