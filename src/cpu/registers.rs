pub struct Registers 
{
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16, 
    pub sp: u16,

}

impl Registers
{
    pub const MASK_ZERO_Z: u8 = 0x80;
    pub const MASK_SUBTRACT_N: u8 = 0x40;
    pub const MASK_HALF_CARRY_H: u8 = 0x20;
    pub const MASK_CARRY_C: u8 = 0x10;


    pub fn new() -> Self
    {
        Self {
            a: 0x01, f: 0xb0, // nem fix h ez az
            b: 0x00, c: 0x13,
            d: 0x00, e: 0xd8,
            h: 0x01, l: 0x4d,
            pc: 0x0100,
            sp: 0xfffe
        }
    }

    pub fn setFlag(&mut self, mask: u8, value: bool)
    {
        if value
        {
            self.f |= mask;
        }
        else 
        {
            self.f &= !mask;   
        }
    }

    pub fn getBc(&self) -> u16
    {
        return ((self.b as u16) << 8) | (self.c as u16);
    }

    pub fn setBc(&mut self, value: u16)
    {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}