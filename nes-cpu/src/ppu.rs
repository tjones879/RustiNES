use mem::Mem;

use std::ops::Deref;

pub struct Ppu {
    /*
    Common Name 	Address 	Bits 	    Notes
    PPUCTRL         $2000 	    VPHB SINN 	NMI enable (V), PPU master/slave (P), sprite height (H), background tile select (B), sprite tile select (S), increment mode (I), nametable select (NN)
    PPUMASK         $2001 	    BGRs bMmG 	color emphasis (BGR), sprite enable (s), background enable (b), sprite left column enable (M), background left column enable (m), greyscale (G)
    PPUSTATUS       $2002 	    VSO- ---- 	vblank (V), sprite 0 hit (S), sprite overflow (O), read resets write pair for $2005/2006
    OAMADDR         $2003 	    aaaa aaaa 	OAM read/write address
    OAMDATA         $2004 	    dddd dddd 	OAM data read/write
    PPUSCROLL       $2005 	    xxxx xxxx 	fine scroll position (two writes: X, Y)
    PPUADDR         $2006 	    aaaa aaaa 	PPU read/write address (two writes: MSB, LSB)
    PPUDATA         $2007 	    dddd dddd 	PPU data read/write
    OAMDMA 	        $4014 	    aaaa aaaa 	OAM DMA high address 
    */
}

impl Mem for Ppu {
    fn loadb(&mut self, addr: u16) -> u8 {
        0
    }

    fn storeb(&mut self, addr: u16, val: u8) {

    }
}

impl Ppu {
    fn getReg(&mut self, addr: u16) -> u8 {
        0
    }
}