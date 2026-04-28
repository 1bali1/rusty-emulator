#[path = "registers.rs"]
mod registers;

use registers::Registers;

enum GameBoyVersion
{
    DMG,
    Colored
}

pub struct PPU
{
    cycles: u32,
    pixelBuffer: [u32; 166 * 144],
    version: GameBoyVersion,
    registers: Registers,
    vram: [[u8; 8192]; 2],
    bgPaletteRam: [u8; 64],
    objPaletteRam: [u8; 64],
    oam: [u8; 160]
}

// i = (y*160) + x

impl PPU {
    pub fn new() -> Self
    {
        let ppu = Self 
        { 
            cycles: 0,
            pixelBuffer: [0; 166 * 144],
            version: GameBoyVersion::DMG,
            registers: Registers::new(),
            vram: [[0; 8192]; 2],
            bgPaletteRam: [0; 64],
            objPaletteRam: [0; 64],
            oam: [0; 160]
        };

        return ppu;
    }
}