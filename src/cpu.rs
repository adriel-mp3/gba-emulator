pub struct Flags {
    pub negative: bool,
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
}

pub struct Cpu {
    pub registers: [u32; 16],
    pub flags: Flags,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0u32; 16],
            flags: Flags {
                negative: false,
                zero: false,
                carry: false,
                overflow: false,
            },
        }
    }

    pub fn sp(&self) -> u32 {
        self.registers[13]
    }

    pub fn set_sp(&mut self, value: u32) {
        self.registers[13] = value;
    }

    pub fn lr(&self) -> u32 {
        self.registers[14]
    }

    pub fn set_lr(&mut self, value: u32) {
        self.registers[14] = value;
    }

    pub fn pc(&self) -> u32 {
        self.registers[15]
    }

    pub fn set_pc(&mut self, value: u32) {
        self.registers[15] = value;
    }

    pub fn mov(&mut self, destination: usize, value: u32) {
        self.registers[destination] = value;
        self.update_nz(value);
    }

    pub fn add(&mut self, destination: usize, left: usize, right: usize) {
        let a = self.registers[left];
        let b = self.registers[right];
        let (result, carry) = a.overflowing_add(b);

        self.registers[destination] = result;
        self.flags.carry = carry;
        self.flags.overflow = ((a ^ result) & (b ^ result) & 0x8000_0000) != 0;
        self.update_nz(result);
    }

    pub fn sub(&mut self, destination: usize, left: usize, right: usize) {
        let a = self.registers[left];
        let b = self.registers[right];
        let (result, borrow) = a.overflowing_sub(b);

        self.registers[destination] = result;
        self.flags.carry = !borrow;
        self.flags.overflow = ((a ^ b) & (a ^ result) & 0x8000_0000) != 0;
        self.update_nz(result);
    }

    fn update_nz(&mut self, value: u32) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x8000_0000) != 0;
    }
}
