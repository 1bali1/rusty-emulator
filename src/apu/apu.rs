#[path = "channels/channel2.rs"]
mod channel2;
#[path = "channels/channel1.rs"]
mod channel1;

use channel2::Channel2;
use channel1::Channel1;
use ringbuf::{HeapProd, traits::{Observer, Producer}};
pub struct APU
{
    channel1: Channel1,
    channel2: Channel2,
    cycles: usize,
    cyclesRatio: usize,
    producer: HeapProd<f32>,
    frameSeqTimer: usize,
    frameSeqStep: u8
}

impl APU
{
    pub fn new(sampleRate: u32, producer: HeapProd<f32>) -> Self
    {
        let channel1 = Channel1::new();
        let channel2 = Channel2::new();

        let apu = Self
        {
            channel1: channel1,
            channel2: channel2,
            cycles: 0,
            cyclesRatio: 4194304 / sampleRate as usize,
            producer: producer,
            frameSeqTimer: 0,
            frameSeqStep: 0
        };

        return apu;
    }

    pub fn tick(&mut self, cycles: u8)
    {
        self.channel1.tick(cycles);
        self.channel2.tick(cycles);

        self.frameSeqTimer += cycles as usize;

        if self.frameSeqTimer >= 8192 // ? 512 Hz = 8192 cycle
        {
            self.frameSeqTimer -= 8192;
            self.frameSeqStep = (self.frameSeqStep + 1) % 8;

            if self.frameSeqStep == 7 // ? 7. bit
            {
                self.channel1.tickEnvelope();
                self.channel2.tickEnvelope();
            }
        }

        // push sample
        self.cycles += cycles as usize;

        if self.cycles >= self.cyclesRatio
        {
            self.cycles -= self.cyclesRatio;

            let sample1 = self.channel1.sample();
            let sample2 = self.channel2.sample();

            let sample = (sample1 + sample2) / 2.0;

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
            _ => 0xff
        };

        return val;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        match address
        {
            0xff10..=0xff14 => self.channel1.write(address, value),
            0xff15..=0xff19 => self.channel2.write(address, value),
            _ => { }
        };
    }
}