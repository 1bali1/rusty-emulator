use core::panic;

pub struct Registers
{
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8, // scroll
    pub scx: u8,
    pub ly: u8, // lcd y coor
    pub lyc: u8, // ly comp
    pub dma: u8,
    pub bgp: u8, // bg pal data
    pub obp0: u8, // obj pal
    pub obp1: u8,
    pub wy: u8, // win
    pub wx: u8,
    pub yCon: bool,
    pub wlc: u8,

    // colored
    pub vbank: u8,
    pub hdma: HDMA,

    // object color palette specification
    pub bcps: u8,
    pub bcpd: u8,
    bpi: u8,

    pub ocps: u8,
    pub ocpd: u8,
    opi: u8,
    pub opri: u8,

    // intr
    pub interrupt: u8,

    // cram
    pub bgPaletteRam: [u8; 64],
    pub objPaletteRam: [u8; 64],
    pub shouldTransfer: bool
}

pub struct HDMA
{
    pub active: bool,
    pub mode: u8,
    pub length: u8,
    pub source: u16,
    pub dest: u16
}

impl Registers {
    pub fn new() -> Self
    {
        let registers = Self 
        { 
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xfc,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            yCon: false,
            wlc: 0,
            vbank: 0,
            hdma: HDMA { active: false, mode: 0, length: 0, source: 0, dest: 0 },
            bcps: 0,
            bcpd: 0,
            bpi: 0,
            ocps: 0,
            ocpd: 0,
            opi: 0,
            opri: 0,
            interrupt: 0,
            bgPaletteRam: [0; 64],
            objPaletteRam: [0; 64],
            shouldTransfer: false
        };
        
        return registers;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address {
            0xff40 => self.lcdc,
            0xff41 => self.stat,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => self.dma,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            0xff4f => self.vbank,
            0xff51 => ((self.hdma.source >> 8) & 0x1f) as u8,
            0xff52 => self.hdma.source as u8,
            0xff53 => ((self.hdma.dest >> 8) & 0x1f) as u8,
            0xff54 => self.hdma.dest as u8,
            0xff55 =>
            { 
                if self.hdma.active
                {
                    self.hdma.length
                }
                else 
                {
                    self.hdma.length | 0x80
                }
            },
            0xff68 => self.bcps | self.bpi,
            0xff69 => self.bgPaletteRam[self.bpi as usize],
            0xff6a => self.ocps | self.opi,
            0xff6b => self.objPaletteRam[self.opi as usize],
            0xff6c => self.opri,
          _ => panic!("PPU Reg addr not found (2) {:X}", address)
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address {
            0xff40 => self.lcdc = value,
            0xff41 => self.stat = value,
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => self.ly = value,
            0xff45 => self.lyc = value,
            0xff46 => self.dma = value,
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            0xff4f => self.vbank = value & 0x01,
            0xff51 => self.hdma.source = (self.hdma.source & !0xff00) | (value as u16) << 8,
            0xff52 => self.hdma.source = (self.hdma.source & 0xff00) | value as u16,
            0xff53 => self.hdma.dest = (self.hdma.dest & !0xff00) | (value as u16) << 8,
            0xff54 => self.hdma.dest = (self.hdma.dest & 0xff00) | value as u16,
            0xff55 =>
            {
                let mode = (value >> 7) & 0x01;
                let blocks = value & 0x7f;

                if self.hdma.active && mode == 1
                {
                    if (value & 0x80) == 0 { self.hdma.active = false }
                }
                else 
                {
                    self.hdma.mode = mode;
                    self.hdma.length = blocks;

                    if mode == 0
                    {
                        self.shouldTransfer = true;
                        self.hdma.active = false;
                    }
                    else
                    {
                        self.hdma.active = true;
                    }
                }
            },
            0xff68 => 
            {
                self.bcps = value & 0x80;
                self.bpi = value & 0x3f;
            },
            0xff69 =>
            {
                self.bgPaletteRam[self.bpi as usize] = value;

                if (self.bcps >> 7) & 0x01 == 1 
                { 
                    self.bpi += 1;
                    
                    if self.bpi >= 64 { self.bpi = 0 }
                }
            },
            0xff6a =>
            {
                self.ocps = value & 0x80;
                self.opi = value & 0x3f;
            },
            0xff6b =>
            {
                self.objPaletteRam[self.opi as usize] = value;

                if (self.ocps >> 7) & 0x01 == 1 
                { 
                    self.opi += 1;
                    
                    if self.opi >= 64 { self.opi = 0 }
                }
            },
            0xff6c => self.opri = value,
          _ => panic!("PPU Reg addr not found (2) {:X}", address) // TODO: ff50
        };
    }

    pub fn setLy(&mut self, value: u8)
    {
        self.ly = value;

        if self.ly == self.lyc
        {
            self.stat |= 0x04;

            if (self.stat >> 6) & 0x01 == 1 
            {
                self.interrupt |= 0x02;
            }
        }
        else
        {
            self.stat &= !0x04;
        }
    }

    pub fn incLy(&mut self)
    {
        let next = if self.ly >= 153 {
            0
        } else {
            self.ly + 1
        };

        self.setLy(next);
    }
}