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
    pub const MASK_ZERO_Z: u8 = 0x80; // 1000 0000
    pub const MASK_SUBTRACT_N: u8 = 0x40; // 0100 0000
    pub const MASK_HALF_CARRY_H: u8 = 0x20; // 0010 0000
    pub const MASK_CARRY_C: u8 = 0x10; // 0001 0000


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

    pub fn getFlag(&self, mask: u8) -> bool 
    {
        let val = (self.f & mask) != 0;

        return val;
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
        self.c = (value & 0xff) as u8;
    }

    pub fn getDe(&self) -> u16
    {
        return ((self.d as u16) << 8) | (self.e as u16);    
    }

    pub fn setDe(&mut self, value: u16)
    {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn getHl(&self) -> u16
    {
        return ((self.h as u16) << 8) | (self.l as u16);
    }

    pub fn setHl(&mut self, value: u16)
    {
        // 0000 0000 0000 0000
        // 0000 0000 1111 1111
        self.h = (value >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }

    pub fn getAf(&self) -> u16
    {
        return ((self.a as u16) << 8) | (self.f as u16);
    }

    pub fn setAf(&mut self, value: u16)
    {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xf0) as u8;
    }
}