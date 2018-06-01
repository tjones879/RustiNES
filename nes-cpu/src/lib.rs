pub mod cpu;
pub mod mem;
pub mod rom;
pub mod util;
pub mod ppu;
pub mod apu;
pub mod ioport;
/*
pub struct Nes {
    cycle: u64,
    cpu: cpu::NesCpu,
    mem: mem::MemoryMap
}

impl Nes {
    pub fn step(&mut self, count: u64) {
        self.cycle += count;
        self.cpu.step_to(self.cycle);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/