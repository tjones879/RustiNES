mod rom;
mod util;
mod mem;
mod cpu;
mod apu;
mod ppu;
mod ioport;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn open_file() -> Result<u8, std::io::Error> {
    let f = File::open("roms/nestest.nes")?;
    let mut reader = BufReader::new(f);
    let nes_result = rom::Rom::load(&mut reader);
    let nes = nes_result.unwrap();
    writeln!(std::io::stdout(), "Header: {}", nes.header);
    Ok(1)
}

fn main() {
    open_file();
}