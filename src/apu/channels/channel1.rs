

const DUTY_WAVES: [u8; 4] = 
[
    0x01,
    0x81,
    0x87,
    0x7e
];

pub struct Channel1
{
    enabled: bool,
    duty: u8,
    lengthTimer: u8,
    initialVolume: u8,
    volume: u8,
    envDir: u8,
    sweepPace: u8,
    freq: u16,
    control: u8,
    waveIndex: u8,
    timer: usize,
    envTimer: u8
}

impl Channel1
{
    pub fn new() -> Self
    {
        let channel1 = Self
        {
            enabled: false,
            duty: 0,
            lengthTimer: 0,
            initialVolume: 0,
            volume: 0,
            envDir: 0,
            sweepPace: 0,
            freq: 0,
            control: 0,
            waveIndex: 0,
            timer: 0,
            envTimer: 0
        };

        return channel1;
    }

    pub fn read(&self, address: u16) -> u8
    {
        let val = match address
        {
            0xff10 => 0xff,
            0xff11 => self.duty << 6 | 0x3f,
            0xff12 => (self.initialVolume << 4) | (self.envDir << 3) | self.sweepPace,
            0xff13 => 0xff,
            0xff14 => (self.control & 0x40) | 0xbf,
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
                self.duty = (value >> 6) & 0x3;
                self.lengthTimer = value & 0x3f;
            },
            0xff12 => 
            {
                self.initialVolume = (value >> 4) & 0xf;
                self.envDir = (value >> 3) & 0x01;
                self.sweepPace = value & 0x7;

                if (value & 0xf8) == 0
                {
                    self.enabled = false;
                }
            },
            0xff13 => self.freq = (self.freq & 0x0700) | value as u16,
            0xff14 => 
            {
                self.control = value;
                self.freq = (self.freq & !0x0700) | (((value & 0x07) as u16) << 8);

                if value & 0x80 != 0
                {
                    self.trigger();
                }
            },
            _ => {}
        };
    }

    pub fn trigger(&mut self)
    {
        let period = ((2048 - (self.freq & 0x7ff)) * 4) as usize;

        self.enabled = true;
        self.waveIndex = 0;
        self.timer = if period == 0 { 4 } else { period };
        self.volume = self.initialVolume;
        self.envTimer = if self.sweepPace == 0 { 8 } else { self.sweepPace };
    }

    pub fn tick(&mut self, cycles: u8)
    {
        if !self.enabled { return; }

        if self.timer > cycles as usize
        {
            self.timer -= cycles as usize;
        }
        else 
        {
            let period = ((2048 - (self.freq & 0x7ff)) * 4) as usize;
            self.timer = if period == 0 { 4 } else { period };
            
            self.waveIndex = (self.waveIndex + 1) % 8;
        }
    }

    pub fn tickEnvelope(&mut self)
    {
        if self.sweepPace == 0 { return; }

        if self.envTimer > 0 { self.envTimer -= 1; }

        if self.envTimer <= 0
        {
            self.envTimer = self.sweepPace;

            let mut volume = self.volume as i8;

            if self.envDir == 1 { volume += 1 } else { volume -= 1 };

            if volume >= 0 && volume <= 15
            {
                self.volume = volume as u8;
            }
        }

    }

    pub fn sample(&self) -> f32
    {
        if !self.enabled { return 0.0; }

        let pattern = DUTY_WAVES[self.duty as usize];

        let bit = (pattern >> (7 - self.waveIndex)) & 0x01;
        
        let normalizedVolume = self.volume as f32 / 15.0;

        let op = if bit == 1 { normalizedVolume } else { -normalizedVolume };

        return op.clamp(-1.0, 1.0);
    }
}