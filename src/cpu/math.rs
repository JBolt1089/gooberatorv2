use super::macros::*;
use super::*;

impl Cpu {
    pub fn add(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self
            .get_from_arg(args[1], memory)
            .wrapping_add(self.get_from_arg(args[2], memory));
        self.set_from_arg(args[3], memory, result);
    }

    pub fn sub(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self
            .get_from_arg(args[1], memory)
            .wrapping_sub(self.get_from_arg(args[2], memory));
        self.set_from_arg(args[3], memory, result);
    }
}
