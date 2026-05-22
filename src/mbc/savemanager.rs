use std::{fs::{self, File}, time::{Duration, Instant}, io::{Read, Write}};

pub struct SaveManager
{
    gameName: String,
    lastSave: Instant
}

impl SaveManager
{
    pub fn new(gameName: String) -> Self
    {
        let saveManager = Self
        {
            gameName: gameName,
            lastSave: Instant::now()
        };

        return saveManager;
    }

    pub fn saveRam(&mut self, ram: &Vec<u8>) 
    {
        if self.lastSave.elapsed() < Duration::from_mins(1) 
        {
            return;
        }

        let _ = fs::create_dir_all("saves");
        let filePath = format!("saves/{}.sav", self.gameName);

        if let Ok(mut file) = File::create(filePath)
        {
            let _ = file.write_all(ram);
            self.lastSave = Instant::now();
            
            println!("Saved!");
        }
        else
        {
            println!("Failed to save game: {}", self.gameName);
        }
    }

    pub fn loadSave(&mut self, ram: &mut Vec<u8>)
    {
        let _ = fs::create_dir_all("saves");
        let filePath = format!("saves/{}.sav", self.gameName);

        if let Ok(mut file) = File::open(filePath)
        {
            ram.clear();
            let _ = file.read_to_end(ram);

            println!("Save loaded!");
        }
    }
}