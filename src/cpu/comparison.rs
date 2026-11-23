use super::macros::*;
use super::*;

impl Cpu {
    pub fn gthan(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 4, self.reg_pc);

        if self.get_from_arg(args[1], memory) > self.get_from_arg(args[2], memory) {
            let result = 0x1;
            self.set_from_arg(args[3], memory, result);
        } else {
            let result = 0x0;
            self.set_from_arg(args[3], memory, result);
        }
    }

    pub fn lthan(&mut self, args: Vec<&str>, memory: &mut Memory) {
	check_args!(args.len(), 4, self.reg_pc);

	if self.get_from_arg(args[1], memory) < self.get_from_arg(args[2], memory) {
	    let result = 0x1;
	    self.set_from_arg(args[3], memory, result);
	} else {
	    let result = 0x0;
	    self.set_from_arg(args[3], memory, result);
	}
    }

    pub fn greater_equal(&mut self, args: Vec<&str>, memory: &mut Memory) {
	if self.get_from_arg(args[1], memory) >= self.get_from_arg(args[2], memory) {
            let result = 0x1;
            self.set_from_arg(args[3], memory, result);
        } else {
            let result = 0x0;
            self.set_from_arg(args[3], memory, result);
        }
    }

    pub fn less_equal(&mut self, args: Vec<&str>, memory: &mut Memory) {
	if self.get_from_arg(args[1], memory) <= self.get_from_arg(args[2], memory) {
            let result = 0x1;
            self.set_from_arg(args[3], memory, result);
        } else {
            let result = 0x0;
            self.set_from_arg(args[3], memory, result);
        }
    }    
}
