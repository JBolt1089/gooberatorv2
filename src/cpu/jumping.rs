use super::macros::*;
use super::*;

impl Cpu {
    pub fn call(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let address = self.get_from_arg(args[1], memory);
        memory.push_stack(self.reg_pc);
        self.reg_pc = address;
    }

    pub fn rjmp(&mut self, args: Vec<&str>, memory: &Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let result = self.get_from_arg(args[1], memory);
        self.reg_pc = self.reg_pc.wrapping_add(result);
    }

    pub fn jz(&mut self, args: Vec<&str>, memory: &Memory) -> bool {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.get_from_arg(args[1], memory);
        match val {
            0 => {
                self.reg_pc = self.get_from_arg(args[2], memory);
                false
            }
            _ => true,
        }
    }

    pub fn jnz(&mut self, args: Vec<&str>, memory: &Memory) -> bool {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.get_from_arg(args[1], memory);
        match val {
            0 => true,
            _ => {
                self.reg_pc = self.get_from_arg(args[2], memory);
                false
            }
        }
    }

    pub fn ret(&mut self, memory: &mut Memory) {
        self.reg_pc = memory.pop_stack();
    }
}
