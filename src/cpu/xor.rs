    // XOR A, B | 1  4 | Z 0 0 0
    fn xorAB(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.b);
        self.registers.a = val;

        return 4;
    }

    // XOR A, C | 1  4 | Z 0 0 0
    fn xorAC(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.c);
        self.registers.a = val;

        return 4;
    }

    // XOR A, D | 1  4 | Z 0 0 0
    fn xorAD(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.d);
        self.registers.a = val;

        return 4;
    }

    // XOR A, E | 1  4 | Z 0 0 0
    fn xorAE(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.e);
        self.registers.a = val;

        return 4;
    }

    // XOR A, H | 1  4 | Z 0 0 0
    fn xorAH(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.h);
        self.registers.a = val;

        return 4;
    }

    // XOR A, L | 1  4 | Z 0 0 0
    fn xorAL(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.l);
        self.registers.a = val;

        return 4;
    }

    // XOR A, [HL] | 1  8 | Z 0 0 0
    fn xorAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        let val = self.xor(self.registers.a, x);
        self.registers.a = val;

        return 8;
    }

    // XOR A, A | 1  4 | Z 0 0 0
    fn xorAA(&mut self, _bus: &mut Bus) -> u8
    {
        let val = self.xor(self.registers.a, self.registers.a);
        self.registers.a = val;

        return 4;
    }