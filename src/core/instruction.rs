use std::fmt::{self, Display};

#[derive(Copy, Clone)]
pub enum Instruction {

    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Input,
    Output,
    OpenBracket(usize),
    CloseBracket(usize),

}

impl Display for Instruction {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        
        match *self {

            Instruction::IncPtr => write!(f, ">"),
            Instruction::DecPtr => write!(f, "<"),
            Instruction::IncVal => write!(f, "+"),
            Instruction::DecVal => write!(f, "-"),
            Instruction::Input  => write!(f, ","),
            Instruction::Output => write!(f, "."),
            Instruction::OpenBracket(_) => write!(f, "["),
            Instruction::CloseBracket(_) => write!(f, "]"),

        }

    }

}

pub mod mapper {

    use super::Instruction;

    pub fn get_instr(c:char) -> Option<super::Instruction> {

        match c {

            '>' => Option::Some(Instruction::IncPtr),
            '<' => Option::Some(Instruction::DecPtr),
            '+' => Option::Some(Instruction::IncVal),
            '-' => Option::Some(Instruction::DecVal),
            ',' => Option::Some(Instruction::Input),
            '.' => Option::Some(Instruction::Output),
            '[' => Option::Some(Instruction::OpenBracket(0usize)),
            ']' => Option::Some(Instruction::CloseBracket(0usize)),
            _   => Option::None,

        }

    }

    #[allow(unused)]
    pub fn get_char(instr:Instruction) -> char {

        match instr {

            Instruction::IncPtr => '>',
            Instruction::DecPtr => '<',
            Instruction::IncVal => '+',
            Instruction::DecVal => '-',
            Instruction::Input => ',',
            Instruction::Output => '.',
            Instruction::OpenBracket(_) => '[',
            Instruction::CloseBracket(_) => ']',

        }

    }

}