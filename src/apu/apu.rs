#[path = "channels/channel2.rs"]
mod channel2;
#[path = "channels/channel1.rs"]
mod channel1;
#[path = "channels/squarechannel.rs"]
mod squarechannel;

use channel2::Channel2;
use channel1::Channel1;
use ringbuf::{HeapProd, traits::{Observer, Producer}};
pub struct APU
{
    enabled: bool,
    channel1: Channel1,
    channel2: Channel2,
    cycles: usize,
    cyclesRatio: usize,
    producer: HeapProd<f32>,
    frameSeqTimer: usize,
    frameSeqStep: u8,
    nr51: u8,
    nr50: u8
}

impl APU
{
    pub fn new(sampleRate: u32, producer: HeapProd<f32>) -> Self
    {
        let channel1 = Channel1::new();
        let channel2 = Channel2::new();

        let apu = Self
        {
            enabled: false,
            channel1: channel1,
            channel2: channel2,
            cycles: 0,
            cyclesRatio: 4194304 / sampleRate as usize,
            producer: producer,
            frameSeqTimer: 0,
            frameSeqStep: 0,
            nr51: 0,
            nr50: 0
        };

        return apu;
    }

    pub fn tick(&mut self, cycles: u8)
    {
        if !self.enabled
        {
            let _ = self.producer.try_push(0.0);

            return;
        }

        self.channel1.channel.tick(cycles);
        self.channel2.channel.tick(cycles);

        self.frameSeqTimer += cycles as usize;

        if self.frameSeqTimer >= 8192 // ? 512 Hz = 8192 cycle
        {
            self.frameSeqTimer -= 8192;
            self.frameSeqStep = (self.frameSeqStep + 1) % 8;

            match self.frameSeqStep  
            {
                0 | 2 | 4 | 6 => 
                {
                    self.channel1.channel.tickLength();
                    self.channel2.channel.tickLength();
                },
                7 =>
                {
                    self.channel1.channel.tickEnvelope();
                    self.channel2.channel.tickEnvelope();
                },
                _ => { }
            };
        }

        // push sample
        self.cycles += cycles as usize;

        if self.cycles >= self.cyclesRatio
        {
            self.cycles -= self.cyclesRatio;

            let sample1 = self.channel1.channel.sample();
            let sample2 = self.channel2.channel.sample();

            let mut left = 0.0;
            let mut right = 0.0;

            if (self.nr51 & 0x10) != 0 { left += sample1 };
            if (self.nr51 & 0x01) != 0 { right += sample1 };

            if (self.nr51 & 0x20) != 0 { left += sample2 };
            if (self.nr51 & 0x02) != 0 { right += sample2 };

            let leftVol = ((self.nr50 >> 4) & 0x07) as f32 / 7.0;
            let rightVol= (self.nr50 & 0x07) as f32 / 7.0;

            let sample = (left * leftVol + right * rightVol) / 2.0;

            if !self.producer.is_full()
            {
                let _ = self.producer.try_push(sample);
            }
        }
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            0xff10..=0xff14 => self.channel1.read(address),
            0xff15..=0xff19 => self.channel2.read(address),
            0xff24 => self.nr50,
            0xff25 => self.nr51,
            0xff26 => 
            {
                let byte = ((self.enabled as u8) << 7) | ((self.channel1.channel.enabled as u8)) | ((self.channel2.channel.enabled as u8) << 1) | ((false as u8) << 2) | ((false as u8) << 3);

                return byte;
            },
            _ => 0xff
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        if !self.enabled && address != 0xff26 { return; }

        match address
        {
            0xff10..=0xff14 => self.channel1.write(address, value),
            0xff15..=0xff19 => self.channel2.write(address, value),
            0xff24 => self.nr50 = value,
            0xff25 => self.nr51 = value,
            0xff26 =>
            {
                let prevSetting = self.enabled;

                self.enabled = (value >> 7) & 0x01 == 1;

                if prevSetting && !self.enabled
                {
                    self.channel1.channel.clear();
                    self.channel2.channel.clear();

                    self.frameSeqTimer = 0;
                    self.frameSeqStep = 0;

                    self.nr51 = 0;
                    self.nr50 = 0;
                }
            },
            _ => { }
        };
    }
}