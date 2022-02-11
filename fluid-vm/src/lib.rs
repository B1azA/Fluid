mod tools;
mod allocator;
mod heap;
mod buffer;

use std::{ mem, fmt::Display, marker::PhantomData, fmt::Debug };
use tools::*;
use allocator::*;
use heap::*;
use buffer::*;

macro_rules! as_t {
    ($a:expr,$typ:ty)=>{
        $a as $typ
    }
}

pub trait Bytes {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_usize(&self) -> usize;
    fn to_u64(&self) -> u64;
}

impl Bytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl Bytes for u16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u16::from_be_bytes(bytes.try_into().unwrap())
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl Bytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u32::from_be_bytes(bytes.try_into().unwrap())
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl Bytes for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u64::from_be_bytes(bytes.try_into().unwrap())
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_u64(&self) -> u64 {
        *self
    }
}

pub struct VM<T>
where T: Copy, T:Bytes, T: Display, T:Debug {
    ip: Address,
    size: PhantomData<T>,
    input: Buffer<T>,
    output: Buffer<T>,
    heap: Heap,
    instructions: Vec<fn(&mut VM<T>)>,
    bytecode: Vec<u8>,
}

impl<T> VM<T> 
where T: Copy, T:Bytes, T: Display, T: Debug {
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
        self.output.push(Immediate::BOOL(true));
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
            VM::nop,        // 0
            VM::push,       // 1
            VM::pop,        // 2
            VM::set,        // 3
            VM::get,        // 4
            VM::clear_i,    // 5
            VM::clear_o,    // 6
        ];
    }

    /// does nothing
    fn nop(&mut self) {    
    }

    /// args: type, value 
    /// 
    /// pushes value to input
    fn push(&mut self) {
        let value = self.get_immediate();
        self.input.push(value);
    }

    /// pops value from output and pushes it to input
    fn pop(&mut self) {
        let value = self.output.pop();
        self.input.push(value);
    }

    /// args: type, value, type, index
    /// 
    /// sets value of input at index
    fn set(&mut self) {
        let value = self.get_immediate();
        let index = self.get_index();
        self.input.set(index, value);
    }

    /// args: type, index
    /// 
    /// pushes value of output at index to input
    fn get(&mut self) {
        let index = self.get_index();
        self.input.push(self.output.get(index))
    }

    /// clears input
    fn clear_i(&mut self) {
        self.input.clear();
    }

    /// clears output
    fn clear_o(&mut self) {
        self.output.clear();
    }

    fn get_immediate(&mut self) -> Immediate<T> {
        self.ip += 1;

        match self.bytecode[self.ip] {
            0 => { // u8
                self.ip += 1;
                let value = self.bytecode[self.ip];
                Immediate::U8(value)
            }

            1 => { // u16
                self.ip += 1;
                let size = mem::size_of::<u16>();
                let value = u16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::U16(value)
            }

            2 => { // u32
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::U32(value)
            }

            3 => { // u64
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::U64(value)
            }

            4 => { // i8
                self.ip += 1;
                let value = self.bytecode[self.ip] as i8;
                Immediate::I8(value)
            }

            5 => { // i16
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::I16(value)
            }

            6 => { // i32
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::I32(value)
            }

            7 => { // i64
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::I64(value)
            }

            8 => { // f32
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::F32(value)
            }

            9 => { // f64
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::F64(value)
            }

            10 => { // bool
                self.ip += 1;
                let value = self.bytecode[self.ip] != 0;
                Immediate::BOOL(value)
            }

            11 => { // size
                self.ip += 1;
                let size = mem::size_of::<T>();
                let value = T::from_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                Immediate::<T>::SIZE(value)
            }

            12 => {
                let mut index = self.input.len();
                if index > 0 {
                    return Immediate::U64(index - 1);
                }

                Immediate::U64(index)
            }

            13 => {
                let mut index = self.output.len();
                if index > 0 {
                    return Immediate::U64(index - 1);
                }

                Immediate::U64(index)
            }

            _ => { Immediate::NONE()}
        }
    }

    fn get_index(&mut self) -> usize {
        self.ip += 1;

        match self.bytecode[self.ip] {
            0 => { // u8
                self.ip += 1;
                let value = self.bytecode[self.ip];
                value as usize
            }

            1 => { // u16
                self.ip += 1;
                let size = mem::size_of::<u16>();
                let value = u16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            2 => { // u32
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            3 => { // u64
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            4 => { // i8
                self.ip += 1;
                let value = self.bytecode[self.ip] as i8;
                value as usize
            }

            5 => { // i16
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            6 => { // i32
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            7 => { // i64
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            8 => { // f32
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            9 => { // f64
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value as usize
            }

            10 => { // bool
                self.ip += 1;
                let value = self.bytecode[self.ip] != 0;
                value as usize
            }

            11 => { // size
                self.ip += 1;
                let size = mem::size_of::<T>();
                let value = T::from_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                value.to_usize()
            }

            _ => { 0 as usize}
        }
    }
}
