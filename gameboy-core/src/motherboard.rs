mod cartridge;
mod cpu;
mod memory;
mod registers;

use self::cpu::Cpu;
use self::memory::Memory;

pub struct Motherboard {
    cpu: Cpu,
    memory: Memory,
}
