mod cpu;
mod machine;
mod memory;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./gooberator program-file-path ref-file-path");
        return;
    }

    let mut machine = machine::Machine::new(&args[1], &args[2]);
    machine.start();
}
