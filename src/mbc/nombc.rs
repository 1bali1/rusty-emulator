use crate::mbc::MBC;

pub struct NoMBC
{
    rom: Vec<u8>
}

impl NoMBC
{
    pub fn new(memory: Vec<u8>, _gameName: &String) -> Self
    {
        let noMbc = Self
        {
            rom: memory
        };

        return noMbc;
    }
}

impl MBC for NoMBC
{
    fn readRom(&self, address: u16) -> u8
    {
        let val = self.rom[address as usize];

        return val;
    }

    fn writeRom(&mut self, _address: u16, _value: u8) { }

    fn readRam(&self, _address: u16) -> u8 
    { 
        return 0xff;
    }

    fn writeRam(&mut self, _address: u16, _value: u8) { }

    fn saveRam(&mut self) { }
    fn loadSave(&mut self) { }
}