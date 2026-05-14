pub struct Joypad
{
    joyp: u8,
    buttonsMatrix: u8,
    directionsMatrix: u8,
    pub interrupt: u8
}

impl Joypad
{
    pub fn new() -> Self
    {
        let joypad = Self 
        { 
            joyp: 0xcf,
            buttonsMatrix: 0x0f,
            directionsMatrix: 0x0f,
            interrupt: 0 
        };

        return joypad;
    }

    pub fn read(&self) -> u8
    {
        let buttonsBit = (self.joyp >> 5) & 0x01;

        if buttonsBit == 0
        {
            return (self.joyp | 0xcf) & self.buttonsMatrix;
        }
        else 
        {
            return (self.joyp | 0xcf) & self.directionsMatrix;
        }
    }

    pub fn write(&mut self, value: u8)
    {
        self.joyp = (value & 0x30) | self.joyp & 0xcf;
    }

    pub fn setKey(&mut self, isKeyDown: bool, key: u8, isButtons: bool)
    {
        if isKeyDown
        {
            if isButtons
            { self.buttonsMatrix &= !(1 << key); }
            else 
            { self.directionsMatrix &= !(1 << key); }

            self.interrupt |= 0x10;
        }
        else 
        {
            if isButtons
            { self.buttonsMatrix |= 1 << key; }
            else 
            { self.directionsMatrix |= 1 << key; }
        }
    }

}