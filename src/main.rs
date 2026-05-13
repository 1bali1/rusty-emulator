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

use cpu::CPU;
use minifb::{Window, WindowOptions};

fn main() 
{
    let winOptions = WindowOptions
    {
        scale: minifb::Scale::X4,
        ..Default::default()
    };
    let mut window = Window::new("Rusty Emulator", 160, 144, winOptions).unwrap();
    window.set_target_fps(60);

    let mut cpu = CPU::new();

    let gbName = String::from("tetris.gb");
    cpu.bus.loadRom(&gbName);
    
    loop
    {
        let cycles = cpu.step();
        cpu.bus.tick(cycles);

        if cpu.bus.ppu.frameReady
        {
            if window.is_open()
            {
                let _ = window.update_with_buffer(&cpu.bus.ppu.pixelBuffer, 160, 144);
                cpu.bus.ppu.frameReady = false;
            }
        }
        
       //println!("Opcode: 0x{:X} | PC: 0x{:X} | B: 0x{:X} | C: 0x{:X} | D: 0x{:X} | E: 0x{:X} | H: 0x{:X} | L: 0x{:X}", bus.read(cpu.registers.pc), cpu.registers.pc, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.f);
    }
}

#[test]
fn test()
{
    
}