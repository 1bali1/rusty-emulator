use crate::apu::squarechannel::SquareChannel;

pub struct Channel2
{
    pub channel: SquareChannel
}

impl Channel2
{
    pub fn new() -> Self
    {
        let channel = SquareChannel::new();

        let channel2 = Self
        {
            channel: channel
        };

        return channel2;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            0xff15 => 0xff,
            0xff16 => self.channel.duty << 6 | 0x3f,
            0xff17 => (self.channel.initialVolume << 4) | (self.channel.envDir << 3) | self.channel.sweepPace,
            0xff18 => 0xff,
            0xff19 => self.channel.control & 0x40,
            _ => 0xff
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address
        {
            0xff15 => {},
            0xff16 => 
            {
                self.channel.duty = (value >> 6) & 0x3;
                self.channel.lengthTimer = value & 0x3f;
            },
            0xff17 => 
            {
                self.channel.initialVolume = (value >> 4) & 0xf;
                self.channel.envDir = (value >> 3) & 0x01;
                self.channel.sweepPace = value & 0x7;

                if (value & 0xf8) == 0
                {
                    self.channel.enabled = false;
                }
            },
            0xff18 => self.channel.freq = (self.channel.freq & 0x0700) | value as u16,
            0xff19 => 
            {
                self.channel.control = value;
                self.channel.freq = (self.channel.freq & !0x0700) | (((value & 0x07) as u16) << 8);

                if value & 0x80 != 0
                {
                    self.channel.trigger();
                }
            },
            _ => {}
        };
    }
}