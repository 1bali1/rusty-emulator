

const DUTY_WAVES: [u8; 4] = 
[
    0x01,
    0x81,
    0x87,
    0x7e
];

pub struct SquareChannel
{
    pub enabled: bool,
    pub duty: u8,
    pub lengthTimer: u8,
    pub initialVolume: u8,
    pub volume: u8,
    pub envDir: u8,
    pub sweepPace: u8,
    pub freq: u16,
    pub control: u8,
    pub waveIndex: u8,
    pub timer: usize,
    pub envTimer: u8
}

impl SquareChannel
{
    pub fn new() -> Self
    {
        let squarechannel = Self
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

        return squarechannel;
    }

    pub fn trigger(&mut self)
    {
        if self.initialVolume == 0 && self.envDir == 0 { return; }

        let period = ((2048 - (self.freq & 0x7ff)) * 4) as usize;

        self.enabled = true;
        self.waveIndex = 0;
        self.timer = if period == 0 { 4 } else { period };
        self.volume = self.initialVolume;
        self.envTimer = if self.sweepPace == 0 { 8 } else { self.sweepPace };

        if self.lengthTimer == 0
        {
            self.lengthTimer = 64;
        }
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

    pub fn tickLength(&mut self)
    {
        if (self.control & 0x40) == 0 { return; }

        if self.lengthTimer > 0
        {
            self.lengthTimer -= 1;

            if self.lengthTimer <= 0
            {
                self.enabled =false;
            }
        }
    }

    pub fn tickEnvelope(&mut self)
    {
        if self.sweepPace == 0 { return; }

        if self.envTimer > 0 { self.envTimer -= 1; }

        if self.envTimer <= 0
        {
            self.envTimer = if self.sweepPace == 0 { 8 } else { self.sweepPace };

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

    pub fn clear(&mut self)
    {
        self.enabled = false;
        self.duty = 0;
        self.lengthTimer = 0;
        self.initialVolume = 0;
        self.volume = 0;
        self.envDir = 0;
        self.sweepPace = 0;
        self.freq = 0;
        self.control = 0;
        self.timer = 0;
        self.envTimer = 0;
        self.waveIndex = 0;
    }
}