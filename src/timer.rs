pub struct Timer 
{
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub shouldInterrupt: bool
}

impl Timer
{
    pub fn new() -> Self
    {
        Self { div: 0, tima: 0, tma: 0, tac: 0, shouldInterrupt: false }
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address {
            0xff04 => (self.div >> 8) as u8,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac | 0xf8,
            _ => panic!("Timer address not found")
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address {
            0xff04 => self.div = 0,
            0xff05 => self.tima = value,
            0xff06 => self.tma = value,
            0xff07 => self.tac = value,
            _ => panic!("Timer address not found (2)")
        }
    }

    pub fn tick(&mut self, cycles: u8)
    {
        let mut tmpDiv = self.div;
        self.div = self.div.wrapping_add(cycles as u16);

        if (self.tac & 0x04) == 0 { return; }

        let speed = self.getClockSpeed();
        let mut ticks = 0;

        for _ in 0..cycles
        {
            tmpDiv = tmpDiv.wrapping_add(1);

            if tmpDiv % speed == 0
            {
                ticks += 1;
            }
        }

        for _ in 0..ticks
        {
            if self.tima == 0xff
            {
                self.tima = self.tma;
                self.shouldInterrupt = true;

                break;
            }
            else 
            {
                self.tima += 1 // should be simple + i think    
            }
        }

    }

    fn getClockSpeed(&self) -> u16
    {
        let val = match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("Clock speed faield")
        };

        return val;
    }
}