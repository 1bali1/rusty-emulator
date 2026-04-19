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

    let gbName = String::from("cputest.gb");
    bus.loadRom(&gbName);
    

    loop {
        let pc = cpu.registers.pc;
        let opcode = bus.read(pc);
        
        // println!("PC: {:#06X} | Opcode: {:#04X}", pc, opcode);
        
        cpu.step(&mut bus);
    }

    println!("{}, {}, {}, {}", cpu.registers.getFlag(Registers::MASK_ZERO_Z), cpu.registers.getFlag(Registers::MASK_SUBTRACT_N), cpu.registers.getFlag(Registers::MASK_HALF_CARRY_H), cpu.registers.getFlag(Registers::MASK_CARRY_C));
}
