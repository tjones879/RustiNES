use mem::Mem;

pub struct IoPort {
    dummy: u8
}

impl Mem for IoPort {
    fn loadb(&mut self, addr: u16) -> u8 {
        self.dummy
    }
    // Write a byte to the given 0-based address
    // such that the initial address of each subsystem is 0
    fn storeb(&mut self, addr: u16, val: u8) {

    }
}