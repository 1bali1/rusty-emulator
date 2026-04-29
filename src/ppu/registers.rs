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

    // colored
    pub vbank: u8,
    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
    pub hdma5: u8, // 0-6 len ; 7. mode : 0|gdma 1|hblank dma

    // object color palette specification
    pub bcps: u8,
    pub bcpd: u8,
    pub ocps: u8,
    pub ocpd: u8,
    pub opri: u8,
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
            vbank: 0,
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
            bcps: 0,
            bcpd: 0,
            ocps: 0,
            ocpd: 0,
            opri: 0
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
            0xff51 => self.hdma1,
            0xff52 => self.hdma2,
            0xff53 => self.hdma3,
            0xff54 => self.hdma4,
            0xff55 => self.hdma5,
            0xff68 => self.bcps,
            0xff69 => self.bcpd,
            0xff6a => self.ocps,
            0xff6b => self.ocpd,
            0xff6c => self.opri,
          _ => panic!("PPU Reg addr not found")  
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
            0xff4f => self.vbank = value,
            0xff51 => self.hdma1 = value,
            0xff52 => self.hdma2 = value,
            0xff53 => self.hdma3 = value,
            0xff54 => self.hdma4 = value,
            0xff55 => self.hdma5 = value,
            0xff68 => self.bcps = value,
            0xff69 => self.bcpd = value,
            0xff6a => self.ocps = value,
            0xff6b => self.ocpd = value,
            0xff6c => self.opri = value,
          _ => panic!("PPU Reg addr not found (2)")  
        };
    }


}