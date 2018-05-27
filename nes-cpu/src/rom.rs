use std::io::{self, Read};
use util;

pub enum RomError {
    IoError(io::Error),
    FormatError,
}

impl From<io::Error> for RomError {
    fn from(err: io::Error) -> Self {
        RomError::IoError(err)
    }
}

pub struct INesHeader {
    // Header                       16 bytes
    // $4E $45 $53 $1A
    magic: [u8; 4],
    // -> 4   Size of PRG ROM in 16KB units
    pub prg_rom: u8,
    // -> 5   Size of CHR ROM in 8KB units
    pub chr_rom: u8,
    // -> 6   Flags: https://wiki.nesdev.com/w/index.php/INES#Flags_6
    // MMMMATPA
    // M - Lower nibble of mapper number
    pub flags_6: u8,
    // -> 7   Flags: https://wiki.nesdev.com/w/index.php/INES#Flags_7
    // MMMMVVPU
    // M - Upper nibble of mapper number
    // V - If equal to 2, following flags are NES 2.0
    // P - Playchoice system
    // U - VS Unisystem
    pub flags_7: u8,
    // -> 8   Size of PRG RAM in 8KB units
    pub prg_ram: u8,
    // -> 9   Flags: https://wiki.nesdev.com/w/index.php/INES#Flags_9
    // RRRRRRRT
    // R - Reserved, set to 0
    // T - TV system (Not usually honored)
    pub flags_9: u8,
    // -> 10  Flags: https://wiki.nesdev.com/w/index.php/INES#Flags_10
    // Not a part of official specification
    pub flags_10: u8,
    zero: [u8; 5]
}

impl INesHeader {
    fn parse_header(header: [u8; 16]) -> Self {
        INesHeader {
            magic: [
                header[0],
                header[1],
                header[2],
                header[3],
            ],
            prg_rom: header[4],
            chr_rom: header[5],
            flags_6: header[6],
            flags_7: header[7],
            prg_ram: header[8],
            flags_9: header[9],
            flags_10: header[10],
            zero: [0u8; 5]
        }
    }

    fn check_magic(&self) -> bool {
        self.magic == [0x4E, 0x45, 0x53, 0x1A]
    }
}

pub struct Rom {
    header: INesHeader,
    prg: Vec<u8>,
    chr: Vec<u8>
}

impl Rom {
    pub fn load(r: &mut Read) -> Result<Rom, RomError> {
        // iNES header
        let mut header = [0u8; 16];
        try!(util::read_to_buf(&mut header, r));
        let nes_header = INesHeader::parse_header(header);
        if !nes_header.check_magic() {
            return Err(RomError::FormatError);
        } else {
            // Trainer (if present)         0 or 512 bytes
            let prg_rom_bytes = nes_header.prg_rom as usize * 16384;
            let mut prg_rom = vec![0u8; prg_rom_bytes];
            try!(util::read_to_buf(&mut prg_rom, r));

            let chr_rom_bytes = nes_header.chr_rom as usize * 8192;
            let mut chr_rom = vec![0u8; chr_rom_bytes];
            try!(util::read_to_buf(&mut chr_rom, r));

            Ok(Rom{
                header: nes_header,
                prg: prg_rom,
                chr: chr_rom
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_one() {
        assert_eq!(1, 1);
    }
}