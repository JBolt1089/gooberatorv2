use super::macros::*;
use super::*;

impl Cpu {
    pub fn printhi(&self) {
        println!("hi");
    }

    pub fn prnb(&self, args: Vec<&str>, memory: &Memory) {
        macros::check_args!(args.len(), 2, self.reg_pc);
        println!("{:#b}", self.get_from_arg(args[1], memory));
    }

    pub fn prnc(&self, args: Vec<&str>, memory: &Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let c = self.get_from_arg(args[1], memory);
        print!("{}", c as u8 as char);
    }
}
