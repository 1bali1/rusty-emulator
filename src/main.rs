#![allow(non_snake_case)]
#[path = "cpu/bus.rs"]
mod bus;
#[path = "cpu/cpu.rs"]
mod cpu;
#[path = "cpu/registers.rs"]
mod registers;
#[path = "ppu/ppu.rs"]
mod ppu;
#[path ="apu/apu.rs"]
mod apu;
mod timer;
mod joypad;
mod serial;
#[path = "mbc/mbc.rs"]
mod mbc;
#[path = "mbc/nombc.rs"]
mod nombc;
#[path = "mbc/mbc1.rs"]
mod mbc1;
#[path = "mbc/mbc3.rs"]
mod mbc3;

use cpu::CPU;

use std::{env, fs::{self}};
use minifb::{Window, WindowOptions, Key};

fn main() 
{
    let args: Vec<String> = env::args().collect();

    let _ = fs::create_dir_all("roms");

    let winOptions = WindowOptions
    {
        scale: minifb::Scale::X4,
        ..Default::default()
    };

    let mut window = Window::new("Rusty Emulator", 160, 144, winOptions).unwrap();
    window.set_target_fps(60);

    let mut cpu = CPU::new();
    let gbName = format!("{}.gb", args[1]);

    cpu.bus.loadRom(&gbName);
    
    loop
    {
        let cycles = cpu.step();
        cpu.bus.tick(cycles);
        
        cpu.bus.joypad.setKey(window.is_key_down(Key::D), 0, false);
        cpu.bus.joypad.setKey(window.is_key_down(Key::A), 1, false);
        cpu.bus.joypad.setKey(window.is_key_down(Key::W), 2, false);
        cpu.bus.joypad.setKey(window.is_key_down(Key::S), 3, false);

        cpu.bus.joypad.setKey(window.is_key_down(Key::R), 0, true);
        cpu.bus.joypad.setKey(window.is_key_down(Key::F), 1, true);
        cpu.bus.joypad.setKey(window.is_key_down(Key::Space), 2, true);
        cpu.bus.joypad.setKey(window.is_key_down(Key::Enter), 3, true);

        if cpu.bus.ppu.frameReady
        {
            if window.is_open()
            {
                let _ = window.update_with_buffer(&cpu.bus.ppu.pixelBuffer, 160, 144);
                cpu.bus.ppu.frameReady = false;
            }
        }
    }
}

#[test]
fn test()
{
    
}