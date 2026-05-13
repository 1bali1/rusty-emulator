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
    cycles: usize,
    pub pixelBuffer: [u32; 160 * 144],
    version: GameBoyVersion,
    pub frameReady: bool,
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
            frameReady: false,
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
        let ppuOn = self.registers.lcdc >> 7;

        if ppuOn & 0x01 == 0
        {
            self.cycles = 0;
            self.registers.wlc = 0; // ?
            self.registers.setLy(0);
            self.setMode(Mode::HBlank);

            return; 
        }

        self.cycles += cycles as usize;

        self.doMode(self.mode);
    }

    // TODO: pixel-perfect simulation
    fn doMode(&mut self, mode: Mode)
    {
        self.setMode(mode);

        match mode {
            Mode::VBlank => self.vblank(),
            Mode::HBlank => self.hblank(),
            Mode::PixelTransfer => self.pixelTransfer(),
            Mode::OAMSearch => self.oamSearch()
        }
    }

    fn vblank(&mut self)
    {
        if self.cycles >= 456
        {
            self.cycles -= 456;
            self.registers.incLy();

            // if vblank completed
            if self.registers.ly >= 153
            {
                println!("Frame ready");
                
                self.registers.wlc = 0;
                self.registers.setLy(0);

                self.frameReady = true;

                self.setMode(Mode::OAMSearch);
                
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
                
                self.registers.interrupt |= 0x01;
                    self.setMode(Mode::VBlank);
                //println!("VBlank");
            }
            else 
            {
                self.setMode(Mode::OAMSearch);
            }
        }
    }

    fn pixelTransfer(&mut self)
    {
        if self.cycles >= 172
        {
            self.cycles -= 172;
            self.setMode(Mode::HBlank);
            
            self.renderLine();
        }
    }

    fn oamSearch(&mut self)
    {
        if self.cycles >= 80
        {
            self.cycles -= 80;
            self.setMode(Mode::PixelTransfer);
        }
    }

    fn renderLine(&mut self)
    {
        let lcdc = self.registers.lcdc;
        let ly = self.registers.ly;
        let scy = self.registers.scy;
        let scx = self.registers.scx;
        let wy = self.registers.wy;
        let wx = self.registers.wx.wrapping_sub(7);
        let wlc = self.registers.wlc;
        let mut winRendered = false;

        let isBackgroundNWinEnabled = (lcdc & 0x01) == 1;
        if !isBackgroundNWinEnabled {/*  println!("{:b}", lcdc); */ return; }

        for x in 0..160
        {
            let windowEnabled = (lcdc >> 5) & 0x01;
            
            let isWindow = windowEnabled == 1 && ly >= wy && x >= wx; // i think it shouldnt be affected by scroll registers or idk
            
            if isWindow { winRendered = true; }

            let yPos = if isWindow { wlc } else { ly.wrapping_add(scy) };
            let xPos = if isWindow { x as u8 - wx } else { (x as u8).wrapping_add(scx) };
            let mapBaseAddr: u16 = if isWindow { if (lcdc >> 6) & 0x01 == 1 { 0x9c00 } else { 0x9800 } } else { if (lcdc >> 3) & 0x01 == 1 { 0x9c00 } else { 0x9800 } };
            
            // ? pokemon ver lines
            let tileRow = (yPos / 8) as u8;
            let tileCol = (xPos / 8) as u8;
            let tileLine = (yPos % 8) as u8;
            let tilePx = 7 - (xPos % 8) as u8;
            
            // * 32 = full matrix
            // ! not sure if + tileCol as u16 will be ok
            let tileMapIndexAddress = mapBaseAddr + (tileRow as u16 * 32) + tileCol as u16;
            let tileIndex = self.readVram(tileMapIndexAddress);

            let bgWinTiles = (lcdc >> 4) & 0x01 == 1;
            let tileBaseAddr = if bgWinTiles 
            { 
                0x8000 + (tileIndex as u16 * 16) // full size 16
            }
            else 
            {
                let offset = (tileIndex as i8 as i32) * 16;
                (0x9000 as i32 + offset) as u16
            }; 

            let tileLineAddr = tileBaseAddr + (tileLine as u16 * 2);

            let lowLine = self.readVram(tileLineAddr);
            let highLine = self.readVram(tileLineAddr + 1);

            let cbit0 = (lowLine >> tilePx) & 0x01;
            let cbit1 = (highLine >> tilePx) & 0x01;

            let colorId = (cbit1 << 1) | cbit0;
            self.pixelBuffer[(ly as usize * 160 + x as usize) as usize] = self.getDmgColors(colorId);
        }

        if winRendered { self.registers.wlc = self.registers.wlc.wrapping_add(1); }
    }

    fn getDmgColors(&self, colorId: u8) -> u32 {
        match colorId {
            0 => 0xffffff,
            1 => 0xaaaaaa,
            2 => 0x555555,
            3 => 0x000000,
            _ => 0x000000,
        }
    }

    fn setMode(&mut self, mode: Mode)
    {
        let stat = self.registers.stat;
        self.mode = mode;

        self.registers.stat = (stat & 0xfc) | mode as u8;

        if mode == Mode::HBlank && (stat >> 3 & 0x01) == 1 
        {
            self.registers.interrupt |= 0x02
        }

        if mode == Mode::VBlank && (stat >> 4 & 0x01) == 1 
        {
            self.registers.interrupt |= 0x02
        }

        if mode == Mode::OAMSearch && (stat >> 5 & 0x01) == 1
        {
            self.registers.interrupt |= 0x02
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
        // TODO: screen is glitchy if this shit is blocking the vram
        if self.mode == Mode::PixelTransfer { return; }

        let index = address - 0x8000;

        if index > 8192 { return; }

        self.vram[self.registers.vbank as usize][index as usize] = value;
    }
}