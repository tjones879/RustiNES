use mem;
use std::ops::Deref;
/*
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
        // TODO
        0
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
        // TODO
        // cpu.loadb(**self)
        0
    }
    fn store(&self, cpu: &mut NesCpu<M>, _: u8) {
        // TODO
        // cpu.storeb(**self)
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

pub struct NesCpu {
    clock: u64,
    regs: Registers,
}

impl NesCpu {
    pub fn step_to(&mut self, cycle: u64) {
        while self.clock < cycle {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) -> u64 {
        // Check if there is CPU Interrupt, handle it if so
        // Fetch the next instruction
        // Prepare the operand according to addressing mode
        // Execute instruction
        // Update PC and return the CPU cycles
    }
}
*/