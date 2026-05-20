use crate::apu::squarechannel::SquareChannel;

pub struct Channel1
{
    pub channel: SquareChannel
}

impl Channel1
{
    pub fn new() -> Self
    {
        let channel = SquareChannel::new();

        let channel1 = Self
        {
            channel: channel
        };

        return channel1;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            0xff10 => 0xff,
            0xff11 => self.channel.duty << 6 | 0x3f,
            0xff12 => (self.channel.initialVolume << 4) | (self.channel.envDir << 3) | self.channel.sweepPace,
            0xff13 => 0xff,
            0xff14 => self.channel.control & 0x40,
            _ => 0xff
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address
        {
            0xff10 => {},
            0xff11 => 
            {
                self.channel.duty = (value >> 6) & 0x3;
                self.channel.lengthTimer = value & 0x3f;
            },
            0xff12 => 
            {
                self.channel.initialVolume = (value >> 4) & 0xf;
                self.channel.envDir = (value >> 3) & 0x01;
                self.channel.sweepPace = value & 0x7;

                if (value & 0xf8) == 0
                {
                    self.channel.enabled = false;
                }
            },
            0xff13 => self.channel.freq = (self.channel.freq & 0x0700) | value as u16,
            0xff14 => 
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