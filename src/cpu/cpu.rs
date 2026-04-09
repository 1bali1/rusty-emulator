use std::panic;

use crate::bus::Bus;
use crate::registers::Registers;

pub struct CPU
{
    pub registers: Registers
}

impl CPU
{
    pub fn new() -> Self
    {
        Self {
            registers: Registers::new()
        }
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

        let overflow = (value & 0x0F) == 0; // 0000 
        self.registers.setFlag(Registers::MASK_HALF_CARRY_H, overflow);

        return decdVal;
    }

    fn execute(&mut self, opcode: u8, bus: &mut Bus)
    {
        match opcode
        {
            0x00 => { }, // NOP | 1  4
            0x01 => { // LD BC, n16 | 3  12
                let val = self.fetchU16(bus);
                self.registers.setBc(val);
            },
            0x02 => { // LD [BC], A | 1  8
                let address = self.registers.getBc();
                bus.write(address, self.registers.a);
            },
            0x03 => { // INC BC | 1  8
                let val = self.registers.getBc();
                let incdVal = val.wrapping_add(1);
                self.registers.setBc(incdVal);
            },
            0x04 => { // INC B | 1 4 | Z 0 H -
                let val = self.incU8(self.registers.b);
                self.registers.b = val;
            },
            0x05 => { // DEC B | 1  4 | Z 1 H -
                let val = self.decU8(self.registers.b);
                self.registers.b = val;
            },
            0x06 => { // LD B, n8 | 2  8
                let val = self.fetch(bus);
                self.registers.b = val;
            },

            _ => {
                panic!("Opcode error");
            }
        }
    }
}