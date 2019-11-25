use std::collections::{hash_map, HashMap};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// The Headache Interpreter for BrainFuck
/// Complies to http://www.muppetlabs.com/~breadbox/bf/standards.html

#[derive(StructOpt)]
#[structopt(name = "headache", about = "An interpreter for Brainfuck")]
struct CliOpt {
    /// Enables debug outputs
    #[structopt(short, long)]
    debug: bool,

    /// Input filepath
    input: PathBuf,
}

fn load_program(filename: &str) -> Option<String> {
    fn is_bf_command(c: char) -> bool {
        c == '.' || c == ',' || c == '[' || c == ']' || c == '<' || c == '>' || c == '+' || c == '-'
    }

    match fs::read_to_string(filename) {
        Err(_) => {
            println!("Could not read file");
            None
        }
        Ok(data) => Some(data.chars().filter(|&x| is_bf_command(x)).collect()),
    }
}

fn run_program(commands: String) -> (i32, HashMap<i32, u8>) {
    let mut ptr = 0;
    let mut ram = HashMap::new();
    let mut pc = 0;
    loop {
        let command = commands.chars().nth(pc);
        if command.is_none() {
            break;
        }

        let command = command.unwrap();
        match command {
            '.' => {
                use hash_map::Entry::Occupied;
                if let Occupied(entry) = ram.entry(ptr) {
                    print!("{}", entry.get());
                }
            }
            ',' => {}
            '<' => ptr -= 1,
            '>' => ptr += 1,
            '+' => {
                match ram.insert(ptr, 1) {
                    Some(value) => ram.insert(ptr, value + 1),
                    None => None,
                };
            }
            '[' => {
                if ram.get(&ptr).unwrap_or(&0) == &0 {
                    loop {
                        let command = commands.chars().nth(pc);
                        match command {
                            None => break,
                            Some(cmd) => {
                                if cmd == ']' {
                                    break;
                                } else {
                                    pc += 1;
                                }
                            }
                        }
                    }
                }
            }
            _ => panic!("Unimplemented BF command: {} @ {}", command, pc),
        }
        pc += 1;
    }

    (ptr, ram)
}

fn main() {
    let opt = CliOpt::from_args();

    // First, read the program
    let program = load_program(
        opt.input
            .into_os_string()
            .to_str()
            .expect("Could not read input file path"),
    )
    .expect("Could not read program");

    if opt.debug {
        println!("Program loaded: {}", program);
    }

    // Then, execute it in some machine
    let final_state = run_program(program);

    if opt.debug {
        let width = 10;
        let center = final_state.0;

        for i in center - (width / 2)..center + (width / 2) {
            print!("{:3}", i);
        }
        println!();

        for i in center - (width / 2)..center + (width / 2) {
            print!("{:3}", final_state.1.get(&i).unwrap_or(&0));
        }
        println!();
    }
}
