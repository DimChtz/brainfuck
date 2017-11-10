mod core;

use core::parser::Parser;
use core::program::Program;
use core::instruction::Instruction;
use core::error::Error;
use core::memory::Memory;

use std::io;
use std::io::prelude::*;
use std::path::Path;

// Struct for the bf Interpreter (brainpreter)
pub struct Inter {

    tape:Memory,
    program:Program,
    parser:Parser,

}

impl Inter {

    // Function to create and return a new Interpreter
    pub fn new() -> Inter {

        Inter {

            tape: Memory::new(),
            program: Program::new(),
            parser: Parser::new(),

        }

    }

    // Function to load code into the parser (text)
    pub fn load<S: Into<String>>(&mut self, p:S) -> Result<(), Error> {

        self.parser.load(p)

    }

    // Function to load code into the parser (file)
    pub fn load_from_file<P: AsRef<Path>>(&mut self, src:P) -> Result<(), Error> {

        self.parser.load_from_file(src)

    }

    // Function to compile the code
    pub fn parse(&mut self) -> Result<(), Error> {

        // Call the parser
        match self.parser.parse() {

            Ok(p) => {

                self.program = p;

                Ok(())

            }

            Err(e) => Result::Err(e),

        }

    }

    // Function to execute bf program
    pub fn run(&mut self) -> Result<(), Error> {

        // Check if a program is properly loaded
        if self.program.get_size() == 0 {
            return Result::Err(Error::EmptyProgram);
        }

        // Execute instructions
        while let Some(instr) = self.program.get() {

            match instr {

                Instruction::IncPtr => { match self.inc_ptr() { Ok(_) => {  } Err(e) => return Result::Err(e), } }
                Instruction::DecPtr => { match self.dec_ptr() { Ok(_) => {  } Err(e) => return Result::Err(e), } }
                Instruction::IncVal => { match self.inc_val() { Ok(_) => {  } Err(e) => return Result::Err(e), } }
                Instruction::DecVal => { match self.dec_val() { Ok(_) => {  } Err(e) => return Result::Err(e), } }
                Instruction::Input  => { self.input(); }
                Instruction::Output => { self.output(); }
                Instruction::OpenBracket(n)  => { self.open_bracket(n); }
                Instruction::CloseBracket(n) => { self.close_bracket(n); }

            };

            // If this was the last instruction
            if !self.program.inc_ptr() {
                break;
            }

        };

        return Result::Ok(());

    }

    // Function to increase the value on the memory (tape)
    fn inc_val(&mut self) -> Result<(), Error> {

        self.tape.inc_val()

    }

    // Function to decrease the value on the memory (tape)
    fn dec_val(&mut self) -> Result<(), Error> {

        self.tape.dec_val()

    }

    // Function to increase the pointer on the memory (tape)
    fn inc_ptr(&mut self) -> Result<(), Error> {

        self.tape.inc_ptr()

    }

    // Function to decrease the pointer on the memory (tape)
    fn dec_ptr(&mut self) -> Result<(), Error> {

        self.tape.dec_ptr()

    }

    // Function to read into the memory (tape)
    fn input(&mut self) {

        match io::stdin().bytes().next() {

            Some(v) => self.tape.set_val(v.unwrap()),

            None => {  }

        }

    }

    // Function to print a char from memory (tape)
    fn output(&mut self) {

        print!("{}", self.tape.get_val() as char);
        
    }

    // Function to handle the open bracket
    fn open_bracket(&mut self, pos:usize) {
        
        if self.tape.get_val() == 0 {
            self.program.set_ptr(pos);
        }

    }

    // Function to handle the close bracket
    fn close_bracket(&mut self, pos:usize) {

        if self.tape.get_val() != 0 {
            self.program.set_ptr(pos);
        }

    }

}
