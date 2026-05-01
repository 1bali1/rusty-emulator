#[path = "registers.rs"]
mod registers;

use registers::Registers;

use crate::bus::Bus;

enum GameBoyVersion
{
    DMG,
    Colored
}

#[derive(PartialEq)]
enum Mode
{
    VBlank,
    HBlank,
    PixelTransfer,
    OAMSearch
}

pub struct PPU
{
    cycles: u32,
    pixelBuffer: [u32; 160 * 144],
    version: GameBoyVersion,
    pub registers: Registers,
    vram: [[u8; 8192]; 2],
    bgPaletteRam: [u8; 64],
    objPaletteRam: [u8; 64],
    oam: [u8; 160],
    mode: Mode
}

// i = (y*160) + x

impl PPU {
    pub fn new() -> Self
    {
        let ppu = Self 
        { 
            cycles: 0,
            pixelBuffer: [0; 160 * 144],
            version: GameBoyVersion::DMG,
            registers: Registers::new(),
            vram: [[0; 8192]; 2],
            bgPaletteRam: [0; 64],
            objPaletteRam: [0; 64],
            oam: [0; 160],
            mode: Mode::OAMSearch
        };

        return ppu;
    }

    pub fn step(&mut self, cycles: u8)
    {
        let isLcdOn = self.registers.lcdc & 0x80;
        
        if !isLcdOn == 0x80 { return; }

        self.cycles += cycles as u32;

        if self.cycles >= 456
        {
            self.registers.incLy();
        }
    }

    pub fn readVram(&self, address: u16) -> u8
    {
        if self.mode == Mode::PixelTransfer { return 0xff }

        let index = address - 0x8000;

        if index > 8192 { return 0xff; }

        let val = self.vram[self.registers.vbank as usize][index as usize];

        return val;
    }

    pub fn writeVram(&mut self, address: u16, value: u8)
    {
        if self.mode == Mode::PixelTransfer { return; }

        let index = address - 0x8000;

        if index > 8192 { return; }

        self.vram[self.registers.vbank as usize][index as usize] = value;
    }
}