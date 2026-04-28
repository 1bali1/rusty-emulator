pub struct Registers
{
    lcdc: u8,
    stat: u8,
    scy: u8, // scroll
    scx: u8,
    ly: u8, // lcd y coor
    lyc: u8, // ly comp
    dma: u8,
    bgp: u8, // bg pal data
    obp0: u8, // obj pal
    obp1: u8,
    wy: u8, // win
    wx: u8,

    // colored
    vbank: u8,
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8, // 0-6 len ; 7. mode : 0|gdma 1|hblank dma

    // object color palette specification
    bcps: u8,
    // bcpd: u8,
    ocps: u8,
    // ocpd: u8,
    opri: u8,
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
            // bcpd: 0,
            ocps: 0,
            // ocpd: 0,
            opri: 0
        };
        
        return registers;
    }
}