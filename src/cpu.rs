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
}