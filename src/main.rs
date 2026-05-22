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
#[path = "mbc/mbc5.rs"]
mod mbc5;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpu::CPU;
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Split};

use std::{env, fs::{self}};
use minifb::{Window, WindowOptions, Key};

use crate::bus::getEmuMode;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let _ = fs::create_dir_all("roms");

    // window
    let winOptions = WindowOptions
    {
        scale: minifb::Scale::X4,
        ..Default::default()
    };

    let mut window = Window::new("Rusty Emulator", 160, 144, winOptions).unwrap();
    window.set_target_fps(60);

    // audio
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Error");
    let audioConfig = device.default_output_config().unwrap().config(); // this shit needs expect
    let sampleRate = audioConfig.sample_rate;
    let channels = audioConfig.channels as usize;

    // thread safe puffer
    let rb = HeapRb::<f32>::new(4096);
    let (producer, mut consumer) = rb.split();

    let stream = device.build_output_stream(
        &audioConfig, 
        move |data: &mut [f32], _|
        {
            for frame in data.chunks_mut(channels)
            {
                // we shoud mute if theres no sample
                let sample = consumer.try_pop().unwrap_or(0.0);

                for dest in frame
                {
                    *dest = sample;
                }
            }
        },
        |error| panic!("Audio error: {}", error), 
        None
    ).unwrap();

    stream.play().unwrap();

    // cpu steup
    let mut cpu = CPU::new(sampleRate, producer);

    cpu.bus.loadRom(&args[1]);

    if getEmuMode()
    {
        cpu.setGbc();
    }
    
    loop
    {
        let cycles = cpu.step();
        cpu.bus.tick(cycles);

        if cpu.bus.ppu.frameReady
        {
            if window.is_open()
            {
                cpu.bus.joypad.setKey(window.is_key_down(Key::D), 0, false);
                cpu.bus.joypad.setKey(window.is_key_down(Key::A), 1, false);
                cpu.bus.joypad.setKey(window.is_key_down(Key::W), 2, false);
                cpu.bus.joypad.setKey(window.is_key_down(Key::S), 3, false);

                cpu.bus.joypad.setKey(window.is_key_down(Key::R), 0, true);
                cpu.bus.joypad.setKey(window.is_key_down(Key::F), 1, true);
                cpu.bus.joypad.setKey(window.is_key_down(Key::Space), 2, true);
                cpu.bus.joypad.setKey(window.is_key_down(Key::Enter), 3, true);

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