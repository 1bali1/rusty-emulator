use std::{fs::{self, File}, io::{Read, Write}};

use crate::{mbc::MBC};

// TODO: RTC
pub struct MBC3
{
    rom: Vec<u8>,
    ram: Vec<u8>,
    romBank: u8,
    ramBank: u8,
    ramAndRTCEnabled: bool,
    hasBattery: bool,
    gameName: String
}

const BATTERY_TYPES: [u8; 3]  = [0x0f, 0x10, 0x13];


impl MBC3
{
    pub fn new(memory: Vec<u8>, gameName: &String) -> Self
    {
        let mbcType = memory[0x0147];
        let hasBattery = BATTERY_TYPES.contains(&mbcType);

        let mbc3 = Self
        {
            rom: memory,
            ram: vec![0; 0x4000],
            romBank: 1,
            ramBank: 0,
            ramAndRTCEnabled: false,
            hasBattery: hasBattery,
            gameName: gameName.to_owned()
        };

        return mbc3;
    }

    fn getRamAddress(&self, address: u16) -> usize
    {
        let offset = (address - 0xa000) as usize;
        let ramAddres = ((self.ramBank as usize * 0x2000) as usize + offset) % self.ram.len();

        return ramAddres;
    }
}

impl MBC for MBC3
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
            0x0000..=0x1fff => if (value & 0xf) == 0xa { self.ramAndRTCEnabled = true } else 
                {
                    let prevMode = self.ramAndRTCEnabled;

                    if prevMode
                    {
                        self.saveRam();
                    }

                    self.ramAndRTCEnabled = false; 
                },
            0x2000..=0x3fff => if (value & 0x7f) == 0 { self.romBank = 1 } else { self.romBank = value },
            0x4000..=0x5fff => self.ramBank = value & 0x3,
            0x6000..=0x7fff => {},
            _ => {}
        }    
    }

    fn readRam(&self, address: u16) -> u8 
    {
        if !self.ramAndRTCEnabled { return 0xff; }

        let val = match self.ramBank
        {
            0x00..=0x03 => 
            {
                let address = self.getRamAddress(address);

                return self.ram[address];
            },
            _ => 0xff
        };

        return val;
    }

    fn writeRam(&mut self, address: u16, value: u8) 
    {
        if !self.ramAndRTCEnabled { return; }

        let address = self.getRamAddress(address);

        match self.ramBank
        {
            0x00..=0x03 => self.ram[address] = value,
            _ => {}
        };
    }

    fn saveRam(&self) 
    {
        if !self.hasBattery { return; }

        let _ = fs::create_dir_all("saves");
        let filePath = format!("saves/{}.sav", self.gameName);

        if let Ok(mut file) = File::create(filePath)
        {
            let _ = file.write_all(&self.ram);
            
            println!("Saved!");
        }
    }

    fn loadSave(&mut self)
    {
        if !self.hasBattery { return; }

        let _ = fs::create_dir_all("saves");
        let filePath = format!("saves/{}.sav", self.gameName);

        if let Ok(mut file) = File::open(filePath)
        {
            self.ram.clear();
            let _ = file.read_to_end(&mut self.ram);

            println!("Save loaded!");
        }
    }
}