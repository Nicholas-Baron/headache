use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod turing_machine;
use turing_machine::{Command, TuringMachine};

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

fn load_program(filename: &str) -> Option<Vec<Command>> {
    match fs::read_to_string(filename) {
        Err(_) => {
            println!("Could not read file");
            None
        }
        Ok(data) => {
            println!("{}", data);
            let mut to_ret = Vec::with_capacity(data.len());
            for c in data.chars() {
                if let Some(cmd) = turing_machine::to_command(c) {
                    to_ret.push(cmd);
                }
            }
            Some(to_ret)
        }
    }
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
        for cmd in &program {
            print!("{} ", cmd);
        }
        println!();
    }

    // Then, execute it in some machine
    let mut machine = TuringMachine::with_program(program);

    machine.run(opt.debug);
}
