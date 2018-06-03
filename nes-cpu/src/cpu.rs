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

enum Flags {
    Carry = 1 << 0,
    Zero = 1 << 1,
    Interrupt = 1 << 2,
    Decimal = 1 << 3,
    Overflow = 1 << 6,
    Negative = 1 << 7
}

impl Registers {
    pub fn flag_set(&self, flag: Flags) -> bool {
        self.status & flag as u8 != 0
    }

    pub fn save_flag(&mut self, flag: Flags, state: bool) {
        if state {
            self.status = self.status | flag as u8;
        } else {
            self.status = self.status & !(flag as u8);
        }
    }

    pub fn check_zero(&mut self, value: u8) {
        self.save_flag(Flags::Zero, value == 0);
    }

    pub fn check_negative(&mut self, value: u8) {
        self.save_flag(Flags::Negative, value & 0x80 != 0)
    }
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

    // Add with carry
    fn adc<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let mut result = self.regs.a as u32 + val as u32;
        if self.regs.flag_set(Flags::Carry) {
            result += 1
        }

        self.regs.save_flag(Flags::Carry, (result & 0x100) != 0);

        let result = result as u8;
        let twos = result ^ self.regs.a;
        self.regs.save_flag(Flags::Overflow, (twos & 0x80 == 0) && (twos & 0x80 == 0x80));
        self.regs.check_negative(result);
        self.regs.check_zero(result);
    }

    // Bitwise and with accumulator
    fn and<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let result = self.regs.a & val;

        self.regs.a = result;
        self.regs.check_negative(result);
        self.regs.check_zero(result);
    }

    // Arithmetic Shift Left
    fn asl<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let old_val = mode.load(self);
        let new_val = old_val << 1;
        
        self.regs.save_flag(Flags::Carry, old_val & (1 << 7) != 0);
        self.regs.a = new_val;
        self.regs.check_negative(new_val);
        self.regs.check_zero(new_val);
    }

    // Test bits
    fn bit<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let result = val & self.regs.a;

        self.regs.save_flag(Flags::Overflow, result & (1 << 6) != 0);
        self.regs.check_negative(result);
        self.regs.check_zero(result);
    }

    // Branch Instructions
    fn try_branch(&mut self, flag: bool) {
        // Not 100% sure that this is correct
        if flag {
            let rel_addr = self.load_pc_bump();
            self.regs.pc += rel_addr as u16;
        }
    }

    // Branch on plus
    fn bpl<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let neg_flag = self.regs.flag_set(Flags::Negative);
        self.try_branch(!neg_flag);
    }

    // Branch on minus
    fn bmi(&mut self) {
        let neg_flag = self.regs.flag_set(Flags::Negative);
        self.try_branch(neg_flag);
    }

    // Branch on overflow clear
    fn bvc(&mut self) {
        let ov_flag = self.regs.flag_set(Flags::Overflow);
        self.try_branch(!ov_flag);
    }

    // Branch on overflow set
    fn bvs(&mut self) {
        let ov_flag = self.regs.flag_set(Flags::Overflow);
        self.try_branch(ov_flag);
    }

    // Branch on carry clear
    fn bcc(&mut self) {
        let carry_flag = self.regs.flag_set(Flags::Carry);
        self.try_branch(!carry_flag);
    }

    // Branch on carry set
    fn bcs(&mut self) {
        let carry_flag = self.regs.flag_set(Flags::Carry);
        self.try_branch(carry_flag);
    }

    // Branch on not equal
    fn bne(&mut self) {
        let zero_flag = self.regs.flag_set(Flags::Zero);
        self.try_branch(!zero_flag);
    }

    // Branch on equal
    fn beq(&mut self) {
        let zero_flag = self.regs.flag_set(Flags::Zero);
        self.try_branch(zero_flag);
    }

    fn brk(&mut self) {

    }

    // Compare accumulator
    fn cmp<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let a = self.regs.a;
        self.regs.save_flag(Flags::Carry, a >= val);
        self.regs.check_zero(a - val);
        self.regs.check_negative(a - val);
    }

    // Compare X register
    fn cpx<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let x = self.regs.x;
        self.regs.save_flag(Flags::Carry, x >= val);
        self.regs.check_zero(x - val);
        self.regs.check_negative(x - val);
    }

    // Compare Y register
    fn cpy<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let y = self.regs.y;
        self.regs.save_flag(Flags::Carry, y >= val);
        self.regs.check_zero(y - val);
        self.regs.check_negative(y - val);
    }

    // Decrement memory
    fn dec<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self) - 1;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
        mode.store(self, val);
    }

    // Decrement X
    fn dex(&mut self) {
        let val = self.regs.x - 1;
        self.regs.x = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Decrement Y
    fn dey(&mut self) {
        let val = self.regs.y - 1;
        self.regs.y = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Bitwise XOR
    fn eor<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let result = self.regs.a ^ val;
        self.regs.a = result;
        self.regs.check_negative(result);
        self.regs.check_zero(result);
    }

    // Increment Memory
    fn inc<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self) + 1;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
        mode.store(self, val);
    }

    // Increment X
    fn inx(&mut self) {
        let val = self.regs.x + 1;
        self.regs.x = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Increment Y
    fn iny(&mut self) {
        let val = self.regs.y + 1;
        self.regs.y = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Jump
    fn jmp(&mut self) {
        // TODO
    }

    // Jump to Subroutine
    fn jsr(&mut self) {
        
    }

    // Load accumulator
    fn lda<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        self.regs.a = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Load X register
    fn ldx<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        self.regs.x = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Load Y register
    fn ldy<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        self.regs.y = val;
        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Logical shift right
    fn lsr<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        self.regs.save_flag(Flags::Carry, val & 1 != 0);
        let val = val >> 1;
        self.regs.check_zero(val);
        self.regs.check_negative(val);
    }

    // No Operation
    fn nop(&mut self) {

    }

    // OR with accumulator
    fn ora<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let result = self.regs.a | val;
        self.regs.a = result;
        self.regs.check_negative(result);
        self.regs.check_zero(result);
    }

    // Rotate left
    fn rol<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let carry = self.regs.flag_set(Flags::Carry);
        self.regs.save_flag(Flags::Carry, val & (1 << 7) != 0);
        let mut val = val << 1;

        if carry {
            val = val | 1;
        }

        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Rotate right
    fn ror<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let val = mode.load(self);
        let carry = self.regs.flag_set(Flags::Carry);
        self.regs.save_flag(Flags::Carry, val & 1 != 0);
        let mut val = val >> 1;

        if carry {
            val = val | (1 << 7);
        }

        self.regs.check_negative(val);
        self.regs.check_zero(val);
    }

    // Return from interrupt
    fn rti(&mut self) {

    }

    // Return from subroutine
    fn rts(&mut self) {

    }

    // Subtract with carry
    fn sbc(&mut self) {

    }

    // Store accumulator
    fn sta<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let a = self.regs.a;
        mode.store(self, a);
    }

    // Store X register
    fn stx<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let x = self.regs.x;
        mode.store(self, x);
    }

    // Store Y register
    fn sty<MODE: AddressingMode<M>>(&mut self, mode: MODE) {
        let y = self.regs.y;
        mode.store(self, y);
    }

    //// Register Instructions
    // Transfer A to X
    fn tax(&mut self) {
        let a = self.regs.a;
        self.regs.x = a;
        self.regs.check_negative(a);
        self.regs.check_zero(a);
    }

    // Transfer X to A
    fn txa(&mut self) {
        let x = self.regs.x;
        self.regs.a = x;
        self.regs.check_negative(x);
        self.regs.check_zero(x);
    }

    // Transfer A to Y
    fn tay(&mut self) {
        let a = self.regs.a;
        self.regs.y = a;
        self.regs.check_negative(a);
        self.regs.check_zero(a);
    }

    // Transfer Y to A
    fn tya(&mut self) {
        let y = self.regs.y;
        self.regs.a = y;
        self.regs.check_negative(y);
        self.regs.check_zero(y);
    }

    //// Processor Status Instructions
    // Clear carry
    fn clc(&mut self) {
        self.regs.save_flag(Flags::Carry, false);
    }

    // Set carry
    fn sec(&mut self) {
        self.regs.save_flag(Flags::Carry, true);
    }

    // Clear interrupt
    fn cli(&mut self) {
        self.regs.save_flag(Flags::Interrupt, false);
    }

    // Set interrupt
    fn sei(&mut self) {
        self.regs.save_flag(Flags::Interrupt, true);
    }

    // Clear overflow
    fn clv(&mut self) {
        self.regs.save_flag(Flags::Overflow, false);
    }

    // Clear decimal
    fn cld(&mut self) {
        self.regs.save_flag(Flags::Decimal, false);
    }

    // Set decimal
    fn sed(&mut self) {
        self.regs.save_flag(Flags::Decimal, true);
    }

    //// Stack Instructions
    // Transfer X to stack ptr
    fn txs(&mut self) {
        self.regs.sp = self.regs.x;
    }

    // Transfer the stack ptr to X
    fn tsx(&mut self) {
        let sp = self.regs.sp;
        self.regs.x = sp;
        self.regs.check_negative(sp);
        self.regs.check_zero(sp);
    }

    fn push(&mut self, val: u8) {

    }

    fn pop(&mut self) -> u8 {
        panic!("Not implemented")
    }

    // Push the accumulator
    fn pha(&mut self) {
        let a = self.regs.a;
        self.push(a);
    }

    // Pop the accumulator
    fn pla(&mut self) {
        let a = self.pop();
        self.regs.a = a;
        self.regs.check_negative(a);
        self.regs.check_zero(a);
    }

    // Push processor status
    fn php(&mut self) {
        let p = self.regs.status;
        self.push(p);
    }

    // Pop processor status
    fn plp(&mut self) {
        let p = self.pop();
        self.regs.status = p;
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
                    panic!("Indirect addresses must always utilize X or Y")
                }
            }
        }
    }
}