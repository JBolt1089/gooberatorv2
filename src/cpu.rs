use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::memory::Memory;

macro_rules! check_args {
    ($actual_args:expr, $expected_args:expr, $pc:expr) => {
        if $actual_args != $expected_args {
            eprintln!(
                "wrong number of arguments: expected {}, found {}. pc is {}",
                $expected_args, $actual_args, $pc
            );
            std::process::exit(69);
        }
    };
}

fn read(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(path)?).lines().collect()
}

fn get_defs(lines: Vec<String>, path: String) -> io::Result<Vec<String>> {
    match read(path) {
        Ok(str) => {
            let mut result = lines.clone();
            for replacement in str {
                let rep: Vec<&str> = replacement.split_whitespace().collect();
                result = result.iter().map(|s| s.replace(rep[0], rep[1])).collect();
            }
            Ok(result)
        }
        Err(e) => {
            println!("error occurred opening defs file: {e}");
            std::process::exit(69);
        }
    }
}

fn parse_num_arg(str: &str) -> u16 {
    match parse_int::parse(str) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("a spooky scary error occurred: {e}");
            std::process::exit(69);
        }
    }
}

#[derive(Clone)]
pub struct Cpu {
    pub reg_pc: u16,
    pub reg_a: u16,
    pub reg_b: u16,
    pub reg_c: u16,

    pub program: Vec<String>,
    pub stopped: bool,
}

pub enum Register {
    PC,
    A,
    B,
    C,
}

impl Cpu {
    pub fn new(program_path: &String, definition_path: &String) -> Self {
        let predef_read_program = match read(program_path) {
            Ok(pg) => pg,
            Err(e) => {
                eprintln!("Error occurred opening file: {e}");
                std::process::exit(69);
            }
        };

        let read_program = match get_defs(predef_read_program, definition_path.to_string()) {
            Ok(pg) => pg,
            Err(e) => {
                eprintln!("Error occurred opening file: {e}");
                std::process::exit(69);
            }
        };

        Self {
            reg_pc: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,

            program: read_program,
            stopped: false,
        }
    }

    pub fn get_reg_str(&self, str: &str) -> Register {
        match str.to_lowercase().as_str() {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "pc" => Register::PC,
            _ => {
                eprintln!("unknown register! pc is {}", self.reg_pc);
                std::process::exit(69);
            }
        }
    }

    pub fn reg_from_string(&self, str: &str) -> u16 {
        match self.get_reg_str(str) {
            Register::PC => self.reg_pc,
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
        }
    }

    pub fn set_reg_from_string(&mut self, str: &str, value: u16) {
        match self.get_reg_str(str) {
            Register::PC => self.reg_pc = value,
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
            Register::C => self.reg_c = value,
        }
    }

    pub fn exec_next(&mut self, memory: &mut Memory) {
        let line = self.program[self.reg_pc as usize].clone();
        let args: Vec<&str> = line.split_whitespace().collect();

        let increment = match args[0] {
            "printhi" => {
                self.printhi();
                true
            }
            "li" => {
                self.li(args);
                true
            }
            "prab" => {
                self.prab(args);
                true
            }
            "ld" => {
                self.ld(args, memory);
                true
            }
            "str" => {
                self.str(args, memory);
                true
            }
            "and" => {
                self.and(args);
                true
            }
            "or" => {
                self.or(args);
                true
            }
            "xor" => {
                self.xor(args);
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
                self.add(args);
                true
            }
            "sub" => {
                self.sub(args);
                true
            }
            "call" => {
                self.call(args, memory);
                false
            }
            "jz" => self.jz(args),
            "jnz" => self.jnz(args),
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
            self.reg_pc += 1;
        }
    }
}

impl Cpu {
    pub fn printhi(&self) {
        println!("hi");
    }

    pub fn li(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 3, self.reg_pc);
        self.set_reg_from_string(args[1], parse_num_arg(args[2]));
    }

    pub fn prab(&self, args: Vec<&str>) {
        check_args!(args.len(), 2, self.reg_pc);
        println!("{:#b}", self.reg_from_string(args[1]));
    }

    pub fn end(&mut self) {
        self.stopped = true;
    }

    pub fn ld(&mut self, args: Vec<&str>, memory: &Memory) {
        check_args!(args.len(), 3, self.reg_pc);
        let val = memory.get_ram(parse_num_arg(args[2]));
        self.set_reg_from_string(args[1], val);
    }

    pub fn str(&self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.reg_from_string(args[1]);
        memory.set_ram(parse_num_arg(args[2]), val);
    }

    pub fn and(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.reg_from_string(args[1]) & self.reg_from_string(args[2]);
        self.set_reg_from_string(args[3], result);
    }

    pub fn or(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.reg_from_string(args[1]) | self.reg_from_string(args[2]);
        self.set_reg_from_string(args[3], result);
    }

    pub fn xor(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.reg_from_string(args[1]) ^ self.reg_from_string(args[2]);
        self.set_reg_from_string(args[3], result);
    }

    pub fn push(&self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        memory.push_stack(self.reg_from_string(args[1]));
    }

    pub fn pop(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        self.set_reg_from_string(args[1], memory.pop_stack());
    }

    pub fn gstk(&mut self, args: Vec<&str>, memory: &Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        self.set_reg_from_string(args[1], memory.get_stack_top());
    }

    pub fn add(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.reg_from_string(args[1]) + self.reg_from_string(args[2]);
        self.set_reg_from_string(args[3], result);
    }

    pub fn sub(&mut self, args: Vec<&str>) {
        check_args!(args.len(), 4, self.reg_pc);
        let result = self.reg_from_string(args[1]) - self.reg_from_string(args[2]);
        self.set_reg_from_string(args[3], result);
    }

    pub fn call(&mut self, args: Vec<&str>, memory: &mut Memory) {
        check_args!(args.len(), 2, self.reg_pc);
        let address = parse_num_arg(args[1]);
        memory.push_stack(self.reg_pc);
        self.reg_pc = address;
    }

    pub fn jz(&mut self, args: Vec<&str>) -> bool {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.reg_from_string(args[1]);
        match val {
            0 => {
                self.reg_pc = parse_num_arg(args[2]);
                false
            }
            _ => true,
        }
    }

    pub fn jnz(&mut self, args: Vec<&str>) -> bool {
        check_args!(args.len(), 3, self.reg_pc);
        let val = self.reg_from_string(args[1]);
        match val {
            0 => true,
            _ => {
                self.reg_pc = parse_num_arg(args[2]);
                false
            }
        }
    }

    pub fn ret(&mut self, memory: &mut Memory) {
        self.reg_pc = memory.pop_stack();
    }
}
