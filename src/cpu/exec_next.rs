use super::*;

impl Cpu {
    pub fn exec_next(&mut self, memory: &mut Memory) {
        let line = self.program[self.reg_pc as usize].clone();
        let args: Vec<&str> = line.split_whitespace().collect();

        let increment = match args[0] {
            "printhi" => {
                self.printhi();
                true
            }
            "prnb" => {
                self.prnb(args, memory);
                true
            }
            "prnc" => {
                self.prnc(args, memory);
                true
            }
            "mov" => {
                self.mov(args, memory);
                true
            }
            "wbtm" => {
                self.wbtm(args, memory);
                true
            }
            "and" => {
                self.and(args, memory);
                true
            }
            "or" => {
                self.or(args, memory);
                true
            }
            "xor" => {
                self.xor(args, memory);
                true
            }
            "push" => {
                self.push(args, memory);
                true
            }
            "pop" => {
                self.pop(args, memory);
                true
            }
            "gstk" => {
                self.gstk(args, memory);
                true
            }
            "add" => {
                self.add(args, memory);
                true
            }
            "sub" => {
                self.sub(args, memory);
                true
            }
            "call" => {
                self.call(args, memory);
                false
            }
            "rjmp" => {
                self.rjmp(args, memory);
                false
            }
            "jz" => self.jz(args, memory),
            "jnz" => self.jnz(args, memory),
            "ret" => {
                self.ret(memory);
                true
            }
            "end" => {
                self.end();
                false
            }
            _ => {
                eprintln!("unknown instruction");
                std::process::exit(69);
            }
        };

        if increment {
            self.reg_pc = self.reg_pc.wrapping_add(1);
        }
    }
}
