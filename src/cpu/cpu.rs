use core::panic;

use crate::bus::Bus;
use crate::registers::Registers;

type InstructionFn = fn(&mut CPU, &mut Bus) -> u8;

pub struct CPU
{
    pub registers: Registers,
    pub isHalted: bool,
    pub imeFlag: bool,
    instructions: [InstructionFn; 256],
    prefixedInstructions: [InstructionFn; 256]
}

impl CPU
{
    pub fn new() -> Self
    {
        let mut cpu = CPU {
            registers: Registers::new(),
            isHalted: false,
            imeFlag: false,
            instructions: [CPU::nop; 256],
            prefixedInstructions: [CPU::nop; 256]
        };

        cpu.instructions[0x01] = CPU::ldBc;
        cpu.instructions[0x02] = CPU::ldBcAddressA;
        cpu.instructions[0x03] = CPU::incBc;
        cpu.instructions[0x04] = CPU::incB;
        cpu.instructions[0x05] = CPU::decB;
        cpu.instructions[0x06] = CPU::ldB;
        cpu.instructions[0x07] = CPU::rlca;
        cpu.instructions[0x08] = CPU::ldAddr16Sp;
        cpu.instructions[0x09] = CPU::addHlBc;
        cpu.instructions[0x0a] = CPU::ldAAddressBc;
        cpu.instructions[0x0b] = CPU::decBc;
        cpu.instructions[0x0c] = CPU::incC;
        cpu.instructions[0x0d] = CPU::decC;
        cpu.instructions[0x0e] = CPU::ldC;
        cpu.instructions[0x0f] = CPU::rrca;
        
        // 0x10
        cpu.instructions[0x11] = CPU::ldDe;
        cpu.instructions[0x12] = CPU::ldDeAddressA;
        cpu.instructions[0x13] = CPU::incDe;
        cpu.instructions[0x14] = CPU::incD;
        cpu.instructions[0x15] = CPU::decD;
        cpu.instructions[0x16] = CPU::ldD;
        cpu.instructions[0x17] = CPU::rla;
        cpu.instructions[0x18] = CPU::jrE8;
        cpu.instructions[0x19] = CPU::addHlDe;
        cpu.instructions[0x1a] = CPU::ldAAddressDe;
        cpu.instructions[0x1b] = CPU::decDe;
        cpu.instructions[0x1c] = CPU::incE;
        cpu.instructions[0x1d] = CPU::decE;
        cpu.instructions[0x1e] = CPU::ldE;
        cpu.instructions[0x1f] = CPU::rra;
        
        cpu.instructions[0x20] = CPU::jrNz;
        cpu.instructions[0x21] = CPU::ldHl;
        cpu.instructions[0x22] = CPU::ldHlPlusAddressA;
        cpu.instructions[0x23] = CPU::incHl;
        cpu.instructions[0x24] = CPU::incH;
        cpu.instructions[0x25] = CPU::decH;
        cpu.instructions[0x26] = CPU::ldH;
        cpu.instructions[0x27] = CPU::daa;
        cpu.instructions[0x28] = CPU::jrZ;
        cpu.instructions[0x29] = CPU::addHlHl;
        cpu.instructions[0x2a] = CPU::ldAAddressHlPlus;
        cpu.instructions[0x2b] = CPU::decHl;
        cpu.instructions[0x2c] = CPU::incL;
        cpu.instructions[0x2d] = CPU::decL;
        cpu.instructions[0x2e] = CPU::ldL;
        cpu.instructions[0x2f] = CPU::cpl;

        cpu.instructions[0x30] = CPU::jrNc;
        cpu.instructions[0x31] = CPU::ldSp;
        cpu.instructions[0x32] = CPU::ldHlMinusAddressA;
        cpu.instructions[0x33] = CPU::incSp;
        cpu.instructions[0x34] = CPU::incAddressHl;
        cpu.instructions[0x35] = CPU::decAddressHl;
        cpu.instructions[0x36] = CPU::ldAddressHl;
        cpu.instructions[0x37] = CPU::scf;
        cpu.instructions[0x38] = CPU::jrC;
        cpu.instructions[0x39] = CPU::addHlSp;
        cpu.instructions[0x3a] = CPU::ldAAddressHlMinus;
        cpu.instructions[0x3b] = CPU::decSp;
        cpu.instructions[0x3c] = CPU::incA;
        cpu.instructions[0x3d] = CPU::decA;
        cpu.instructions[0x3e] = CPU::ldA;
        cpu.instructions[0x3f] = CPU::ccf;

        cpu.instructions[0x40] = CPU::ldBB;
        cpu.instructions[0x41] = CPU::ldBC;
        cpu.instructions[0x42] = CPU::ldBD;
        cpu.instructions[0x43] = CPU::ldBE;
        cpu.instructions[0x44] = CPU::ldBH;
        cpu.instructions[0x45] = CPU::ldBL;
        cpu.instructions[0x46] = CPU::ldBAddressHl;
        cpu.instructions[0x47] = CPU::ldBA;
        cpu.instructions[0x48] = CPU::ldCB;
        cpu.instructions[0x49] = CPU::ldCC;
        cpu.instructions[0x4a] = CPU::ldCD;
        cpu.instructions[0x4b] = CPU::ldCE;
        cpu.instructions[0x4c] = CPU::ldCH;
        cpu.instructions[0x4d] = CPU::ldCL;
        cpu.instructions[0x4e] = CPU::ldCAddressHl;
        cpu.instructions[0x4f] = CPU::ldCA;

        cpu.instructions[0x50] = CPU::ldDB;
        cpu.instructions[0x51] = CPU::ldDC;
        cpu.instructions[0x52] = CPU::ldDD;
        cpu.instructions[0x53] = CPU::ldDE;
        cpu.instructions[0x54] = CPU::ldDH;
        cpu.instructions[0x55] = CPU::ldDL;
        cpu.instructions[0x56] = CPU::ldDAddressHl;
        cpu.instructions[0x57] = CPU::ldDA;
        cpu.instructions[0x58] = CPU::ldEB;
        cpu.instructions[0x59] = CPU::ldEC;
        cpu.instructions[0x5a] = CPU::ldED;
        cpu.instructions[0x5b] = CPU::ldEE;
        cpu.instructions[0x5c] = CPU::ldEH;
        cpu.instructions[0x5d] = CPU::ldEL;
        cpu.instructions[0x5e] = CPU::ldEAddressHl;
        cpu.instructions[0x5f] = CPU::ldEA;

        cpu.instructions[0x60] = CPU::ldHB;
        cpu.instructions[0x61] = CPU::ldHC;
        cpu.instructions[0x62] = CPU::ldHD;
        cpu.instructions[0x63] = CPU::ldHE;
        cpu.instructions[0x64] = CPU::ldHH;
        cpu.instructions[0x65] = CPU::ldHL;
        cpu.instructions[0x66] = CPU::ldHAddressHl;
        cpu.instructions[0x67] = CPU::ldHA;
        cpu.instructions[0x68] = CPU::ldLB;
        cpu.instructions[0x69] = CPU::ldLC;
        cpu.instructions[0x6a] = CPU::ldLD;
        cpu.instructions[0x6b] = CPU::ldLE;
        cpu.instructions[0x6c] = CPU::ldLH;
        cpu.instructions[0x6d] = CPU::ldLL;
        cpu.instructions[0x6e] = CPU::ldLAddressHl;
        cpu.instructions[0x6f] = CPU::ldLA;

        cpu.instructions[0x70] = CPU::ldAddressHlB;
        cpu.instructions[0x71] = CPU::ldAddressHlC;
        cpu.instructions[0x72] = CPU::ldAddressHlD;
        cpu.instructions[0x73] = CPU::ldAddressHlE;
        cpu.instructions[0x74] = CPU::ldAddressHlH;
        cpu.instructions[0x75] = CPU::ldAddressHlL;
        cpu.instructions[0x76] = CPU::halt;
        cpu.instructions[0x77] = CPU::ldAddressHlA;
        cpu.instructions[0x78] = CPU::ldAB;
        cpu.instructions[0x79] = CPU::ldAC;
        cpu.instructions[0x7a] = CPU::ldAD;
        cpu.instructions[0x7b] = CPU::ldAE;
        cpu.instructions[0x7c] = CPU::ldAH;
        cpu.instructions[0x7d] = CPU::ldAL;
        cpu.instructions[0x7e] = CPU::ldAAddressHl;
        cpu.instructions[0x7f] = CPU::ldAA;

        cpu.instructions[0x80] = CPU::addAB;
        cpu.instructions[0x81] = CPU::addAC;
        cpu.instructions[0x82] = CPU::addAD;
        cpu.instructions[0x83] = CPU::addAE;
        cpu.instructions[0x84] = CPU::addAH;
        cpu.instructions[0x85] = CPU::addAL;
        cpu.instructions[0x86] = CPU::addAAddressHl;
        cpu.instructions[0x87] = CPU::addAA;
        cpu.instructions[0x88] = CPU::adcAB;
        cpu.instructions[0x89] = CPU::adcAC;
        cpu.instructions[0x8a] = CPU::adcAD;
        cpu.instructions[0x8b] = CPU::adcAE;
        cpu.instructions[0x8c] = CPU::adcAH;
        cpu.instructions[0x8d] = CPU::adcAL;
        cpu.instructions[0x8e] = CPU::adcAAddressHl;
        cpu.instructions[0x8f] = CPU::adcAA;

        cpu.instructions[0x90] = CPU::subAB;
        cpu.instructions[0x91] = CPU::subAC;
        cpu.instructions[0x92] = CPU::subAD;
        cpu.instructions[0x93] = CPU::subAE;
        cpu.instructions[0x94] = CPU::subAH;
        cpu.instructions[0x95] = CPU::subAL;
        cpu.instructions[0x96] = CPU::subAAddressHl;
        cpu.instructions[0x97] = CPU::subAA;
        cpu.instructions[0x98] = CPU::sbcAB;
        cpu.instructions[0x99] = CPU::sbcAC;
        cpu.instructions[0x9a] = CPU::sbcAD;
        cpu.instructions[0x9b] = CPU::sbcAE;
        cpu.instructions[0x9c] = CPU::sbcAH;
        cpu.instructions[0x9d] = CPU::sbcAL;
        cpu.instructions[0x9e] = CPU::sbcAAddressHl;
        cpu.instructions[0x9f] = CPU::sbcAA;

        cpu.instructions[0xa0] = CPU::andAB;
        cpu.instructions[0xa1] = CPU::andAC;
        cpu.instructions[0xa2] = CPU::andAD;
        cpu.instructions[0xa3] = CPU::andAE;
        cpu.instructions[0xa4] = CPU::andAH;
        cpu.instructions[0xa5] = CPU::andAL;
        cpu.instructions[0xa6] = CPU::andAAddressHl;
        cpu.instructions[0xa7] = CPU::andAA;
        cpu.instructions[0xa8] = CPU::xorAB;
        cpu.instructions[0xa9] = CPU::xorAC;
        cpu.instructions[0xaa] = CPU::xorAD;
        cpu.instructions[0xab] = CPU::xorAE;
        cpu.instructions[0xac] = CPU::xorAH;
        cpu.instructions[0xad] = CPU::xorAL;
        cpu.instructions[0xae] = CPU::xorAAddressHl;
        cpu.instructions[0xaf] = CPU::xorAA;

        cpu.instructions[0xb0] = CPU::orAB;
        cpu.instructions[0xb1] = CPU::orAC;
        cpu.instructions[0xb2] = CPU::orAD;
        cpu.instructions[0xb3] = CPU::orAE;
        cpu.instructions[0xb4] = CPU::orAH;
        cpu.instructions[0xb5] = CPU::orAL;
        cpu.instructions[0xb6] = CPU::orAAddressHl;
        cpu.instructions[0xb7] = CPU::orAA;
        cpu.instructions[0xb8] = CPU::cpAB;
        cpu.instructions[0xb9] = CPU::cpAC;
        cpu.instructions[0xba] = CPU::cpAD;
        cpu.instructions[0xbb] = CPU::cpAE;
        cpu.instructions[0xbc] = CPU::cpAH;
        cpu.instructions[0xbd] = CPU::cpAL;
        cpu.instructions[0xbe] = CPU::cpAAddressHl;
        cpu.instructions[0xbf] = CPU::cpAA;

        cpu.instructions[0xc0] = CPU::retNz;
        cpu.instructions[0xc1] = CPU::popBc;
        cpu.instructions[0xc2] = CPU::jpNz;
        cpu.instructions[0xc3] = CPU::jp;
        cpu.instructions[0xc4] = CPU::callNz;
        cpu.instructions[0xc5] = CPU::pushBc;
        cpu.instructions[0xc6] = CPU::addA;
        cpu.instructions[0xc7] = CPU::rst00;
        cpu.instructions[0xc8] = CPU::retZ;
        cpu.instructions[0xc9] = CPU::ret;
        cpu.instructions[0xca] = CPU::jpZ;
        cpu.instructions[0xcb] = CPU::prefix;
        cpu.instructions[0xcc] = CPU::callZ;
        cpu.instructions[0xcd] = CPU::call;
        cpu.instructions[0xce] = CPU::adcA;
        cpu.instructions[0xcf] = CPU::rst08;

        cpu.instructions[0xd0] = CPU::retNc;
        cpu.instructions[0xd1] = CPU::popDe;
        cpu.instructions[0xd2] = CPU::jpNc;
        cpu.instructions[0xd3] = CPU::notImplemented;
        cpu.instructions[0xd4] = CPU::callNc;
        cpu.instructions[0xd5] = CPU::pushDe;
        cpu.instructions[0xd6] = CPU::subA;
        cpu.instructions[0xd7] = CPU::rst10;
        cpu.instructions[0xd8] = CPU::retC;
        cpu.instructions[0xd9] = CPU::reti;
        cpu.instructions[0xda] = CPU::jpC;
        cpu.instructions[0xdb] = CPU::notImplemented;
        cpu.instructions[0xdc] = CPU::callC;
        cpu.instructions[0xdd] = CPU::notImplemented;
        cpu.instructions[0xde] = CPU::sbcA;
        cpu.instructions[0xdf] = CPU::rst18;

        cpu.instructions[0xe0] = CPU::ldhAddressA;
        cpu.instructions[0xe1] = CPU::popHl;
        cpu.instructions[0xe2] = CPU::ldhAddressCA;
        cpu.instructions[0xe3] = CPU::notImplemented;
        cpu.instructions[0xe4] = CPU::notImplemented;
        cpu.instructions[0xe5] = CPU::pushHl;
        cpu.instructions[0xe6] = CPU::andA;
        cpu.instructions[0xe7] = CPU::rst20;
        cpu.instructions[0xe8] = CPU::addSp;
        cpu.instructions[0xe9] = CPU::jpHl;
        cpu.instructions[0xea] = CPU::ldAddressA;
        cpu.instructions[0xeb] = CPU::notImplemented;
        cpu.instructions[0xec] = CPU::notImplemented;
        cpu.instructions[0xed] = CPU::notImplemented;
        cpu.instructions[0xee] = CPU::xorA;
        cpu.instructions[0xef] = CPU::rst28;

        cpu.instructions[0xf0] = CPU::ldhAAddress;
        cpu.instructions[0xf1] = CPU::popAf;
        cpu.instructions[0xf2] = CPU::ldhAAddressC;
        cpu.instructions[0xf3] = CPU::di;
        cpu.instructions[0xf4] = CPU::notImplemented;
        cpu.instructions[0xf5] = CPU::pushAf;
        cpu.instructions[0xf6] = CPU::orA;
        cpu.instructions[0xf7] = CPU::rst30;
        cpu.instructions[0xf8] = CPU::ldHlSpPlusByte;
        cpu.instructions[0xf9] = CPU::ldSpHl;
        cpu.instructions[0xfa] = CPU::ldAAddress;
        cpu.instructions[0xfb] = CPU::ei;
        cpu.instructions[0xfc] = CPU::notImplemented;
        cpu.instructions[0xfd] = CPU::notImplemented;
        cpu.instructions[0xfe] = CPU::cpA;
        cpu.instructions[0xff] = CPU::rst38;

        cpu.prefixedInstructions[0x00] = CPU::rlcB;
        cpu.prefixedInstructions[0x01] = CPU::rlcC;
        cpu.prefixedInstructions[0x02] = CPU::rlcD;
        cpu.prefixedInstructions[0x03] = CPU::rlcE;
        cpu.prefixedInstructions[0x04] = CPU::rlcH;
        cpu.prefixedInstructions[0x05] = CPU::rlcL;
        cpu.prefixedInstructions[0x06] = CPU::rlcAddressHl;
        cpu.prefixedInstructions[0x07] = CPU::rlcA;
        cpu.prefixedInstructions[0x08] = CPU::rrcB;
        cpu.prefixedInstructions[0x09] = CPU::rrcC;
        cpu.prefixedInstructions[0x0a] = CPU::rrcD;
        cpu.prefixedInstructions[0x0b] = CPU::rrcE;
        cpu.prefixedInstructions[0x0c] = CPU::rrcH;
        cpu.prefixedInstructions[0x0d] = CPU::rrcL;
        cpu.prefixedInstructions[0x0e] = CPU::rrcAddressHl;
        cpu.prefixedInstructions[0x0f] = CPU::rrcA;

        cpu.prefixedInstructions[0x10] = CPU::rlB;
        cpu.prefixedInstructions[0x11] = CPU::rlC;
        cpu.prefixedInstructions[0x12] = CPU::rlD;
        cpu.prefixedInstructions[0x13] = CPU::rlE;
        cpu.prefixedInstructions[0x14] = CPU::rlH;
        cpu.prefixedInstructions[0x15] = CPU::rlL;
        cpu.prefixedInstructions[0x16] = CPU::rlAddressHl;
        cpu.prefixedInstructions[0x17] = CPU::rlA;
        cpu.prefixedInstructions[0x18] = CPU::rrB;
        cpu.prefixedInstructions[0x19] = CPU::rrC;
        cpu.prefixedInstructions[0x1a] = CPU::rrD;
        cpu.prefixedInstructions[0x1b] = CPU::rrE;
        cpu.prefixedInstructions[0x1c] = CPU::rrH;
        cpu.prefixedInstructions[0x1d] = CPU::rrL;
        cpu.prefixedInstructions[0x1e] = CPU::rrAddressHl;
        cpu.prefixedInstructions[0x1f] = CPU::rrA;

        cpu.prefixedInstructions[0x20] = CPU::slaB;
        cpu.prefixedInstructions[0x21] = CPU::slaC;
        cpu.prefixedInstructions[0x22] = CPU::slaD;
        cpu.prefixedInstructions[0x23] = CPU::slaE;
        cpu.prefixedInstructions[0x24] = CPU::slaH;
        cpu.prefixedInstructions[0x25] = CPU::slaL;
        cpu.prefixedInstructions[0x26] = CPU::slaAddressHl;
        cpu.prefixedInstructions[0x27] = CPU::slaA;
        cpu.prefixedInstructions[0x28] = CPU::sraB;
        cpu.prefixedInstructions[0x29] = CPU::sraC;
        cpu.prefixedInstructions[0x2a] = CPU::sraD;
        cpu.prefixedInstructions[0x2b] = CPU::sraE;
        cpu.prefixedInstructions[0x2c] = CPU::sraH;
        cpu.prefixedInstructions[0x2d] = CPU::sraL;
        cpu.prefixedInstructions[0x2e] = CPU::sraAddressHl;
        cpu.prefixedInstructions[0x2f] = CPU::sraA;

        return cpu;

    }

