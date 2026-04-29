#![allow(non_snake_case)]
#[path = "cpu/bus.rs"]
mod bus;
#[path = "cpu/cpu.rs"]
mod cpu;
#[path = "cpu/registers.rs"]
mod registers;
#[path = "ppu/ppu.rs"]
mod ppu;

mod timer;

use bus::Bus;
use cpu::CPU;

use crate::ppu::PPU;


fn main() 
{
    let mut ppu = PPU::new();
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    let gbName = String::from("c.gb");
    bus.loadRom(&gbName);
    
    loop {
        let cycles = cpu.step(&mut bus);
        ppu.step(cycles);
        
       //println!("Opcode: 0x{:X} | PC: 0x{:X} | B: 0x{:X} | C: 0x{:X} | D: 0x{:X} | E: 0x{:X} | H: 0x{:X} | L: 0x{:X}", bus.read(cpu.registers.pc), cpu.registers.pc, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.f);
    }
}

#[test]
fn test()
{
    
}