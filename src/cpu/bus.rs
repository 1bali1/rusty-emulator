use std::{fs::File, io::Read};
use std::io::{self, Write};
pub struct Bus 
{
    pub memory: [u8; 0x10000]
}

impl Bus 
{
    pub fn new() -> Self
    {
        Self { 
            memory: [0; 0x10000] 
        }
    }

    pub fn read(&self, address: u16) -> u8
    {
        return self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        if address == 0xff01 || address == 0xff02
        {
            print!("{}", value as char);
            io::stdout().flush().unwrap();
            return;
        }

/*         if address < 0x8000 
        {
            return;
        } */

        self.memory[address as usize] = value;
    }

    pub fn loadRom(&mut self, name: &String)
    {
        let dir = String::from("roms/");
        let mut file = File::open(dir + name).expect("ROM load failed");

        let mut buff = Vec::new();
        let _ = file.read_to_end(&mut buff);

        for (i, &byte) in buff.iter().enumerate()
        {
            if i < 0x10000
            {
                self.memory[i] = byte;
            }
        }

        println!("ROM has loaded successfully!")
    }
}