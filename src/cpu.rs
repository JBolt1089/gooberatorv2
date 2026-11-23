pub mod arg_parsing;
pub mod exec_next;
pub mod jumping;
pub mod logic;
pub mod macros;
pub mod math;
pub mod other;
pub mod printing;
pub mod comparison;

#[path = "cpu/memory.rs"]
pub mod memory_instructions;

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::memory::Memory;

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
}
