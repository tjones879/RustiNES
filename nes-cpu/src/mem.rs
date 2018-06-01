use ppu::Ppu;
use apu::Apu;
use ioport::IoPort;

use std::ops::Deref;

pub trait Mem {
    // Retrieve a byte at the given 0-based address
    // such that the initial address of each subsystem is 0
    fn loadb(&mut self, addr: u16) -> u8;
    // Write a byte to the given 0-based address
    // such that the initial address of each subsystem is 0
    fn storeb(&mut self, addr: u16, val: u8);
}

pub struct Ram {
    mem: [u8; 0x0800],
}

impl Deref for Ram {
    type Target = [u8; 0x0800];

    fn deref(&self) -> &[u8; 0x0800] {
        &self.mem
    }
}

impl Mem for Ram {
    fn loadb(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn storeb(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }
}

pub struct MemoryMap {
    /*
    $0000-$07FF     $0800 	2KB internal RAM
    $0800-$0FFF     $0800 	Mirrors of $0000-$07FF
    $1000-$17FF     $0800
    $1800-$1FFF     $0800
    $2000-$2007     $0008 	NES PPU registers
    $2008-$3FFF     $1FF8 	Mirrors of $2000-2007 (repeats every 8 bytes)
    $4000-$4017     $0018 	NES APU and I/O registers
    $4018-$401F     $0008 	APU and I/O functionality that is normally disabled. See CPU Test Mode.
    $4020-$FFFF     $BFE0 	Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note) 
    */
    ram: Ram,
    ppu_regs: Ppu,
    apu_regs: Apu,
    joy1: IoPort,
    joy2: IoPort,
    // TODO: Mappers
}

impl MemoryMap {
}

impl Mem for MemoryMap {
    fn loadb(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000...0x1FFF => {
                // RAM has 3 mirrors, all are based on the bottom 12 bits
                self.ram.loadb(addr & 0x07FF)
            }
            0x2000...0x3FFF => {
                // PPU is mirrored every 8 bytes
                self.ppu_regs.loadb(addr % 8)
            }
            0x4000...0x4015 => {
                self.apu_regs.loadb(addr - 0x4000)
            }
            0x4016 => {
                self.joy1.loadb(0)
            }
            0x4017 => {
                self.joy2.loadb(0)
            }
            0x4020...0xFFFF => {
                // TODO
                0
            }
            _ => {
                self.ram.loadb(addr)
            }
        }
    }

    fn storeb(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000...0x1FFF => {
                self.ram.storeb(addr & 0x07FFF, value);
            }
            0x2000...0x3FFF => {
                self.ppu_regs.storeb(addr % 8, value);
            }
            0x4000...0x4015 => {
                self.apu_regs.storeb(addr - 0x4000, value);
            }
            0x4016 => {
                self.joy1.storeb(0, value);
            }
            0x4017 => {
                self.joy2.storeb(0, value);
            }
            0x4020...0xFFFF => {
                // TODO
            }
            _ => {
                self.ram.storeb(addr, value);
            }
        }
    }
}