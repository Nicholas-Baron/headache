use std::collections::hash_map::HashMap;
use std::fmt;

/// These are the eight commands that Brainfuck supports.
#[derive(PartialEq)]
pub enum Command {
    MoveLeft,
    MoveRight,
    AddOne,
    SubOne,
    Output,
    Input,
    JumpForward,
    JumpBack,
}

/// This function formats a character into a BF command, returning `None` if the character is not
/// used by BF.
/// ```
/// assert!(to_command('/') == None);
/// assert!(to_command(',') == Some(Input));
/// ```
pub fn to_command(c: char) -> Option<Command> {
    use Command::*;
    match c {
        '<' => Some(MoveLeft),
        '>' => Some(MoveRight),
        '+' => Some(AddOne),
        '-' => Some(SubOne),
        '.' => Some(Output),
        ',' => Some(Input),
        '[' => Some(JumpForward),
        ']' => Some(JumpBack),
        _ => None,
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Command::*;
        match *self {
            MoveLeft => write!(f, "<"),
            MoveRight => write!(f, ">"),
            AddOne => write!(f, "+"),
            SubOne => write!(f, "-"),
            Input => write!(f, "."),
            Output => write!(f, ","),
            JumpForward => write!(f, "["),
            JumpBack => write!(f, "]"),
        }
    }
}

pub struct TuringMachine {
    ptr: i32,
    ram: HashMap<i32, u32>,
    pc: usize,
    program: Vec<Command>,
    user_input: String,
}

impl TuringMachine {
    pub fn with_program(program: Vec<Command>) -> Self {
        TuringMachine {
            ptr: 0,
            pc: 0,
            ram: HashMap::new(),
            program,
            user_input: String::new(),
        }
    }

    fn read_user_input(&mut self) -> char {
        if self.user_input.is_empty() {
            use std::io::{stdin, stdout, Write};
            let _ = stdout().flush();
            stdin()
                .read_line(&mut self.user_input)
                .expect("Could not read user input.");

            if let Some('\n') = self.user_input.chars().next_back() {
                self.user_input.pop();
            }

            if let Some('\r') = self.user_input.chars().next_back() {
                self.user_input.pop();
            }
        }

        self.user_input.drain(..1).next().unwrap()
    }

    fn read_ram_ptr(&self) -> &u32 {
        self.ram.get(&self.ptr).unwrap_or(&0)
    }

    fn write_ram_ptr(&mut self, data: u32) {
        self.ram.insert(self.ptr, data);
    }

    pub fn step(&mut self) {
        use Command::*;
        match self.program[self.pc] {
            MoveLeft => self.ptr -= 1,
            MoveRight => self.ptr += 1,
            AddOne => self.write_ram_ptr(self.read_ram_ptr() + 1),
            SubOne => self.write_ram_ptr(self.read_ram_ptr() - 1),
            Output => print!("{}", std::char::from_u32(*self.read_ram_ptr()).unwrap()),
            Input => {
                let value = self.read_user_input() as u32;
                self.write_ram_ptr(value);
            }
            JumpForward => {
                if self.read_ram_ptr() == &0 {
                    let mut depth = 0;
                    while self.program[self.pc] != JumpBack || depth > 0 {
                        self.pc += 1;
                        match self.program[self.pc] {
                            JumpForward => depth += 1,
                            JumpBack => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            JumpBack => {
                if self.read_ram_ptr() != &0 {
                    let mut depth = 0;
                    while self.program[self.pc] != JumpForward || depth > 0 {
                        self.pc -= 1;
                        match self.program[self.pc] {
                            JumpForward => depth -= 1,
                            JumpBack => depth += 1,
                            _ => {}
                        }
                    }
                }
            }
        };
        self.pc += 1;
    }

    pub fn run(&mut self, debug: bool) {
        while self.pc < self.program.len() {
            self.step();
            if debug {
                println!(
                    "PC: {:3} | PTR: {:3} @ {:3}",
                    self.pc,
                    self.read_ram_ptr(),
                    self.ptr
                );
            }
        }
    }
}
