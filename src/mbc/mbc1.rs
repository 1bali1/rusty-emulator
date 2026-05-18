use crate::mbc::{BankingMode, MBC};

pub struct MBC1
{
    rom: Vec<u8>,
    ram: Vec<u8>,
    ramEnabled: bool,
    romBank: u8,
    ramBank: u8,
    bankMode: BankingMode
}

impl MBC1
{
    pub fn new(memory: Vec<u8>) -> Self
    {
        let mbc1 = Self
        {
            rom: memory,
            ram: vec![0; 0x8000],
            ramEnabled: false,
            romBank: 1,
            ramBank: 0,
            bankMode: BankingMode::Simple
        };

        return mbc1;
    }

    fn getRamAddress(&self, address: u16) -> usize
    {
        let mut ramBank = self.ramBank;

        if self.bankMode == BankingMode::Simple
        {
            ramBank = 0;
        }

        let offset = (address - 0xa000) as usize;
        let ramAddress = ((ramBank as usize * 0x2000) + offset) % self.ram.len();

        return ramAddress;
    }
}

impl MBC for MBC1
{
    fn readRom(&self, address: u16) -> u8
    {
        let mut romBank = self.romBank & 0x1f; // just for safety ykyk

        if address < 0x4000
        {
            if self.bankMode == BankingMode::Advanced
            {
                romBank = self.ramBank << 5;
            }
            else 
            {
                romBank = 0;    
            }
        }
        else 
        {
            if self.bankMode == BankingMode::Advanced
            {
                romBank = (self.ramBank << 5) | romBank;
            }    
        }

        let offset = if address < 0x4000 { address as usize} else { (address - 0x4000) as usize };
        let romAddress = ((romBank as usize * 0x4000) + offset) % self.rom.len();

        let val = self.rom[romAddress];

        return val;
    }

    fn writeRom(&mut self, address: u16, value: u8) 
    {
        match address {
            0x6000..=0x7fff => if (value & 0x01) == 1 { self.bankMode = BankingMode::Advanced } else { self.bankMode = BankingMode::Simple },
            0x4000..=0x5fff => self.ramBank = value & 0x03,
            0x2000..=0x3fff => self.romBank = if (value & 0x1f) == 0 { 1 } else { value & 0x1f },
            0x0000..=0x1fff => if (value & 0xf) == 0xa { self.ramEnabled = true; } else { self.ramEnabled = false; }
            _ => {}
        }
    }

    fn readRam(&self, address: u16) -> u8 
    { 
        if !self.ramEnabled { return 0xff; }

        let ramAddress = self.getRamAddress(address);

        let val = self.ram[ramAddress];

        return val;
    }

    fn writeRam(&mut self, address: u16, value: u8) 
    {
        if !self.ramEnabled { return; }

        let ramAddress = self.getRamAddress(address);

        self.ram[ramAddress] = value;
    }
}