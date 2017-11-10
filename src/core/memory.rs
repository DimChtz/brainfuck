use super::error::Error;

pub const MEMORY_SIZE:usize = 30000;

// Memory tape
#[derive(Debug)]
pub struct Memory {
    
    cells: Vec<u8>,
    ptr:usize,

}

impl Memory {
     
    // Function to create and return a new Memory tape
    pub fn new() -> Memory {

        let mut v = Vec::<u8>::new();
        v.push(0);

        Memory {

            cells: v,
            ptr:0,

        }

    }

    // Function to move the pointer to the right
    pub fn inc_ptr(&mut self) -> Result<(), Error> {

        self.ptr += 1;

        match self.ptr {

            n if n >= MEMORY_SIZE => {

                Result::Err(Error::PtrOverflow)

            }

            _ => {
                
                if self.ptr >= self.cells.len() {

                    self.cells.push(0);

                }

                Result::Ok(())

            }

        }

    }

    // Function to move the poiner to the left
    pub fn dec_ptr(&mut self) -> Result<(), Error> {

        match self.ptr {

            n if n == 0 => {

                Result::Err(Error::PtrUnderflow)

            }

            _ => {

                self.ptr -= 1;

                Result::Ok(())

            }

        }

    }

    // Function to increase the value
    pub fn inc_val(&mut self) -> Result<(), Error> {

        match self.cells[self.ptr].checked_add(1) {
            
            Some(v) => {

                self.cells[self.ptr] = v;

                Result::Ok(())

            }

            None => Result::Err(Error::ValOverflow),

        }

    }

    // Function to decrease the value
    pub fn dec_val(&mut self) -> Result<(), Error> {

        match self.cells[self.ptr].checked_sub(1) {

            Some(v) => {

                self.cells[self.ptr] = v;

                Result::Ok(())

            }

            None => Result::Err(Error::ValUnderflow),

        }

    }
     
    // Function to get the current value
    pub fn get_val(&self) -> u8 {

        self.cells[self.ptr]

    }

    // Function to set the current value
    pub fn set_val(&mut self, v:u8) {

        self.cells[self.ptr] = v;

    }

}