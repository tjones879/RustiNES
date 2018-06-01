use mem::Mem;

pub struct Apu {
    /*
    Registers 	    Channel 	Units
    $4000-$4003 	Pulse 1 	Timer, length counter, envelope, sweep
    $4004-$4007 	Pulse 2 	Timer, length counter, envelope, sweep
    $4008-$400B 	Triangle 	Timer, length counter, linear counter
    $400C-$400F 	Noise 	    Timer, length counter, envelope, linear feedback shift register
    $4010-$4013 	DMC 	    Timer, memory reader, sample buffer, output unit
    $4015 	        All 	    Channel enable and length counter status
    $4017 	        All     	Frame counter
    */
    pulse_1: [u8; 4],
    pulse_2: [u8; 4],
    triangle: [u8; 4],
    noise: [u8; 4],
    dmc: [u8; 4],
    status: u8,
    frame_counter: u8
}

impl Apu {
    fn getMemLocation(&mut self, addr: usize) -> &mut u8 {
        match addr {
            0...3 => {
                &mut self.pulse_1[addr % 4]
            }
            4...7 => {
                &mut self.pulse_2[addr % 4]
            }
            8...11 => {
                &mut self.triangle[addr % 4]
            }
            12...15 => {
                &mut self.noise[addr % 4]
            }
            16...19 => {
                &mut self.dmc[addr % 4]
            }
            20 => {
                &mut self.status
            }
            21 => {
                &mut self.frame_counter
            }
            _ => {
                panic!("Invalid address in Apu")
            }
        }
    }
}

impl Mem for Apu {
    fn loadb(&mut self, addr: u16) -> u8 {
        *self.getMemLocation(addr as usize)
    }
    fn storeb(&mut self, addr: u16, val: u8) {
        *self.getMemLocation(addr as usize) = val;
    }
}