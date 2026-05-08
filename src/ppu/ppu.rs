#[path = "registers.rs"]
mod registers;

use registers::Registers;

enum GameBoyVersion
{
    DMG,
    Colored
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
enum Mode
{
    VBlank = 0b01,
    HBlank = 0b00,
    PixelTransfer = 0b11,
    OAMSearch = 0b10
}

struct Sprite
{
    y: u8,
    x: u8,
    tileId: u8
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
        
        if !isLcdOn == 0x80 
        { 
            self.cycles = 0;
            self.registers.ly = 0;
            self.mode = Mode::VBlank;

            return; 
        }

        self.cycles += cycles as u32;

        self.doMode(self.mode);
    }

    fn doMode(&mut self, mode: Mode)
    {
        self.mode = mode;
        self.registers.stat = (self.registers.stat & 0xfc) | mode as u8;

        match mode {
            Mode::VBlank => self.vblank(),
            Mode::HBlank => self.hblank(),
            Mode::PixelTransfer => self.pixelTransfer(),
            Mode::OAMSearch => self.oamSearch(),
            _ => panic!("Panic")
        }
    }

    fn vblank(&mut self)
    {
        if self.cycles >= 456
        {
            self.cycles -= 456;
            self.registers.incLy();

            // if vblank completed
            if self.registers.ly > 153
            {
                self.registers.ly = 0;
                self.mode = Mode::OAMSearch;
            }
        }
    }

    fn hblank(&mut self)
    {
        if self.cycles >= 204
        {
            self.cycles -= 204;
            self.registers.incLy();

            if self.registers.ly >= 144
            {
               // TODO: request vblank interrupt
               self.mode = Mode::VBlank;
            }
            else 
            {
                self.mode = Mode::OAMSearch;    
            }
        }
    }

    fn pixelTransfer(&mut self)
    {
        if self.cycles >= 172
        {
            self.cycles -= 172;
            self.mode = Mode::HBlank;
            // TODO: render scanline
        }
    }

    fn oamSearch(&mut self)
    {
        if self.cycles >= 80
        {
            self.cycles -= 80;
            self.mode = Mode::PixelTransfer;
        }
    }

    pub fn readOam(&self, address: u16) -> u8
    {
        if self.mode == Mode::PixelTransfer || self.mode == Mode::OAMSearch { return 0xff; }

        let index = address - 0xfe00;

        return self.oam[index as usize];
    }

    pub fn writeOam(&mut self, address: u16, value: u8)
    {
        if self.mode == Mode::PixelTransfer || self.mode == Mode::OAMSearch { return; }

        let index = address - 0xfe00;

        self.oam[index as usize] = value;
    }

    pub fn readVram(&self, address: u16) -> u8
    {
        if self.mode == Mode::PixelTransfer { return 0xff; }

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