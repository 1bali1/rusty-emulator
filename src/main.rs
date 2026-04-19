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

    bus.write(0x0100, 0x97);
    

    for _ in 0..2 {
        cpu.step(&mut bus);
    }

    println!("{}, {}, {}, {}", cpu.registers.getFlag(Registers::MASK_ZERO_Z), cpu.registers.getFlag(Registers::MASK_SUBTRACT_N), cpu.registers.getFlag(Registers::MASK_HALF_CARRY_H), cpu.registers.getFlag(Registers::MASK_CARRY_C));
}
