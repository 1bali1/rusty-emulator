use crate::{mbc::MBC, savemanager::SaveManager};

pub struct MBC5
{
    rom: Vec<u8>,
    ram: Vec<u8>,
    romBank: u16,
    ramBank: u8,
    ramEnabled: bool,
    hasBattery: bool,
    saveManager: SaveManager
}

const BATTERY_TYPES: [u8; 3]  = [0x13, 0x1b, 0x1e];


impl MBC5
{
    pub fn new(memory: Vec<u8>, gameName: &String) -> Self
    {
        let mbcType = memory[0x0147];
        let hasBattery = BATTERY_TYPES.contains(&mbcType);
        let saveManager = SaveManager::new(gameName.to_owned());

        let mbc5 = Self
        {
            rom: memory,
            ram: vec![0; 0x20000],
            romBank: 1,
            ramBank: 0,
            ramEnabled: false,
            hasBattery: hasBattery,
            saveManager: saveManager
        };

        return mbc5;
    }

    fn getRamAddress(&self, address: u16) -> usize
    {
        let offset = (address - 0xa000) as usize;
        let ramAddres = (((self.ramBank & 0xf) as usize * 0x2000) as usize + offset) % self.ram.len();

        return ramAddres;
    }
}

impl MBC for MBC5
{
    fn readRom(&self, address: u16) -> u8 
    {
        if address < 0x4000 
        {
            return self.rom[address as usize];
        }

        let offset = (address - 0x4000) as usize;
        let romAddress = ((self.romBank as usize * 0x4000) as usize + offset) % self.rom.len();

        let val = self.rom[romAddress];

        return val;
    }

    fn writeRom(&mut self, address: u16, value: u8) 
    {
        match address 
        {
            0x0000..=0x1fff => if (value & 0xf) == 0xa { self.ramEnabled = true } else 
                {
                    let prevMode = self.ramEnabled;

                    if prevMode
                    {
                        self.saveRam();
                    }

                    self.ramEnabled = false; 
                },
            0x2000..=0x2fff => self.romBank = (self.romBank & 0x100) | value as u16,
            0x3000..=0x3fff => self.romBank = (self.romBank & 0xff) | ((value & 0x01) as u16) << 8,
            0x4000..=0x5fff => self.ramBank = value,
            0x6000..=0x7fff => {},
            _ => {}
        }    
    }

    fn readRam(&self, address: u16) -> u8 
    {
        if !self.ramEnabled { return 0xff; }

        let address = self.getRamAddress(address);

        let val = self.ram[address];
      
        return val;
    }

    fn writeRam(&mut self, address: u16, value: u8) 
    {
        if !self.ramEnabled { return; }

        let address = self.getRamAddress(address);

        self.ram[address] = value;

        self.saveRam();
    }

    fn saveRam(&mut self) 
    {
        if !self.hasBattery { return; }

        self.saveManager.saveRam(&self.ram);
    }

    fn loadSave(&mut self)
    {
        if !self.hasBattery { return; }

        self.saveManager.loadSave(&mut self.ram);
    }
}