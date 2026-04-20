#![allow(non_snake_case)]
#[path = "cpu/bus.rs"]
mod bus;
#[path = "cpu/cpu.rs"]
mod cpu;
#[path = "cpu/registers.rs"]
mod registers;

use bus::Bus;
use cpu::CPU;

use crate::registers::Registers;




fn main() 
{
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    let gbName = String::from("f.gb");
    bus.loadRom(&gbName);

    loop {
        //let pc = cpu.registers.pc;
        //let opcode = bus.read(pc);
        
        //println!("PC: {:#06X} | Opcode: {:#04X}", pc, opcode);
        
        cpu.step(&mut bus);
    }

    //println!("{}, {}, {}, {}", cpu.registers.getFlag(Registers::MASK_ZERO_Z), cpu.registers.getFlag(Registers::MASK_SUBTRACT_N), cpu.registers.getFlag(Registers::MASK_HALF_CARRY_H), cpu.registers.getFlag(Registers::MASK_CARRY_C));
}

#[test]
fn test()
{
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    bus.write(0x0100, 0x31);
    bus.write(0x0101, 0xff);
    bus.write(0x0102, 0x0f);

    bus.write(0x0103, 0x21);
    bus.write(0x0104, 0x01);
    bus.write(0x0105, 0x00);

    bus.write(0x0106, 0x39);

    bus.write(0x0107, 0x18);
    bus.write(0x0108, 0xfe);

    for i in 0..6
    {
        cpu.step(&mut bus);
    }

    println!("{:x}", cpu.registers.sp);

    println!("{}, {}, {}, {}", cpu.registers.getFlag(Registers::MASK_ZERO_Z), cpu.registers.getFlag(Registers::MASK_SUBTRACT_N), cpu.registers.getFlag(Registers::MASK_HALF_CARRY_H), cpu.registers.getFlag(Registers::MASK_CARRY_C));
}