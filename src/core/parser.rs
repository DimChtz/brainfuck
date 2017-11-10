use super::instruction::{mapper, Instruction};
use super::program::Program;
use super::error::Error;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;

pub struct Parser {

    code_buffer:Vec<u8>,

}

impl Parser {
     
    // Function to create and return a new Parser
    pub fn new() -> Parser {

        Parser {
            code_buffer:Vec::<u8>::new(),
        }

    }

    // Function to load code from simple text
    pub fn load<S: Into<String>>(&mut self, buffer:S) -> Result<(), Error> {

        self.code_buffer = buffer.into().into_bytes();

        match self.code_buffer.len() {

            l if l > 0 => Result::Ok(()),

            _ => Result::Err(Error::EmptyProgram),

        }

    }

    // Function to load code from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, src:P) -> Result<(), Error> {

        match File::open(src) {

            Ok(mut file) => {
                
                let mut content = String::new();

                match file.read_to_string(&mut content) {

                    _ => self.load(content)

                }

            }

            Err(_) => Result::Err(Error::NoSuchFile),

        }

    }

    // Function to compile the code into intermidiate code (program)
    pub fn parse(&mut self) -> Result<Program, Error> {

        // Create a new program
        let mut prog = Program::new();

        // Create a stack for the loop brackets
        let mut loop_stack = Vec::<usize>::new();

        // Create a code buffer ptr
        let mut code_ptr:usize = 0;

        // Compile the code buffer and feed the program
        if self.code_buffer.len() != 0 {
            
            loop {

                match self.code_buffer[code_ptr] as char {

                    b @ '+' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ '-' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ '>' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ '<' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ ',' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ '.' => { prog.push(mapper::get_instr(b).unwrap()); }
                    b @ '[' => { 

                        // Add the instruction to the program (leave 0 for now)
                        prog.push(mapper::get_instr(b).unwrap());

                        // Update the loop stack
                        loop_stack.push(prog.get_size() - 1);

                     }
                    ']' => { 

                        if loop_stack.is_empty() {

                            // Missmatch `]` bracket found
                            return Result::Err(Error::MissingOpenBracket);

                        }

                        let curr_ptr = prog.get_size();

                        let last_open_bracket_ptr = loop_stack.pop().unwrap();

                        prog.update_instr(last_open_bracket_ptr, Instruction::OpenBracket(curr_ptr));

                        prog.push(Instruction::CloseBracket(last_open_bracket_ptr));

                     }
                    _   => { /* IGNORE UNUSED SYMBOLS */ }

                };

                code_ptr += 1;

                if code_ptr >= self.code_buffer.len() {
                    break;
                }

            }

            // Check if loop_stack is empty or not -> not = error (missmatched brackets)
            if !loop_stack.is_empty() {
                return Result::Err(Error::MissingCloseBracket);
            }

        } else {

            // The code buffer is empty -> empty code loaded or just never loaded
            return Result::Err(Error::EmptyProgram);

        }
        
        // Program is empty return Error (code buffer contains only unused symbols)
        if prog.get_size() == 0 {
            return Result::Err(Error::EmptyProgram);
        }

        return Result::Ok(prog);

    }

}