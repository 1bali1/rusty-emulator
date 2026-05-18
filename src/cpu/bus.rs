use std::{fs::File, io::Read};
use std::io::{self, Write};

use crate::mbc::MBC;
use crate::nombc::NoMBC;
use crate::mbc1::MBC1;
use crate::mbc3::MBC3;
use crate::serial::Serial;
use crate::timer::Timer;
use crate::ppu::PPU;
use crate::joypad::Joypad;

pub struct Bus 
{
    pub mbc: Box<dyn MBC>,
    wram: [u8; 8192],
    hram: [u8; 127],
    timer: Timer,
    serial: Serial,
    pub ppu: PPU,
    pub joypad: Joypad,
    pub ie: u8,
    pub ifl: u8
}

// TODO: remove memory vec
impl Bus 
{
    pub fn new() -> Self
    {
        let timer = Timer::new();
        let serial = Serial::new();
        let ppu = PPU::new();
        let joypad = Joypad::new();

        let bus = Self 
        { 
            mbc: Box::new(NoMBC::new([0].to_vec())),
            wram: [0; 8192],
            hram: [0; 127],
            timer: timer,
            serial: serial,
            ppu: ppu,
            joypad: joypad,
            ie: 0,
            ifl: 0
        };

        return bus;
    }

    pub fn tick(&mut self, cycles: u8)
    {
        // ppu step & interrupt request handling
        self.ppu.step(cycles);

        self.ifl |= self.ppu.registers.interrupt;
        self.ppu.registers.interrupt = 0;

        // timer step & interrupt request handling
        self.timer.tick(cycles);

        self.ifl = self.ifl | self.timer.interrupt;
        self.timer.interrupt = 0;

        // joypad interrupts
        self.ifl |= self.joypad.interrupt;
        self.joypad.interrupt = 0;

        // serial step & interrupt request handling
        self.serial.step(cycles);

        self.ifl |= self.serial.interrupt;
        self.serial.interrupt = 0;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            //0xff4c..=0xff4d => 0xff,
            0x0000..=0x7fff => self.mbc.readRom(address),
            0x8000..=0x9fff => self.ppu.readVram(address),
            0xa000..=0xbfff => self.mbc.readRam(address),
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize],
            0xe000..=0xfdff => self.read(address - 0x2000),
            0xfe00..=0xfe9f => self.ppu.readOam(address),
            0xff80..=0xfffe => self.hram[(address - 0xff80) as usize],
            0xfea0..=0xfeff => 0xff,
            // io ranges
            0xff04..=0xff07 => self.timer.read(address),
            0xff40..=0xff4b | 0xff4f | 0xff51..=0xff55 | 0xff68..=0xff6c => self.ppu.registers.read(address),
            0xff00 => self.joypad.read(),
            0xff01..=0xff02 => self.serial.read(address),
            0xffff => self.ie,
            0xff0f => self.ifl,
            _ => 0xff
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address 
        {
            //0xff4c..=0xff4d => {},
            0x0000..=0x7fff => self.mbc.writeRom(address, value),
            0x8000..=0x9fff => self.ppu.writeVram(address, value),
            0xa000..=0xbfff => self.mbc.writeRam(address, value),
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize] = value,
            0xe000..=0xfdff => self.write(address - 0x2000, value),
            0xfe00..=0xfe9f => self.ppu.writeOam(address, value),
            0xff80..=0xfffe => self.hram[(address - 0xff80) as usize] = value,
            0xfea0..=0xfeff => {},
            // io ranges
            0xff04..=0xff07 => self.timer.write(address, value),
            0xff46 => self.dmaTransfer(value),
            0xff40..=0xff4b | 0xff4f | 0xff51..=0xff55 | 0xff68..=0xff6c => self.ppu.registers.write(address, value),
            0xff00 => self.joypad.write(value),
            0xff01..=0xff02 => self.serial.write(address, value),
            0xffff => self.ie = value,
            0xff0f => self.ifl = value | 0xe0,
            _ => {}
        }

        if address == 0xff01 || address == 0xff02
        {
            // print!("{}", value as char);
            io::stdout().flush().unwrap();
            return;
        }
    }

    fn dmaTransfer(&mut self, value: u8)
    {
        let source = (value as u16) << 8;

        for i in 0..0xa0
        {
            self.ppu.oam[i] = self.read(source + i as u16);
        }
    }

    pub fn loadRom(&mut self, name: &String)
    {
        let dir = String::from("roms/");
        let mut file = File::open(dir + name).expect("ROM load failed");

        let mut buff = Vec::new();
        let _ = file.read_to_end(&mut buff);

        let carType = buff[0x0147];

        let controller: Box<dyn MBC> = match carType
        {
            0x00 => Box::new(NoMBC::new(buff)),
            0x13 => Box::new(MBC3::new(buff)),
            _ => 
            {
                println!("0x{:x}", carType);

                Box::new(MBC1::new(buff))
            }
        };

        self.mbc = controller;

        println!("ROM has loaded successfully!")
    }
}