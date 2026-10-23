const U16_POSSIBLE_VALUES: u32 = 65536;

pub struct Memory {
    pub ram: [u16; U16_POSSIBLE_VALUES as usize],
    pub stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; U16_POSSIBLE_VALUES as usize],
            stack: Vec::new(),
        }
    }

    pub fn get_ram(&self, address: u16) -> u16 {
        self.ram[address as usize]
    }

    pub fn set_ram(&mut self, address: u16, value: u16) {
        self.ram[address as usize] = value;
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        match self.stack.pop() {
            Some(val) => val,
            None => {
                eprintln!("stack is empty and you tried to get something from it you idiot");
                std::process::exit(69);
            }
        }
    }

    pub fn get_stack_top(&self) -> u16 {
        match self.stack.last() {
            Some(val) => val.to_owned(),
            None => {
                eprintln!("stack is empty and you tried to get something from it you idiot");
                std::process::exit(69420);
            }
        }
    }
}
