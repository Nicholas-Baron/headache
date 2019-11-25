
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// The Headache Interpreter for BrainFuck
/// Complies to http://www.muppetlabs.com/~breadbox/bf/standards.html

#[derive(StructOpt)]
#[structopt(name="headache", about="An interpreter for Brainfuck")]
struct CliOpt{

    /// Enables debug outputs
    #[structopt(short,long)]
    debug: bool,

    /// Input filepath
    input: PathBuf,
}

fn load_program(filename : &str) -> Option<String>{

    fn is_bf_command(c : char) -> bool{
        c == '.' || c == ',' || c == '[' || c == ']'
            || c == '<' || c == '>' || c == '+' || c == '-'
    }

    match fs::read_to_string(filename){
        Err(_) => { println!("Could not read file"); None },
        Ok(data) => Some(data.chars().filter(|&x| is_bf_command(x)).collect())
    }
}


fn main(){

    let opt = CliOpt::from_args();
    // First, read the program

    let program = load_program(opt.input.into_os_string().to_str().expect("Could not read input file path"));

    // Then, execute it in some machine
}
