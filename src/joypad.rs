pub struct Joypad
{
    joyp: u8,
    pub interrupt: u8
}

impl Joypad
{
    pub fn new() -> Self
    {
        let joypad = Self 
        { 
            joyp: 0, 
            interrupt: 0 
        };

        return joypad;
    }

    pub fn read(&self) -> u8
    {
        return self.joyp;
    }

    pub fn write(&mut self, value: u8)
    {
        self.joyp = value;
    }
}