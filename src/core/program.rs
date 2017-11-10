use super::instruction::Instruction;

// Struct for the program (a series of instructions)
pub struct Program {
    
    instr:Vec<Instruction>,
    ptr:usize,

}

impl Program {

    // Function to create and return a new program
    pub fn new() -> Program {

        Program {

            instr:Vec::<Instruction>::new(),
            ptr:0,

        }

    }

    // Function to increase the pointer
    pub fn inc_ptr(&mut self) -> bool {

        if self.ptr < self.instr.len() - 1 {
            self.ptr += 1;
            return true;
        }

        false

    }

    // Function to decrease the pointer
    #[allow(unused)]
    pub fn dec_ptr(&mut self) -> bool {

        if self.ptr > 0 {
            self.ptr -= 1;
            return true;
        }

        false

    }

    // Function to move the pointer
    pub fn set_ptr(&mut self, pos:usize) -> bool {

        if pos >= self.get_size() {
            return false;
        }

        self.ptr = pos;

        true

    }

    // Function to get the current instruction (if a program is already loaded and the instruction exists)
    pub fn get(&self) -> Option<Instruction> {

        if self.instr.len() > 0 { Option::Some(self.instr[self.ptr]) } else { Option::None }

    }

    // Function to increase the pointer and get the instruction
    #[allow(unused)]
    pub fn get_next(&mut self) -> Option<Instruction> {

        if self.inc_ptr() { self.get() } else { Option::None }

    }

    // Function to decrease the pointer and get the instruction
    #[allow(unused)]
    pub fn get_prev(&mut self) -> Option<Instruction> {

        if self.dec_ptr() { self.get() } else { Option::None }

    }

    // Function to get an instruction
    #[allow(unused)]
    pub fn get_at(&self, p:usize) -> Option<Instruction> {

        if p >= (0 as usize) && p < self.instr.len() { Option::Some(self.instr[p]) } else { Option::None }

    }

    // Function to add a new instruction at the end of the program
    pub fn push(&mut self, i:Instruction) -> &mut Program {

        self.instr.push(i);

        self

    }

    // Function to remove the last instruction from the program (if exists)
    #[allow(unused)]
    pub fn pop(&mut self) -> bool {
        
        return match self.instr.pop() {

            Some(_) => {
                
                if self.ptr >= self.instr.len() {
                    if self.ptr > 0 {
                        self.ptr -= 1;
                    }
                }

                true

            }

            None => false

        }

    }

    // Function to clear the program and set pointer to 0
    #[allow(unused)]
    pub fn clear(&mut self) -> &mut Program {

        self.instr.clear();
        self.ptr = 0;

        self

    }
    
    // Function to reset the pointer (set to 0)
    #[allow(unused)]
    pub fn reset_ptr(&mut self) -> &mut Program {

        self.ptr = 0;

        self

    }

    // Function to update a specific instruction
    pub fn update_instr(&mut self, pos:usize, new_instr:Instruction) -> bool {

        if self.instr.len() == 0 || pos >= self.instr.len() {
            return false;
        }

        self.instr[pos] = new_instr;

        return true;

    }

    // Function to get program's size
    pub fn get_size(&self) -> usize {

        self.instr.len()

    }

}