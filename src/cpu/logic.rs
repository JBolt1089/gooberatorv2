use super::macros::*;
use super::*;

impl Cpu {
    pub fn and(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.get_from_arg(args[1], memory) & self.get_from_arg(args[2], memory);
        self.set_from_arg(args[3], memory, result);
    }

    pub fn or(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.get_from_arg(args[1], memory) | self.get_from_arg(args[2], memory);
        self.set_from_arg(args[3], memory, result);
    }

    pub fn xor(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.get_from_arg(args[1], memory) ^ self.get_from_arg(args[2], memory);
        self.set_from_arg(args[3], memory, result);
    }
}
