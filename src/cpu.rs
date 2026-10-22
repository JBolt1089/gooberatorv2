use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::memory::Memory;

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
        return match str.to_lowercase().as_str() {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "pc" => Register::PC,
            _ => {
                eprintln!("unknown register! pc is {}", self.reg_pc);
                std::process::exit(69);
            }
        };
    }

    pub fn reg_from_string(&self, str: &str) -> u16 {
        return match self.get_reg_str(str) {
            Register::PC => self.reg_pc,
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
        };
    }

    pub fn set_reg_from_string(&mut self, str: &str, value: u16) {
        return match self.get_reg_str(str) {
            Register::PC => self.reg_pc = value,
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
            Register::C => self.reg_c = value,
        };
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
        self.set_reg_from_string(args[1], args[2].parse().expect("aaaaaaaaaaaa"));
    }

    pub fn prab(&self, args: Vec<&str>) {
        println!("{:#b}", self.reg_from_string(args[1]));
    }

    pub fn end(&mut self) {
        self.stopped = true;
    }

    pub fn ld(&mut self, args: Vec<&str>, memory: &Memory) {
        let val = memory.get_ram(args[2].parse().expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
        self.set_reg_from_string(args[1], val);
    }

    pub fn str(&self, args: Vec<&str>, memory: &mut Memory) {
        let val = self.reg_from_string(args[1]);
        memory.set_ram(args[2].parse().expect("BBBBBBBBBBBBBBBBBBBBBBB"), val);
    }
}
