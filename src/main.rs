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

    cpu.registers.setFlag(Registers::MASK_ZERO_Z, false);

    bus.write(0x0100, 0x20);
    bus.write(0x0101, 0xfe);
    

    for _ in 0..2 {
        cpu.step(&mut bus);
    }
}
