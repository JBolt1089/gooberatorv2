use super::macros::*;
use super::*;

impl Cpu {
    pub fn mov(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.get_from_arg(args[1], memory);
        self.set_from_arg(args[2], memory, val);
    }

    pub fn wbtm(&mut self, args: Vec<&str>, memory: &mut Memory) {
        let mut args2 = VecDeque::from(args);
        args2.pop_front();
        let mut address = self.get_from_arg(args2.pop_front().unwrap(), memory);
        while !args2.is_empty() {
            let val = self.get_from_arg(args2.pop_front().unwrap(), memory);
            memory.set_ram(address, val);
            address += 1;
        }
    }

    pub fn push(&self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        memory.push_stack(self.get_from_arg(args[1], memory));
    }

    pub fn pop(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let value = memory.pop_stack();
        self.set_from_arg(args[1], memory, value);
    }

    pub fn gstk(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let value = memory.get_stack_top();
        self.set_from_arg(args[1], memory, value);
    }
}
