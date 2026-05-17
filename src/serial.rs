use core::panic;

pub struct Serial
{
    data: u8,
    control: u8,
    cycles: usize,
    pub interrupt: u8
}

impl Serial
{
    pub fn new() -> Self
    {
        let serial = Self
        {
            data: 0,
            control: 0x7e,
            cycles: 0,
            interrupt: 0
        };
        
        return serial;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address 
        {
            0xff01 => self.data,
            0xff02 => self.control | 0x7e,
            _ => panic!("Serial reg not found")
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address 
        {
            0xff01 => self.data = value,
            0xff02 => {
                self.control = value;

                if (self.control & 0x81) == 0x81
                {
                    self.cycles = 0;
                }
            },
            _ => panic!("Serial reg not found")
        };
    }


    pub fn step(&mut self, cycles: u8)
    {
        let started = (self.control >> 7) & 0x01 == 1;
        let master = self.control & 0x01 == 1;

        if !started || !master
        {
            self.cycles = 0; 
            return; 
        }

        self.cycles += cycles as usize;

        if self.cycles >= 4096 
        { 
            self.cycles -= 4096;

            self.control &= !0x80;
            self.data = 0xff;
            self.interrupt |= 0x08;

            return; 
        }
    }
}