    pub fn step(&mut self, bus: &mut Bus)
    {  
        // TODO: impl interrupts + ticks
        if self.isHalted
        {
            panic!("Halt");
            return;
        }

        let opcode = self.fetch(bus);

        self.execute(opcode, bus);
    }

    fn fetch(&mut self, bus: &mut Bus) -> u8
    {
        let val = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        return val;
    }

    fn fetchU16(&mut self, bus: &mut Bus) -> u16
    {
        let low = self.fetch(bus) as u16;
        let high = self.fetch(bus) as u16;

        let val = (high << 8) | low;

        return val;
    }

    fn incU8(&mut self, value: u8) -> u8
    {
        let incdVal = value.wrapping_add(1);

        self.registers.setFlag(Registers::MASK_ZERO_Z, incdVal == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let overflow = ((value & 0xf).wrapping_add(1)) & 0x10 != 0;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, overflow);

        return incdVal;
    }

    fn decU8(&mut self, value: u8) -> u8
    {
        let decdVal = value.wrapping_sub(1);

        self.registers.setFlag(Registers::MASK_ZERO_Z, decdVal == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, true);

        let overflow = (value & 0x0F) == 0;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, overflow);

        return decdVal;
    }

    fn add(&mut self, num1: u8, num2: u8, withCarry: bool) -> u8
    {
        let carry = if withCarry { self.registers.getFlag(Registers::MASK_CARRY_C) as u8 } else { 0 };

        let sum = (num1 as u16).wrapping_add(num2 as u16).wrapping_add(carry as u16);

        self.registers.setFlag(Registers::MASK_ZERO_Z, (sum as u8) == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = (num1 & 0xf) + (num2 & 0xf) + carry > 0xf;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = sum > 0xff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

        return sum as u8;
    }

    fn addU16(&mut self, num1: u16, num2: u16) -> u16
    {
        let sum = (num1 as u32).wrapping_add(num2 as u32);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        
        let halfCarried = ((num1 as u32) & 0x0fff) + ((num2 as u32) & 0x0fff) > 0x0fff;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = sum > 0xffff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

        return sum as u16;
    }

    fn sub(&mut self, num1: u8, num2: u8, withCarry: bool) -> u8
    {
        let carry = if withCarry { self.registers.getFlag(Registers::MASK_CARRY_C) as u8 } else { 0 };

        let sum = num1.wrapping_sub(num2).wrapping_sub(carry);

        self.registers.setFlag(Registers::MASK_ZERO_Z, sum == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, true);

        let halfCarried = ((num1 as i32) & 0x0f) - ((num2 as i32) & 0x0f) - (carry as i32) < 0;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = (num1 as i32) - (num2 as i32) - (carry as i32) < 0;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

        return sum;
    }

    fn and(&mut self, num1: u8, num2: u8) -> u8
    {
        let val = num1 & num2;

        self.registers.setFlag(Registers::MASK_ZERO_Z, val == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, true);
        self.registers.setFlag(Registers::MASK_CARRY_C, false);

        return val;
    }

    fn xor(&mut self, num1: u8, num2: u8) -> u8
    {
        let val = num1 ^ num2;

        self.registers.setFlag(Registers::MASK_ZERO_Z, val == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, false);

        return val;
    }

    fn or(&mut self, num1: u8, num2: u8) -> u8
    {
        let val = num1 | num2;

        self.registers.setFlag(Registers::MASK_ZERO_Z, val == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, false);

        return val;
    }

    fn popU16(&mut self, bus: &mut Bus) -> u16
    {
        let low = bus.read(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let high = bus.read(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
        
        return low | high << 8;
    }

    fn pushU16(&mut self, bus: &mut Bus, value: u16)
    {
        let high = (value >> 8) as u8;
        let low = (value & 0xff) as u8;

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write(self.registers.sp, high);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write(self.registers.sp, low);
    }

    fn callFn(&mut self, bus: &mut Bus, targetAddress: u16)
    {
        let address = self.registers.pc;

        self.pushU16(bus, address);

        self.registers.pc = targetAddress;
    }

    fn rst(&mut self, bus: &mut Bus, target: u16)
    {
        let address = self.registers.pc;
        self.pushU16(bus, address);

        self.registers.pc = target;
    }

    fn setRotateFlags(&mut self, val: u8, bit: u8)
    {
        self.registers.setFlag(Registers::MASK_ZERO_Z, val == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, bit == 1);
    }

    fn rlc(&mut self, value: u8) -> u8
    {
        let bit = (value & 0x80) >> 7;
        let res = (value << 1) | bit;

        self.setRotateFlags(res, bit);

        return res;
    }

    fn rrc(&mut self, value: u8) -> u8
    {
        let bit = value & 0x01;
        let res = (value >> 1) | (bit << 7);

        self.setRotateFlags(res, bit);

        return res;
    }

    fn rl(&mut self, value: u8) -> u8
    {
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let bit = (value & 0x80) >> 7;
        let res = (value << 1) | oldCarry;

        self.setRotateFlags(res, bit);

        return res;
    }
    
    fn rr(&mut self, value: u8) -> u8
    {
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let bit = value & 0x01;
        let res = (value >> 1) | (oldCarry << 7);

        self.setRotateFlags(res, bit);

        return res;
    }

    fn sla(&mut self, value: u8) -> u8
    {
        let bit = (value & 0x80) >> 7;
        let res = value << 1;

        self.setRotateFlags(res, bit);

        return res;
    }

    fn sra(&mut self, value: u8) -> u8
    {
        let bit = value & 0x01;
        let res = value >> 1;

        self.setRotateFlags(res, bit);

        return res;
    }

    fn rlcAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.rlc(val);
        bus.write(address, res);
    }

    fn rrcAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.rrc(val);
        bus.write(address, res);
    }

    fn rlAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.rl(val);
        bus.write(address, res);
    }

    fn rrAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.rr(val);
        bus.write(address, res);
    }

    fn slaAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.sla(val);
        bus.write(address, res);
    }

    fn sraAddress(&mut self, bus: &mut Bus, address: u16)
    {
        let val = bus.read(address);
        let res = self.sra(val);
        bus.write(address, res);
    }

    // LD BC, n16 | 3  12
    fn ldBc(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetchU16(bus);
        self.registers.setBc(val);

        return 12;
    }

    // LD [BC], A | 1  8
    fn ldBcAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getBc();
        bus.write(address, self.registers.a);

        return 8;
    }
    
    // INC BC | 1  8
    fn incBc(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.registers.getBc();

        self.registers.setBc(val.wrapping_add(1));

        return 8;
    }

    // INC B | 1 4 | Z 0 H -
    fn incB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.b);
        self.registers.b = val;

        return 4;
    }

    // DEC B | 1  4 | Z 1 H -
    fn decB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.b);
        self.registers.b = val;

        return 4;
    }

    // LD B, n8 | 2  8
    fn ldB(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.b = val;

        return 8;
    }

    // RLCA | 1  4 | 0 0 0 C
    fn rlca(&mut self, _bus: &mut Bus) -> u8
    {
        let alu =  self.registers.a;
        let byte = (alu & 0x80) >> 7;

        self.registers.a = (alu << 1) | byte;
        
        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, byte == 1);

        return 4;
    }

    // LD [a16], SP | 3  20 | - - - -
    fn ldAddr16Sp(&mut self, bus: &mut Bus) -> u8
    {
        let low = (self.registers.sp & 0xff) as u8;
        let high = ((self.registers.sp >> 8) & 0xff) as u8;

        let address = self.fetchU16(bus);

        bus.write(address, low);
        bus.write(address.wrapping_add(1), high);

        return 20;
    }

    // ADD HL, BC | 1  8 | - 0 H C
    fn addHlBc(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let bc = self.registers.getBc();
        let val = self.addU16(hl, bc);
        self.registers.setHl(val);

        return 8;
    }

    // LD A, [BC] | 1  8 | - - - -
    fn ldAAddressBc(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getBc();
        let val = bus.read(address);
                
        self.registers.a = val;

        return 8;
    }
    
    // DEC BC | 1  8 | - - - -
    fn decBc(&mut self, _bus: &mut Bus) -> u8
    {
        let decdBc = self.registers.getBc().wrapping_sub(1);
        self.registers.setBc(decdBc);

        return 8;
    }

    // INC C | 1  4 | Z 0 H -
    fn incC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.c);
        self.registers.c = val;

        return 4;
    }

    // DEC C | 1  4 | Z 1 H -
    fn decC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.c);
        self.registers.c = val;

        return 4;
    }

    // LD C, n8 | 2  8 - - - -
    fn ldC(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.c = val;

        return 8;
    }

    // RRCA | 1  4 | 0 0 0 C
    fn rrca(&mut self, _bus: &mut Bus) -> u8
    {
        let alu = self.registers.a;
        let byte = alu & 0x01;

        let rotated = (alu >> 1) | (byte << 7);

        self.registers.a = rotated;

        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, byte == 1);

        return 4;
    }

    // LD DE, n16 | 3  12
    fn ldDe(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetchU16(bus);
        self.registers.setDe(val);

        return 12;
    }

    // LD [DE], A | 1  8
    fn ldDeAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getDe();
        bus.write(address, self.registers.a);

        return 8;
    }

    // INC DE | 1  8
    fn incDe(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.registers.getDe();
        let incdVal = val.wrapping_add(1);
        self.registers.setDe(incdVal);

        return 8;
    }

    // INC D | 1  4 | Z 0 H -
    fn incD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.d);
        self.registers.d = val;

        return 4;
    }

    // DEC D | 1  4 | Z 1 H -
    fn decD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.d);
        self.registers.d = val;

        return 4;
    }

    // LD D, n8 | 2  8 | - - - -
    fn ldD(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.d = val;

        return 8;
    }

    // RLA | 1  4 | 0 0 0 C
    fn rla(&mut self, _bus: &mut Bus) -> u8
    {
        let alu = self.registers.a;
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let newCarry = (alu & 0x80) != 0;

        self.registers.a = (alu << 1) | oldCarry;

        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, newCarry);

        return 4;
    }

    // JR e8 | 2  12 | - - - -
    fn jrE8(&mut self, bus: &mut Bus) -> u8
    {
        let offset = self.fetch(bus) as i8;
        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
 
        return 12;
    }

    // ADD HL, DE | 1  8 | - 0 H C
    fn addHlDe(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let de = self.registers.getDe();
        let val = self.addU16(hl, de);
        self.registers.setHl(val);

        return 8;
    }

    // LD A, [DE] | 1  8 | - - - -
    fn ldAAddressDe(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getDe();
        let val = bus.read(address);
        self.registers.a = val;

        return 8;
    }

    // DEC DE | 1  8 | - - - -
    fn decDe(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.registers.getDe().wrapping_sub(1);
        self.registers.setDe(val);

        return 8;
    }

    // INC E | 1  4 | Z 0 H -
    fn incE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.e);
        self.registers.e = val;

        return 4;
    }

    // DEC E | 1  4 | Z 1 H -
    fn decE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.e);
        self.registers.e = val;

        return 4;
    }

    // LD E, n8 | 2  8 | - - - -
    fn ldE(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.e = val;

        return 8;
    }

    // JR NZ, e8 | 2  12/8 | - - - -
    fn jrNz(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);
        let offset = self.fetch(bus) as i8;
    
        if flag
        {
            return 8;
        }

        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);

        return 12;
    }

    // LD HL, n16 | 3  12 | - - - -
    fn ldHl(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetchU16(bus);
        self.registers.setHl(val);

        return 12;
    }

    // LD [HL+], A | 1  8 | - - - -
    fn ldHlPlusAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();

        bus.write(address, self.registers.a);

        self.registers.setHl(address.wrapping_add(1));

        return 8;
    }

    // INC HL | 1  8 | - - - -
    fn incHl(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();

        self.registers.setHl(hl.wrapping_add(1));

        return 8;
    }

    // INC H | 1  4 | Z 0 H -
    fn incH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.h);
        self.registers.h = val;

        return 4;
    }

    // DEC H | 1  4 | Z 1 H -
    fn decH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.h);
        self.registers.h = val;

        return 4;
    }

    // RRA | 1  4 | 0 0 0 C
    fn rra(&mut self, _bus: &mut Bus) -> u8
    {
        let alu = self.registers.a;
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let newCarry = (alu & 0x01) != 0;

        self.registers.a = (alu >> 7) | (oldCarry << 7);

        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, newCarry);

        return 4;
    }

    // LD H, n8 | 2  8 | - - - -
    fn ldH(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.h = val;

        return 8;
    }

    // DAA | 1  4 | Z - 0 C
    fn daa(&mut self, _bus: &mut Bus) -> u8
    {
        let mut alu = self.registers.a;
        let nFlag = self.registers.getFlag(Registers::MASK_SUBTRACT_N);
        let hFlag = self.registers.getFlag(Registers::MASK_HALF_CARRY_H);
        let mut cFlag = self.registers.getFlag(Registers::MASK_CARRY_C);

        let mut correction = 0;

        if !nFlag
        { 
            if hFlag || (alu & 0x0f) > 0x09
            {
                correction |= 0x06;
            }

            if cFlag || alu > 0x99
            {
                correction |= 0x60;
                cFlag = true;
            }

            alu = alu.wrapping_add(correction);
        }
        else 
        {
            if hFlag
            {
                correction |= 0x06;
            }

            if cFlag
            {
                correction |= 0x60;
            }

            alu = alu.wrapping_sub(correction);
        }

        self.registers.a = alu;
        
        self.registers.setFlag(Registers::MASK_ZERO_Z, alu == 0);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, cFlag);

        return 4;
    }

    // JR Z, e8 | 2  12/8 | - - - -
    fn jrZ(&mut self, bus: &mut Bus) -> u8
    {
        let offset = self.fetch(bus) as i8;
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        if !flag
        {
            return 8;
        }

        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
        
        return 12;
    }

    // ADD HL, HL | 1  8 | - 0 H C
    fn addHlHl(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let val = self.addU16(hl, hl);
        self.registers.setHl(val);

        return 8;
    }

    // LD A, [HL+] | 1  8 | - - - -
    fn ldAAddressHlPlus(&mut self, bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let val = bus.read(hl);

        self.registers.a = val;
        self.registers.setHl(hl.wrapping_add(1));

        return 8;
    }

    // DEC HL | 1  8 | - - - -
    fn decHl(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let val = hl.wrapping_sub(hl);

        self.registers.setHl(val);

        return 8;
    }

    // INC L | 1  4 | Z 0 H -
    fn incL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.l);
        self.registers.l = val;

        return 4;
    }

    // DEC L | 1  4 | Z 1 H -
    fn decL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.l);
        self.registers.l = val;

        return 4;
    }

    // LD L, n8 | 2  8 | - - - -
    fn ldL(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.l = val;

        return 8;
    }

    // CPL | 1  4 | - 1 1 -
    fn cpl(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = !self.registers.a;

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, true);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, true);

        return 4;
    }

    // JR NC, e8 | 2  12/8 | - - - -
    fn jrNc(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);
        let offset = self.fetch(bus) as i8;

        if flag
        {
            return 8;
        }

        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);

        return 12;
    }

    // LD SP, n16 | 3  12 | - - - -
    fn ldSp(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetchU16(bus);
        self.registers.sp = val;

        return 12;
    }

    // LD [HL-], A | 1  8 | - - - -
    fn ldHlMinusAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.a);

        self.registers.setHl(address.wrapping_sub(1));

        return 8;
    }

    // INC SP | 1  8 | - - - -
    fn incSp(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.registers.sp.wrapping_add(1);
        self.registers.sp = val;

        return 8;
    }

    // INC [HL] | 1  12 | Z 0 H -
    fn incAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        let incdVal = self.incU8(val);

        bus.write(address, incdVal);

        return 12;
    }

    // DEC [HL] | 1  12 | Z 1 H -
    fn decAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        let decdVal = self.decU8(val);

        bus.write(address, decdVal);

        return 12;
    }

    // LD [HL], n8 | 2  12 | - - - -
    fn ldAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = self.fetch(bus);

        bus.write(address, val);

        return 12;
    }

    // SCF | 1  4 | - 0 0 1
    fn scf(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, true);

        return 4;
    }

    // JR C, e8 | 2  12/8 | - - - -
    fn jrC(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);
        let offset = self.fetch(bus) as i8;

        if !flag
        {
            return 8;
        }

        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);

        return 12;
    }

    // ADD HL, SP | 1  8 | - 0 H C
    fn addHlSp(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let val = self.addU16(hl, self.registers.sp);
        self.registers.setHl(val);

        return 8;
    }

    // LD A, [HL-] | 1  8 | - - - -
    fn ldAAddressHlMinus(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);

        self.registers.a = val;
        self.registers.setHl(address.wrapping_sub(1));

        return 8;
    }

    // DEC SP | 1  8 | - - - -
    fn decSp(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.sp = self.registers.sp.wrapping_sub(1);

        return 8;
    }

    // INC A | 1  4 | Z 0 H -
    fn incA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.incU8(self.registers.a);
        self.registers.a = val;

        return 4;
    }

    // DEC A | 1  4 | Z 1 H -
    fn decA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.decU8(self.registers.a);
        self.registers.a = val;

        return 4;
    }

    // LD A, n8 | 2  8 | - - - -
    fn ldA(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        self.registers.a = val;

        return 8;
    }

    // CCF | 1  4 | - 0 0 C
    fn ccf(&mut self, _bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, false);
        self.registers.setFlag(Registers::MASK_CARRY_C, !flag);

        return 4;
    }

    // LD B, B | 1  4 | - - - -
    fn ldBB(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD B, C | 1  4 | - - - -
    fn ldBC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.c;

        return 4;
    }

    // LD B, D | 1  4 | - - - -
    fn ldBD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.d;

        return 4;
    }

    // LD B, E | 1  4 | - - - -
    fn ldBE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.e;

        return 4;
    }

    // LD B, H | 1  4 | - - - -
    fn ldBH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.h;

        return 4;
    }

    // LD B, L | 1  4 | - - - -
    fn ldBL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.l;
        
        return 4;
    }

    // LD B, [HL] | 1  8 | - - - -
    fn ldBAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.b = val;

        return 8;
    }

    // LD B, A | 1  4 | - - - -
    fn ldBA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.registers.a;

        return 4;
    }

    // LD C, B | 1  4 | - - - -
    fn ldCB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.b;

        return 4;
    }

    // LD C, C | 1  4 | - - - -
    fn ldCC(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD C, D | 1  4 | - - - -
    fn ldCD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.d;
        
        return 4;
    }

    // LD C, E | 1  4 | - - - -
    fn ldCE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.e;

        return 4;
    }

    // LD C, H | 1  4 | - - - -
    fn ldCH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.h;

        return 4;
    }

    // LD C, L | 1  4 | - - - -
    fn ldCL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.l;

        return 4;
    }
    
    // LD C, [HL] | 1  8 | - - - -
    fn ldCAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.c = val;

        return 8;
    }

    // LD C, A | 1  4 | - - - -
    fn ldCA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.registers.a;

        return 4;
    }

    // LD D, B | 1  4 | - - - -
    fn ldDB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.b;

        return 4;
    }

    // LD D, C | 1  4 | - - - -
    fn ldDC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.c;

        return 4;
    }

    // LD D, D | 1  4 | - - - -
    fn ldDD(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD D, E | 1  4 | - - - -
    fn ldDE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.e;

        return 4;
    }

    // LD D, H | 1  4 | - - - -
    fn ldDH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.h;

        return 4;
    }

    // LD D, L | 1  4 | - - - -
    fn ldDL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.l;

        return 4;
    }

    // LD D, [HL] | 1  8 | - - - -
    fn ldDAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.d = val;

        return 8;
    }

    // LD D, A | 1  4 | - - - -
    fn ldDA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.registers.a;

        return 4;
    }

    // LD E, B | 1  4 | - - - -
    fn ldEB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.b;
        return 4;
    }

    // LD E, C | 1  4 | - - - -
    fn ldEC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.c;
        return 4;
    }

    // LD E, D | 1  4 | - - - -
    fn ldED(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.d;
        return 4;
    }

    // LD E, E | 1  4 | - - - -
    fn ldEE(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD E, H | 1  4 | - - - -
    fn ldEH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.h;
        return 4;
    }

    // LD E, L | 1  4 | - - - -
    fn ldEL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.l;
        return 4;
    }

    // LD E, [HL] | 1  8 | - - - -
    fn ldEAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.e = val;

        return 8;
    }

    // LD E, A | 1  4 | - - - -
    fn ldEA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.registers.a;

        return 4;
    }

    // LD H, B | 1  4 | - - - -
    fn ldHB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.b;

        return 4;
    }

    // LD H, C | 1  4 | - - - -
    fn ldHC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.c;
        
        return 4;
    }

    // LD H, D | 1  4 | - - - -
    fn ldHD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.d;

        return 4;
    }

    // LD H, E | 1  4 | - - - -
    fn ldHE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.e;

        return 4;
    }

    // LD H, H | 1  4 | - - - -
    fn ldHH(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD H, L | 1  4 | - - - -
    fn ldHL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.l;
        
        return 4;
    }

    // LD H, [HL] | 1  8 | - - - -
    fn ldHAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.h = val;

        return 8;
    }

    // LD H, A | 1  4 | - - - -
    fn ldHA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.registers.a;

        return 4;
    }

    // LD L, B | 1  4 | - - - -
    fn ldLB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.b;
        
        return 4;
    }

    // LD L, C | 1  4 | - - - -
    fn ldLC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.c;

        return 4;
    }

    // LD L, D | 1  4 | - - - -
    fn ldLD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.d;

        return 4;
    }

    // LD L, E | 1  4 | - - - -
    fn ldLE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.e;

        return 4;
    }

    // LD L, H | 1  4 | - - - -
    fn ldLH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.h;

        return 4;
    }

    // LD L, L | 1  4 | - - - -
    fn ldLL(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // LD L, [HL] | 1  8 | - - - -
    fn ldLAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.l = val;

        return 8;
    }

    // LD L, A | 1  4 | - - - -
    fn ldLA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.registers.a;
        
        return 4;
    }

    // LD [HL], B | 1  8 | - - - -
    fn ldAddressHlB(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.b);

        return 8;
    }

    // LD [HL], C | 1  8 | - - - -
    fn ldAddressHlC(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.c);

        return 8;
    }

    // LD [HL], D | 1  8 | - - - -
    fn ldAddressHlD(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.d);

        return 8;
    }

    // LD [HL], E | 1  8 | - - - -
    fn ldAddressHlE(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.e);
        
        return 8;
    }

    // LD [HL], H | 1  8 | - - - -
    fn ldAddressHlH(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.h);

        return 8;
    }

    // LD [HL], L | 1  8 | - - - -
    fn ldAddressHlL(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.l);

        return 8;
    }

    // HALT | 1  4 | - - - -
    fn halt(&mut self, _bus: &mut Bus) -> u8
    {
        self.isHalted = true;

        return 4;
    }

    // LD [HL], A | 1  8 | - - - -
    fn ldAddressHlA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        bus.write(address, self.registers.a);

        return 8;
    }

    // LD A, B | 1  4 | - - - -
    fn ldAB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.b;

        return 4;
    }

    // LD A, C | 1  4 | - - - -
    fn ldAC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.c;

        return 4;
    }

    // LD A, D | 1  4 | - - - -
    fn ldAD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.d;

        return 4;
    }

    // LD A, E | 1  4 | - - - -
    fn ldAE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.e;

        return 4;
    }

    // LD A, H | 1  4 | - - - -
    fn ldAH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.h;

        return 4;
    }

    // LD A, L | 1  4 | - - - -
    fn ldAL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.registers.l;

        return 4;
    }

    // LD A, [HL] | 1  8 | - - - -
    fn ldAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        self.registers.a = val;

        return 8;
    }

    // LD A, A | 1  4 | - - - -
    fn ldAA(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    // ADD A, B | 1  4 | Z 0 H C 
    fn addAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.b, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, C | 1  4 | Z 0 H C 
    fn addAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.c, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, D | 1  4 | Z 0 H C 
    fn addAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.d, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, E | 1  4 | Z 0 H C 
    fn addAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.e, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, H | 1  4 | Z 0 H C 
    fn addAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.h, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, L | 1  4 | Z 0 H C 
    fn addAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.l, false);
        self.registers.a = val;

        return 4;
    }

    // ADD A, [HL] | 1  8 | Z 0 H C
    fn addAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.add(self.registers.a, x, false);
        self.registers.a = val;

        return 8;
    }

    // ADD A, A | 1  4 | Z 0 H C 
    fn addAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.a, false);
        self.registers.a = val;

        return 4;
    }
    
    // ADC A, B | 1  4 | Z 0 H C
    fn adcAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.b, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, C | 1  4 | Z 0 H C
    fn adcAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.c, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, D | 1  4 | Z 0 H C
    fn adcAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.d, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, E | 1  4 | Z 0 H C
    fn adcAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.e, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, H | 1  4 | Z 0 H C
    fn adcAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.h, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, L | 1  4 | Z 0 H C
    fn adcAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.l, true);
        self.registers.a = val;

        return 4;
    }

    // ADC A, [HL] | 1  8 | Z 0 H C
    fn adcAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.add(self.registers.a, x, true);
        self.registers.a = val;

        return 8;
    }

    // ADC A, A | 1  4 | Z 0 H C
    fn adcAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.add(self.registers.a, self.registers.a, true);
        self.registers.a = val;

        return 4;
    }
 
    // SUB A, B | 1  4 | Z 1 H C
    fn subAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.b, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, C | 1  4 | Z 1 H C
    fn subAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.c, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, D | 1  4 | Z 1 H C
    fn subAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.d, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, E | 1  4 | Z 1 H C
    fn subAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.e, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, H| 1  4 | Z 1 H C
    fn subAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.h, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, L | 1  4 | Z 1 H C
    fn subAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.l, false);
        self.registers.a = val;

        return 4;
    }

    // SUB A, [HL] | 1  8 | Z 1 H C
    fn subAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.sub(self.registers.a, x, false);
        self.registers.a = val;

        return 8;
    }

    // SUB A, A | 1  4 | 1 1 0 0
    fn subAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.a, false);
        self.registers.a = val;

        return 4;
    }

    // SBC A, B | 1  4 | Z 1 H C
    fn sbcAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.b, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, C | 1  4 | Z 1 H C
    fn sbcAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.c, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, D | 1  4 | Z 1 H C
    fn sbcAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.d, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, E | 1  4 | Z 1 H C
    fn sbcAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.e, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, H| 1  4 | Z 1 H C
    fn sbcAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.h, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, L | 1  4 | Z 1 H C
    fn sbcAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.l, true);
        self.registers.a = val;

        return 4;
    }

    // SBC A, [HL] | 1  8 | Z 1 H C
    fn sbcAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.sub(self.registers.a, x, true);
        self.registers.a = val;

        return 8;
    }

    // SBC A, A | 1  4 | 1 1 0 0
    fn sbcAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.sub(self.registers.a, self.registers.a, true);
        self.registers.a = val;

        return 4;
    }

    // AND A, B | 1  4 | Z 0 1 0
    fn andAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.b);
        self.registers.a = val;

        return 4;
    }

    // AND A, C | 1  4 | Z 0 1 0
    fn andAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.c);
        self.registers.a = val;

        return 4;
    }

    // AND A, D | 1  4 | Z 0 1 0
    fn andAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.d);
        self.registers.a = val;

        return 4;
    }

    // AND A, E | 1  4 | Z 0 1 0
    fn andAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.e);
        self.registers.a = val;

        return 4;
    }

    // AND A, H | 1  4 | Z 0 1 0
    fn andAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.h);
        self.registers.a = val;

        return 4;
    }

    // AND A, L | 1  4 | Z 0 1 0
    fn andAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.l);
        self.registers.a = val;

        return 4;
    }

    // AND A, [HL] | 1  8 | Z 0 1 0
    fn andAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.and(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // AND A, A | 1  4 | Z 0 1 0
    fn andAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.and(self.registers.a, self.registers.a);
        self.registers.a = val;

        return 4;
    }

    // XOR A, B | 1  4 | Z 0 0 0
    fn xorAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.b);
        self.registers.a = val;

        return 4;
    }

    // XOR A, C | 1  4 | Z 0 0 0
    fn xorAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.c);
        self.registers.a = val;

        return 4;
    }

    // XOR A, D | 1  4 | Z 0 0 0
    fn xorAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.d);
        self.registers.a = val;

        return 4;
    }

    // XOR A, E | 1  4 | Z 0 0 0
    fn xorAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.e);
        self.registers.a = val;

        return 4;
    }

    // XOR A, H | 1  4 | Z 0 0 0
    fn xorAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.h);
        self.registers.a = val;

        return 4;
    }

    // XOR A, L | 1  4 | Z 0 0 0
    fn xorAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.l);
        self.registers.a = val;

        return 4;
    }

    // XOR A, [HL] | 1  8 | Z 0 0 0
    fn xorAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.xor(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // XOR A, A | 1  4 | Z 0 0 0
    fn xorAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.a);
        self.registers.a = val;

        return 4;
    }

    // OR A, B | 1  4 | Z 0 0 0
    fn orAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.b);
        self.registers.a = val;

        return 4;
    }

    // OR A, C | 1  4 | Z 0 0 0
    fn orAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.c);
        self.registers.a = val;

        return 4;
    }

    // OR A, D | 1  4 | Z 0 0 0
    fn orAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.d);
        self.registers.a = val;

        return 4;
    }

    // OR A, E | 1  4 | Z 0 0 0
    fn orAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.e);
        self.registers.a = val;

        return 4;
    }

    // OR A, H | 1  4 | Z 0 0 0
    fn orAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.h);
        self.registers.a = val;

        return 4;
    }

    // OR A, L | 1  4 | Z 0 0 0
    fn orAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.l);
        self.registers.a = val;

        return 4;
    }

    // OR A, [HL] | 1  8 | Z 0 0 0
    fn orAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.or(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // OR A, A | 1  4 | Z 0 0 0
    fn orAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.or(self.registers.a, self.registers.a);
        self.registers.a = val;

        return 4;
    }
    
    // CP A, B | 1  4 | Z 1 H C
    fn cpAB(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.b, false);

        return 4;
    }

    // CP A, C | 1  4 | Z 1 H C
    fn cpAC(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.c, false);

        return 4;
    }

    // CP A, D | 1  4 | Z 1 H C
    fn cpAD(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.d, false);

        return 4;
    }

    // CP A, E | 1  4 | Z 1 H C
    fn cpAE(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.e, false);

        return 4;
    }

    // CP A, H | 1  4 | Z 1 H C
    fn cpAH(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.h, false);

        return 4;
    }

    // CP A, L | 1  4 | Z 1 H C
    fn cpAL(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.l, false);

        return 4;
    }

    // CP A, [HL] | 1  8 | Z 1 H C
    fn cpAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        self.sub(self.registers.a, x, false);

        return 8;
    }

    // CP A, A | 1  4 | Z 1 H C
    fn cpAA(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.a, false);

        return 4;
    }

    // RET NZ | 1  20/8 | - - - -
    fn retNz(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        if flag
        {
            return 8;
        }

        self.registers.pc = self.popU16(bus);

        return 20;
    }

    // POP BC | 1  12 | - - - -
    fn popBc(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.popU16(bus);
        self.registers.setBc(val);

        return 12;
    }
 
    // JP NZ, a16 | 3  16/12 | - - - -
    fn jpNz(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        if flag
        {
            return 12;
        }

        let address = self.fetchU16(bus);
        self.registers.pc = address;

        return 16;
    }

    // JP a16 | 3  16 | - - - -
    fn jp(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.fetchU16(bus);
        self.registers.pc = address;

        return 16;
    }

    // CALL NZ, a16 | 3  24/12 | - - - -
    fn callNz(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        let targetAddress = self.fetchU16(bus);

        if flag
        {
            return 12;
        }

        self.callFn(bus, targetAddress);

        return 24;
    }

    // PUSH BC | 1  16 | - - - -
    fn pushBc(&mut self, bus: &mut Bus) -> u8
    {
        self.pushU16(bus, self.registers.getBc());

        return 16;
    }

    // ADD A, n8 | 2  8 | Z 0 H C
    fn addA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.add(self.registers.a, x, false);
        self.registers.a = val;

        return 8;
    }

    // RST $00 | 1  16 | - - - - 
    fn rst00(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x00);

        return 16;
    }
 
    // RET Z | 1 | 20/8 | - - - -
    fn retZ(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        if !flag
        {
            return 8;
        }

        let address = self.popU16(bus);
        self.registers.pc = address;

        return 20;
    }

    // RET | 1  16 | - - - -
    fn ret(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.popU16(bus);
        self.registers.pc = address;

        return 16;
    }

    // JP Z, a16 | 3  16/12 | - - - -
    fn jpZ(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        let address = self.fetchU16(bus);

        if !flag
        {
            return 12;
        }

        self.registers.pc = address;

        return 16;
    }

    // PREFIX | 1  4 | - - - -
    fn prefix(&mut self, bus: &mut Bus) -> u8
    {
        let opcode = self.fetch(bus);
        let clockCycle = self.prefixedInstructions[opcode as usize](self, bus);

        return 4 + clockCycle;
    }

    // CALL Z, a16 | 3  24/12 | - - - -
    fn callZ(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_ZERO_Z);

        let targetAddress = self.fetchU16(bus);

        if !flag
        {
            return 12;
        }

        self.callFn(bus, targetAddress);

        return 24;
    }

    // CALL a16 | 3  24 | - - - -
    fn call(&mut self, bus: &mut Bus) -> u8
    {
        let targetAddress = self.fetchU16(bus);

        self.callFn(bus, targetAddress);

        return 24;
    }

    // ADC A, n8 | 2  8 | Z 0 H C
    fn adcA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.add(self.registers.a, x, true);
        self.registers.a = val;

        return 8;
    }

    // RST $08 | 1  16 | - - - -
    fn rst08(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x08);

        return 16;
    }

    // RET NC | 1  20/8 | - - - -
    fn retNc(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        if flag
        {
            return 8;
        }

       self.registers.pc = self.popU16(bus);

        return 20;
    }

    // POP DE | 1  12 | - - - -
    fn popDe(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.popU16(bus);
        self.registers.setDe(val);
        
        return 12;
    }

    // JP NC, a16 | 3  16/12 | - - - -
    fn jpNc(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        let address = self.fetchU16(bus);

        if flag
        {
            return 12;
        }

        self.registers.pc = address;

        return 16;
    }

    // CALL NC, a16 | 3  24/12 | - - - -
    fn callNc(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        let targetAddress = self.fetchU16(bus);

        if flag
        {
            return 12;
        }

        self.callFn(bus, targetAddress);

        return 24;
    }

    // PUSH DE | 1  16 | - - - -
    fn pushDe(&mut self, bus: &mut Bus) -> u8
    {
        self.pushU16(bus, self.registers.getDe());

        return 16;
    }

    // SUB A, n8 | 2  8 | Z 1 H C
    fn subA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.sub(self.registers.a, x, false);
        self.registers.a = val;

        return 8;
    }

    // RST $10 | 1  16 | - - - -
    fn rst10(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x10);

        return 16;
    }

    // RET C | 1  20/8 | - - - -
    fn retC(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        if !flag
        {
            return 8;
        }

        self.registers.pc = self.popU16(bus);

        return 20;
    }

    // RETI | 1  16 | - - - -
    // TODO: impl interrupts
    fn reti(&mut self, bus: &mut Bus) -> u8
    {
        self.registers.pc = self.popU16(bus);

        return 16;
    }

    // JP C, a16 | 3  16/12 | - - - -
    fn jpC(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        let address = self.fetchU16(bus);

        if !flag
        {
            return 12;
        }

        self.registers.pc = address;

        return 16;
    }

    // CALL C, a16 | 3  24/12 | - - - -
    fn callC(&mut self, bus: &mut Bus) -> u8
    {
        let flag = self.registers.getFlag(Registers::MASK_CARRY_C);

        let targetAddress = self.fetchU16(bus);

        if !flag
        {
            return 12;
        }

        self.callFn(bus, targetAddress);

        return 16;
    }

    // SBC A, n8 | 2  8 | Z 1 H C
    fn sbcA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.sub(self.registers.a, x, true);
        self.registers.a = val;

        return 8;
    }
    
    // RST $18 | 1  16 | - - - -
    fn rst18(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x18);

        return 16;
    }

    // LDH [a8], A | 2  12 | - - - -
    fn ldhAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus);
        let address = 0xff00 | (val as u16);

        bus.write(address, self.registers.a);

        return 12;
    }

    // POP HL | 1  12 | - - - -
    fn popHl(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.popU16(bus);
        self.registers.setHl(val);

        return 12;
    }

    // LDH [C], A | 1  8 | - - - -
    fn ldhAddressCA(&mut self, bus: &mut Bus) -> u8
    {
        let address = 0xff00 | (self.registers.c as u16);
        bus.write(address, self.registers.a);

        return 8;
    }

    // PUSH HL | 1  16 | - - - -
    fn pushHl(&mut self, bus: &mut Bus) -> u8
    {
        self.pushU16(bus, self.registers.getHl());

        return 16;
    }

    // AND A, n8 | 2  8 | Z 0 1 0
    fn andA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.and(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // LD [a16], A | 3  16 | - - - -
    fn ldAddressA(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.fetchU16(bus);
        bus.write(address, self.registers.a);

        return 16;
    }

    // RST $20 | 1  16 | - - - -
    fn rst20(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x20);

        return 16;
    }

    // XOR A, n8 | 2  8 | Z 0 0 0
    fn xorA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.xor(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // ADD SP, e8 | 2  16 | 0 0 H C
    fn addSp(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.fetch(bus) as i8;
        let sp = self.registers.sp;

        self.registers.sp = sp.wrapping_add(val as i16 as u16);

        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        
        let halfCarried = (sp & 0xf) + ((val as u8 as u16) & 0xf) > 0xf;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = (sp & 0xff) + ((val as u8 as u16) & 0xff) > 0xff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

        return 16;
    }

    // JP HL | 1  4 | - - - -
    fn jpHl(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.pc = self.registers.getHl();

        return 4;
    }

    // RST $28 | 1  16 | - - - -
    fn rst28(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x28);

        return 16;
    }

    // LDH A, [a8] | 2  12 | - - - -
    fn ldhAAddress(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let address = 0xff00 | (x as u16);
        let val = bus.read(address);

        self.registers.a = val;

        return 12;
    }

    // POP AF | 1  12 | Z N H C
    fn popAf(&mut self, bus: &mut Bus) -> u8
    {
        let val = self.popU16(bus);
        
        self.registers.setAf(val);

        return 12;
    }

    // LDH A, [C] | 1  8 | - - - -
    fn ldhAAddressC(&mut self, bus: &mut Bus) -> u8
    {
        let address = 0xff00 | (self.registers.c as u16);
        let val = bus.read(address);

        self.registers.a = val;

        return 8;
    }

    // DI | 1  4 | - - - -
    fn di(&mut self, _bus: &mut Bus) -> u8
    {
        self.imeFlag = false;

        return 4;
    }

    // PUSH AF | 1  16 | - - - -
    fn pushAf(&mut self, bus: &mut Bus) -> u8
    {
        self.pushU16(bus, self.registers.getAf());
        
        return 16;
    }

    // OR A, n8 | 2  8 | Z 0 0 0
    fn orA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        let val = self.or(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // RST $30 | 1  16 | - - - -
    fn rst30(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x30);

        return 16;
    }

    // LD HL, SP + e8 | 2  12 | 0 0 H C
    fn ldHlSpPlusByte(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus) as i8;
        let sp = self.registers.sp;
        let val = sp.wrapping_add(x as u16);

        self.registers.setFlag(Registers::MASK_ZERO_Z, false);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = ((sp & 0x0f) + (x as u16 & 0x0f)) > 0x0f;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = ((sp & 0xff) + (x as u16 & 0xff)) > 0xff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

        self.registers.setHl(val);

        return 12;
    }

    // LD SP, HL | 1  8 | - - - -
    fn ldSpHl(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.sp = self.registers.getHl();

        return 8;
    }

    // LD A, [a16] | 3  16 | - - - -
    fn ldAAddress(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.fetchU16(bus);
        let val = bus.read(address);
        self.registers.a = val;

        return 16;
    }

    // EI | 1  4 | - - - -
    fn ei(&mut self, _bus: &mut Bus) -> u8
    {
        self.imeFlag = true;

        return 4;
    }

    // CP A, n8 | 2  8 | Z 1 H C
    fn cpA(&mut self, bus: &mut Bus) -> u8
    {
        let x = self.fetch(bus);
        self.sub(self.registers.a, x, false);

        return 8;
    }

    // RST $38 | 1  16 | - - - -
    fn rst38(&mut self, bus: &mut Bus) -> u8
    {
        self.rst(bus, 0x38);

        return 16;
    }

    // RLC B | 2  8 | Z 0 0 C
    fn rlcB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.rlc(self.registers.b);
        
        return 8;
    }

    // RLC C | 2  8 | Z 0 0 C
    fn rlcC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.rlc(self.registers.c);
        
        return 8;
    }

    // RLC D | 2  8 | Z 0 0 C
    fn rlcD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.rlc(self.registers.d);
        
        return 8;
    }

    // RLC E | 2  8 | Z 0 0 C
    fn rlcE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.rlc(self.registers.e);
        
        return 8;
    }

    // RLC H | 2  8 | Z 0 0 C
    fn rlcH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.rlc(self.registers.h);
        
        return 8;
    }

    // RLC L | 2  8 | Z 0 0 C
    fn rlcL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.rlc(self.registers.l);
        
        return 8;
    }


    // RLC [HL] | 2  8 | Z 0 0 C
    fn rlcAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.rlcAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // RLC A | 2  8 | Z 0 0 C
    fn rlcA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.rlc(self.registers.a);
        
        return 8;
    }

    // RRC B | 2  8 | Z 0 0 C
    fn rrcB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.rrc(self.registers.b);
        
        return 8;
    }

    // RRC C | 2  8 | Z 0 0 C
    fn rrcC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.rrc(self.registers.c);
        
        return 8;
    }

    // RRC D | 2  8 | Z 0 0 C
    fn rrcD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.rrc(self.registers.d);
        
        return 8;
    }

    // RRC E | 2  8 | Z 0 0 C
    fn rrcE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.rrc(self.registers.e);
        
        return 8;
    }

    // RRC H | 2  8 | Z 0 0 C
    fn rrcH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.rrc(self.registers.h);
        
        return 8;
    }

    // RRC L | 2  8 | Z 0 0 C
    fn rrcL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.rrc(self.registers.l);
        
        return 8;
    }

    // RRC [HL] | 2  8 | Z 0 0 C
    fn rrcAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.rrcAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // RRC A | 2  8 | Z 0 0 C
    fn rrcA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.rrc(self.registers.a);
        
        return 8;
    }

    // RL B | 2  8 | Z 0 0 C
    fn rlB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.rl(self.registers.b);

        return 8;
    }

    // RL C | 2  8 | Z 0 0 C
    fn rlC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.rl(self.registers.c);

        return 8;
    }

    // RL D | 2  8 | Z 0 0 C
    fn rlD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.rl(self.registers.d);

        return 8;
    }

    // RL E | 2  8 | Z 0 0 C
    fn rlE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.rl(self.registers.e);

        return 8;
    }

    // RL H | 2  8 | Z 0 0 C
    fn rlH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.rl(self.registers.h);

        return 8;
    }

    // RL L | 2  8 | Z 0 0 C
    fn rlL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.rl(self.registers.l);

        return 8;
    }

    // RL [HL] | 2  16 | Z 0 0 C
    fn rlAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.rlAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // RL A | 2  8 | Z 0 0 C
    fn rlA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.rl(self.registers.a);

        return 8;
    }
    

    // RR B | 2  8 | Z 0 0 C
    fn rrB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.rr(self.registers.b);

        return 8;
    }

    // RR C | 2  8 | Z 0 0 C
    fn rrC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.rr(self.registers.c);

        return 8;
    }

    // RR D | 2  8 | Z 0 0 C
    fn rrD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.rr(self.registers.d);

        return 8;
    }

    // RR E | 2  8 | Z 0 0 C
    fn rrE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.rr(self.registers.e);

        return 8;
    }

    // RR H | 2  8 | Z 0 0 C
    fn rrH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.rr(self.registers.h);

        return 8;
    }

    // RR L | 2  8 | Z 0 0 C
    fn rrL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.rr(self.registers.l);

        return 8;
    }

    // RR [HL] | 2  16 | Z 0 0 C
    fn rrAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.rrAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // RR A | 2  8 | Z 0 0 C
    fn rrA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.rr(self.registers.a);

        return 8;
    }

    // SLA B | 2  8 | Z 0 0 C
    fn slaB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.sla(self.registers.b);

        return 8;
    }

    // SLA C | 2  8 | Z 0 0 C
    fn slaC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.sla(self.registers.c);

        return 8;
    }

    // SLA D | 2  8 | Z 0 0 C
    fn slaD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.sla(self.registers.d);

        return 8;
    }

    // SLA E | 2  8 | Z 0 0 C
    fn slaE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.sla(self.registers.e);

        return 8;
    }

    // SLA H | 2  8 | Z 0 0 C
    fn slaH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.sla(self.registers.h);

        return 8;
    }

    // SLA L | 2  8 | Z 0 0 C
    fn slaL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.sla(self.registers.l);

        return 8;
    }

    // SLA [HL] | 2  16 | Z 0 0 C
    fn slaAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.slaAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // SLA A | 2  8 | Z 0 0 C
    fn slaA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.sla(self.registers.a);

        return 8;
    }
    
    // SRA B | 2  8 | Z 0 0 C
    fn sraB(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.b = self.sra(self.registers.b);

        return 8;
    }

    // SRA C | 2  8 | Z 0 0 C
    fn sraC(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.c = self.sra(self.registers.c);

        return 8;
    }

    // SRA D | 2  8 | Z 0 0 C
    fn sraD(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.d = self.sra(self.registers.d);

        return 8;
    }

    // SRA E | 2  8 | Z 0 0 C
    fn sraE(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.e = self.sra(self.registers.e);

        return 8;
    }

    // SRA H | 2  8 | Z 0 0 C
    fn sraH(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.h = self.sra(self.registers.h);

        return 8;
    }

    // SRA L | 2  8 | Z 0 0 C
    fn sraL(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.l = self.sra(self.registers.l);

        return 8;
    }

    // SRA [HL] | 2  16 | Z 0 0 C
    fn sraAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        self.sraAddress(bus, self.registers.getHl());
        
        return 16;
    }

    // SRA A | 2  8 | Z 0 0 C
    fn sraA(&mut self, _bus: &mut Bus) -> u8
    {
        self.registers.a = self.sra(self.registers.a);

        return 8;
    }


    fn execute(&mut self, opcode: u8, bus: &mut Bus)
    {
        let _clockCycle = self.instructions[opcode as usize](self, bus);
    }

    // NOP | 1  4
    fn nop(&mut self, _bus: &mut Bus) -> u8
    {
        return 4;
    }

    fn notImplemented(&mut self, _bus: &mut Bus) -> u8
    {
        panic!("Not implemented");
    }
}