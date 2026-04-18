use crate::bus::Bus;
use crate::registers::Registers;

type InstructionFn = fn(&mut CPU, &mut Bus) -> u8;

pub struct CPU
{
    pub registers: Registers,
    instructions: [InstructionFn; 256]
}

impl CPU
{
    pub fn new() -> Self
    {
        let mut cpu = CPU {
            registers: Registers::new(),
            instructions: [CPU::nop; 256]
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

        return cpu;

    }

    pub fn step(&mut self, bus: &mut Bus)
    {
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

        let overflow = (value & 0x0f) == 0; // 0000 
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, overflow);

        return decdVal;
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
        let val = hl.wrapping_add(bc);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = (hl & 0xfff) + (bc & 0xfff) > 0xfff;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = (hl as u32) + (bc as u32) > 0xffff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

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
        let val = hl.wrapping_add(de);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = (hl & 0x0fff) + (de & 0x0fff) > 0x0fff;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = (hl as u32) + (de as u32) > 0xffff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

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
            if hFlag || (alu & 0xf) > 0x9
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
        let sum = (hl as u32) + (hl as u32);
        let val = sum as u16;

        self.registers.setHl(val);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);
        
        let halfCarried = (hl & 0xfff) + (hl & 0xfff) > 0xfff;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        let carried = sum > 0xffff;
        self.registers.setFlag(Registers::MASK_CARRY_C, carried);

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

    // NC SP | 1  8 | - - - -
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
        let incdVal = val.wrapping_add(1);

        bus.write(address, incdVal);

        self.registers.setFlag(Registers::MASK_ZERO_Z, incdVal == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = ((val & 0xf).wrapping_add(1)) & 0x10 != 0;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

        return 12;
    }

    // DEC [HL] | 1  12 | Z 1 H -
    fn decAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let val = bus.read(address);
        let decdVal = val.wrapping_sub(1);

        bus.write(address, decdVal);

        self.registers.setFlag(Registers::MASK_ZERO_Z, decdVal == 0);
        self.registers.setFlag(Registers::MASK_SUBTRACT_N, true);

        let halfCarried = (val & 0x0f) == 0;
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, halfCarried);

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

    fn execute(&mut self, opcode: u8, bus: &mut Bus)
    {
        let _clockCycle = self.instructions[opcode as usize](self, bus);
    }
    
    // NOP | 1  4
    pub fn nop(&mut self, _bus: &mut Bus) -> u8
    {
        return 1;
    }
}