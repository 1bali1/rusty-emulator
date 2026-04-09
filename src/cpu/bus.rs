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
        self.memory[address as usize] = value;
    }
}