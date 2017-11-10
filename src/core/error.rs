use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {

    NoSuchFile,
    EmptyProgram,
    MissingOpenBracket,
    MissingCloseBracket,
    PtrOverflow,
    PtrUnderflow,
    ValOverflow,
    ValUnderflow,

}

impl error::Error for Error {

    fn description(&self) -> &str {

        match *self {

            Error::NoSuchFile => "Failed to open file",
            Error::EmptyProgram => "The program is empty",
            Error::MissingOpenBracket => "Missing open bracket(s)",
            Error::MissingCloseBracket => "Missing close bracket(s)",
            Error::PtrOverflow => "Memory pointer overflow",
            Error::PtrUnderflow => "Memory pointer underflow",
            Error::ValOverflow => "Memory value overflow",
            Error::ValUnderflow => "Memory value underflow",

        }

    }

}

impl fmt::Display for Error {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        match *self {

            Error::NoSuchFile => write!(f, "Couldn't open file."),
            Error::EmptyProgram => write!(f, "The program is empty."),
            Error::MissingOpenBracket => write!(f, "Missing open bracket(s)."),
            Error::MissingCloseBracket => write!(f, "Missing close bracket(s)."),
            Error::PtrOverflow => write!(f,"Memory pointer overflow."),
            Error::PtrUnderflow => write!(f,"Memory pointer underflow."),
            Error::ValOverflow => write!(f,"Memory value overflow."),
            Error::ValUnderflow => write!(f,"Memory value underflow."),

        }

    }

}