#![allow(non_snake_case)]
#[path = "cpu/bus.rs"]
mod bus;
#[path = "cpu/cpu.rs"]
mod cpu;
#[path = "cpu/registers.rs"]
mod registers;

use bus::Bus;
use cpu::CPU;

fn main() 
{
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    for _ in 0..20 {
        cpu.step(&mut bus);
    }
}
