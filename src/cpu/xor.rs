    // CP A, B | 1  4 | Z 1 H C
    fn cpAB(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.b, true);

        return 4;
    }

    // CP A, C | 1  4 | Z 1 H C
    fn cpAC(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.c, true);

        return 4;
    }

    // CP A, D | 1  4 | Z 1 H C
    fn cpAD(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.d, true);
        return 4;
    }

    // CP A, E | 1  4 | Z 1 H C
    fn cpAE(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.e, true);

        return 4;
    }

    // CP A, H | 1  4 | Z 1 H C
    fn cpAH(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.h, true);

        return 4;
    }

    // CP A, L | 1  4 | Z 1 H C
    fn cpAL(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.l, true);

        return 4;
    }

    // CP A, [HL] | 1  8 | Z 1 H C
    fn cpAAddressHl(&mut self, bus: &mut Bus) -> u8
    {
        let address = self.registers.getHl();
        let x = bus.read(address);
        self.sub(self.registers.a, x);

        return 8;
    }

    // CP A, A | 1  4 | Z 1 H C
    fn cpAA(&mut self, _bus: &mut Bus) -> u8
    {
        self.sub(self.registers.a, self.registers.a, true);

        return 4;
    }