use std::{fs::File, io::Read};
use std::io::{self, Write};

use crate::timer::Timer;
use crate::ppu::PPU;
pub struct Bus 
{
    pub memory: [u8; 0x10000],
    timer: Timer,
    pub ppu: PPU,
    pub ie: u8,
    pub ifl: u8
}

// TODO: remove memory vec
impl Bus 
{
    pub fn new() -> Self
    {
        let timer = Timer::new();
        let ppu = PPU::new();

        let bus = Self 
        { 
            memory: [0; 0x10000], 
            timer: timer,
            ppu: ppu,
            ie: 0,
            ifl: 0
        };

        return bus;
    }

    pub fn tick(&mut self, cycles: u8)
    {
        // ppu step & interrupt request handling
        self.ppu.step(cycles);

        self.ifl = self.ifl | self.ppu.registers.interrupt;
        self.ppu.registers.interrupt = 0;

        // timer step & interrupt request handling
        self.timer.tick(cycles);

        self.ifl = self.ifl | self.timer.interrupt;
        self.timer.interrupt = 0;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            0xfe00..0xfe9f => self.ppu.readOam(address),
            0x8000..0x9fff => self.ppu.readVram(address),
            0xff04..0xff07 => self.timer.read(address),
            0xff40..0xff55 | 0xff68..0xff6c => self.ppu.registers.read(address),
            0xffff => self.ie,
            0xff0f => self.ifl,
            _ => self.memory[address as usize]
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address 
        {
            0xfe00..0xfe9f => self.ppu.writeOam(address, value),
            0x8000..0x9fff => self.ppu.writeVram(address, value),
            0xff04..0xff07 => self.timer.write(address, value),
            0xff40..0xff55 | 0xff68..0xff6c => self.ppu.registers.write(address, value),
            0xffff => self.ie = value,
            0xff0f => self.ifl = value | 0xe0,
            _ => self.memory[address as usize] = value
        }

        if address == 0xff01 || address == 0xff02
        {
            print!("{}", value as char);
            io::stdout().flush().unwrap();
            return;
        }

        /* if address < 0x8000 
        {
            return;
        } */

    }

    pub fn loadRom(&mut self, name: &String)
    {
        let dir = String::from("roms/");
        let mut file = File::open(dir + name).expect("ROM load failed");

        let mut buff = Vec::new();
        let _ = file.read_to_end(&mut buff);

        for (i, &byte) in buff.iter().enumerate()
        {
            if i < 0x10000
            {
                self.memory[i] = byte;
            }
        }

        println!("ROM has loaded successfully!")
    }
}