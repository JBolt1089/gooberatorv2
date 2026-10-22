use crate::{cpu, memory};

pub struct Machine {
    pub memory: memory::Memory,
    pub cpu: cpu::Cpu,
}

impl Machine {
    pub fn new(program_path: &String, defs_path: &String) -> Self {
        Self {
            memory: memory::Memory::new(),
            cpu: cpu::Cpu::new(program_path, defs_path),
        }
    }

    pub fn start(&mut self) {
        while !self.cpu.stopped {
            self.cpu.exec_next(&mut self.memory);
        }
    }
}
