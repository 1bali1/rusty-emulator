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
        
        cpu.instructions[0x26] = CPU::ldH;


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

        let overflow = (value & 0x0f) + 1 > 0x0f;
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
        let incdVal = val.wrapping_add(1);
        self.registers.setBc(incdVal);

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
        let acc =  self.registers.a;
        let byte = (acc & 0x80) >> 7;

        self.registers.a = (acc << 1) | byte;
        
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

        let lowAddr = self.fetch(bus) as u16;
        let highAddr = self.fetch(bus) as u16;

        let address = (highAddr << 8) | lowAddr;

        bus.write(address, low);
        bus.write(address.wrapping_add(1 ), high);

        return 20;
    }

    // ADD HL, BC | 1  8 | - 0 H C
    fn addHlBc(&mut self, _bus: &mut Bus) -> u8
    {
        let hl = self.registers.getHl();
        let bc = self.registers.getBc();
        let val = hl.wrapping_add(bc);

        self.registers.setFlag(Registers::MASK_SUBTRACT_N, false);

        let halfCarried = (hl & 0x0fff) + (bc & 0x0fff) > 0x0fff;
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
        let acc = self.registers.a;
        let byte = acc & 0x01;

        let rotated = (acc >> 1) | (byte << 7);

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
        let acc = self.registers.a;
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let newCarry = (acc & 0x80) != 0;

        self.registers.a = (acc << 1) | oldCarry;

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
        let val = self.registers.pc as i32;
        
        self.registers.pc = (val + (offset as i32)) as u16;
 
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

    // RRA | 1  4 | 0 0 0 C
    fn rra(&mut self, _bus: &mut Bus) -> u8
    {
        let acc = self.registers.a;
        let oldCarry = self.registers.getFlag(Registers::MASK_CARRY_C) as u8;
        let newCarry = (acc & 0x01) != 0;

        self.registers.a = (acc >> 7) | (oldCarry << 7);

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