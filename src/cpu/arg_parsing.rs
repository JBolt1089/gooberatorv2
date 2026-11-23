use super::*;

impl Cpu {
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

    pub fn get_from_arg(&self, str: &str, mem: &Memory) -> u16 {
        match str.to_lowercase() {
            s if s.starts_with("a_") => mem.get_ram(parse_num_arg(s.strip_prefix("a_").unwrap())),
            s if s.starts_with("r_") => self.reg_from_string(s.strip_prefix("r_").unwrap()),
            s if s.starts_with("i_") => parse_num_arg(str.strip_prefix("i_").unwrap()),
            s if s.starts_with("p_") => {
                mem.get_ram(mem.get_ram(parse_num_arg(s.strip_prefix("p_").unwrap())))
            }
            _ => {
                eprintln!("prefix could not be matched! pc is {}", self.reg_pc);
                std::process::exit(69);
            }
        }
    }

    pub fn set_from_arg(&mut self, str: &str, mem: &mut Memory, value: u16) {
        match str.to_lowercase() {
            s if s.starts_with("a_") => {
                mem.set_ram(parse_num_arg(str.strip_prefix("a_").unwrap()), value)
            }
            s if s.starts_with("r_") => {
                self.set_reg_from_string(str.strip_prefix("r_").unwrap(), value);
            }
            s if s.starts_with("p_") => {
                let address = mem.get_ram(parse_num_arg(s.strip_prefix("p_").unwrap()));
                mem.set_ram(address, value);
            }
            _ => {
                eprintln!("prefix could not be matched! pc is {}", self.reg_pc);
                std::process::exit(69);
            }
        }
    }
}
