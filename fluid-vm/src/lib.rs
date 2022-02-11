mod tools;
mod allocator;
mod heap;
mod buffer;

use std::{ mem, fmt::Display, marker::PhantomData };
use tools::*;
use allocator::*;
use heap::*;
use buffer::*;

pub struct VM<T>
where T: Copy, T: Display {
    ip: Address,
    size: PhantomData<T>,
    input: Buffer,
    output: Buffer,
    heap: Heap,
    instructions: Vec<fn(&mut VM<T>)>,
    bytecode: Vec<u8>,
}

impl<T> VM<T> 
where T: Copy, T: Display {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let mut vm = VM {
            ip: 0,
            size: PhantomData,
            input: Buffer::new(),
            output: Buffer::new(),
            heap: Heap::new(),
            instructions: Vec::new(),
            bytecode
        };

        vm.generate_instructions();
        vm
    }

    pub fn execute(&mut self) {
        while self.ip < self.bytecode.len() {
            let instruction = self.bytecode[self.ip];
            self.execute_instruction(instruction);
            self.ip += 1;
        }
    }

    fn execute_instruction(&mut self, instruction: u8) {
        if (instruction as usize) < self.instructions.len() {
            self.instructions[instruction as usize](self);
        } else {
            panic!("Index of instruction out of bounds");
        }
    }

    fn generate_instructions(&mut self)  {
        self.instructions = vec![
            VM::nop, // 0
        ];
    }

    /// does nothing
    fn nop(&mut self) {    
    }

    fn 
}
