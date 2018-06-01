use mem::Mem;
use std::ops::Deref;

trait AddressingMode<M: Mem> {
    fn load(&self, cpu: &mut NesCpu<M>) -> u8;
    fn store(&self, cpu: &mut NesCpu<M>, val: u8);
}

struct AccumulatorAddressingMode;
impl<M: Mem> AddressingMode<M> for AccumulatorAddressingMode {
    fn load(&self, cpu: &mut NesCpu<M>) -> u8 {
        cpu.regs.a
    }
    fn store(&self, cpu: &mut NesCpu<M>, val: u8) {
        cpu.regs.a = val;
    }
}

// Allows the programmer to directly specify an 8 bit constant
struct ImmediateAddressingMode;
impl<M: Mem> AddressingMode<M> for ImmediateAddressingMode {
    fn load(&self, cpu: &mut NesCpu<M>) -> u8 {
        cpu.load_pc_bump()
    }
    fn store(&self, cpu: &mut NesCpu<M>, _: u8) {
        panic!("Can't store to an immediate value")
    }
}

// TODO: Memory Addressing Mode
struct MemoryAddressingMode {
    val: u16
}

impl Deref for MemoryAddressingMode {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.val
    }
}

impl<M: Mem> AddressingMode<M> for MemoryAddressingMode {
    fn load(&self, cpu: &mut NesCpu<M>) -> u8 {
        cpu.loadb(**self)
    }
    fn store(&self, cpu: &mut NesCpu<M>, val: u8) {
        cpu.storeb(**self, val);
    }
}

// Decode the given opcode
fn decode_op() {

}

struct Registers {
    // Accumulator
    a: u8,
    // Indexes
    x: u8,
    y: u8,
    // Program Counter
    pc: u16,
    // Stack pointer
    sp: u8,
    // Status Register
    status: u8
}

pub struct NesCpu<M: Mem> {
    clock: u64,
    regs: Registers,
    pub mem: M
}

enum MemRegType {
    X,
    Y,
    NoType
}

impl<M: Mem> Mem for NesCpu<M> {
    fn loadb(&mut self, addr: u16) -> u8 {
        self.mem.loadb(addr)
    }

    fn storeb(&mut self, addr: u16, val: u8) {
        self.mem.storeb(addr, val);
    }
}

impl<M: Mem> NesCpu<M> {
    pub fn step_to(&mut self, cycle: u64) {
        while self.clock < cycle {
            self.execute_instruction();
        }
    }

    fn load_pc_bump(&mut self) -> u8 {
        let pc = self.regs.pc;
        let val = self.loadb(pc);
        self.regs.pc += 1;
        val
    }

    fn loadw_pc_bump(&mut self) -> u16 {
        let pc = self.regs.pc;
        let val = self.loadw(pc);
        self.regs.pc += 2;
        val
    }

    fn loadw_from_zp(&mut self, base: u16) -> u16 {
        let lower = self.loadb(base % 256) as u16;
        let higher = self.loadb((base + 1) % 256) as u16;
        lower | higher << 8
    }

    fn execute_instruction(&mut self) -> u64 {
        // Check if there is CPU Interrupt, handle it if so
        // Fetch the next instruction
        // Prepare the operand according to addressing mode
        // Execute instruction
        // Update PC and return the CPU cycles
        0
    }

    // Memory addressing modes
    fn accumulator(&mut self) -> AccumulatorAddressingMode {
        AccumulatorAddressingMode
    }

    fn immediate(&mut self) -> ImmediateAddressingMode {
        ImmediateAddressingMode
    }

    fn zero_page(&mut self, zero_type: MemRegType) -> MemoryAddressingMode {
        MemoryAddressingMode {
            val: match zero_type {
                MemRegType::X => {
                    (self.load_pc_bump() + self.regs.x) as u16
                },
                MemRegType::Y => {
                    (self.load_pc_bump() + self.regs.y) as u16
                },
                MemRegType::NoType => { 
                    self.load_pc_bump() as u16
                }
            }
        }
    }

    fn absolute(&mut self, abs_type: MemRegType) -> MemoryAddressingMode {
        MemoryAddressingMode {
            val: match abs_type {
                MemRegType::X => {
                    self.loadw_pc_bump() + self.regs.x as u16
                },
                MemRegType::Y => {
                    self.loadw_pc_bump() + self.regs.y as u16
                },
                MemRegType::NoType => {
                    self.loadw_pc_bump()
                }
            }
        }
    }

    fn indirect(&mut self, ind_type: MemRegType) -> MemoryAddressingMode {
        let ptr = self.load_pc_bump();
        MemoryAddressingMode {
            val: match ind_type {
                MemRegType::X => {
                    let x = self.regs.x;
                    self.loadw_from_zp(ptr as u16 + x as u16)
                },
                MemRegType::Y => {
                    let y = self.regs.y;
                    self.loadw_from_zp(ptr as u16) + y as u16
                },
                MemRegType::NoType => {
                    panic!("There indirect addresses must always utilize X or Y")
                }
            }
        }
    }
}