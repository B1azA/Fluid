mod tools;
mod allocator;
mod heap;
mod buffer;

use std::{ mem, fmt::Display, marker::PhantomData, fmt::Debug };
use tools::*;
use allocator::*;
use heap::*;
use buffer::*;

pub struct VM {
    ip: usize,
    input: Buffer,
    output: Buffer,
    heap: Heap,
    instructions: Vec<fn(&mut VM)>,
    bytecode: Vec<u8>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let mut vm = VM {
            ip: 0,
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
            VM::popi,       // 3
            VM::set,        // 4
            VM::get,        // 5
            VM::geti,       // 6
            VM::clear_i,    // 7
            VM::clear_o,    // 8
            VM::save,       // 9
            VM::savei,      // 10
            VM::load,       // 11
            VM::loadi,      // 12
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

    /// args: type, index
    /// 
    /// pops value from output and sets it to input at index 
    fn popi(&mut self) {
        let value = self.output.pop();
        let index = self.get_index();
        self.input.set(index, value);
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
    /// pushes value of output to index
    fn get(&mut self) {
        let index = self.get_index();
        self.input.push(self.output.get(index))
    }

    /// args: type, output_index, type, input_index
    /// 
    /// gets value of output at index and sets it to input at index
    fn geti(&mut self) {
        let o_index = self.get_index();
        let i_index = self.get_index();
        let value = self.output.get(o_index);
        self.input.set(i_index, value);
    }

    /// clears input
    fn clear_i(&mut self) {
        self.input.clear();
    }

    /// clears output
    fn clear_o(&mut self) {
        self.output.clear();
    }

    /// args: type, value
    /// 
    /// saves value to heap and pushes its address to input
    fn save(&mut self) {
        let ptr = self.get_value_as_ptr();
        let address = self.heap.add(ptr);
        self.input.push(Immediate::U64(address as u64));
    }

    /// args: type_of_value, value, type_of_index, index
    /// 
    /// saves value to heap and sets its address to input at index
    fn savei(&mut self) {
        let ptr = self.get_value_as_ptr();
        let address = self.heap.add(ptr);
        let index = self.get_index();
        self.input.set(index, Immediate::U64(address as u64));
    }

    /// args: address (u64), type_of_value
    /// 
    /// loads value from address at heap and pushes it to input
    fn load(&mut self) {
        let address = self.get_address();
        let ptr = self.heap.get(address);
        self.ip += 1;
        match self.bytecode[self.ip] {
            0 => { // u8
                let value = Immediate::U8(ptr.get_data::<u8>());
                self.input.push(value);
            }

            1 => { // u16
                let value = Immediate::U16(ptr.get_data::<u16>());
                self.input.push(value);
            }

            2 => { // u32
                let value = Immediate::U32(ptr.get_data::<u32>());
                self.input.push(value);
            }

            3 => { // u64
                let value = Immediate::U64(ptr.get_data::<u64>());
                self.input.push(value);
            }

            4 => { // i8
                let value = Immediate::I8(ptr.get_data::<i8>());
                self.input.push(value);
            }

            5 => { // i16
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.input.push(value);
            }

            6 => { // i32
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.input.push(value);
            }

            7 => { // i64
                let value = Immediate::I64(ptr.get_data::<i64>());
                self.input.push(value);
            }

            8 => { // f32
                let value = Immediate::F32(ptr.get_data::<f32>());
                self.input.push(value);
            }

            9 => { // f64
                let value = Immediate::F64(ptr.get_data::<f64>());
                self.input.push(value);
            }

            10 => { // bool
                let value = Immediate::BOOL(ptr.get_data::<bool>());
                self.input.push(value);
            }

            _ => {}
        }
    }

    /// args: address (u64), type_of_index, index, type_of_value,
    /// 
    /// loads value from address at heap and sets it to input at index
    fn loadi(&mut self) {
        let address = self.get_address();
        let ptr = self.heap.get(address);
        let index = self.get_index();
        self.ip += 1;
        match self.bytecode[self.ip] {
            0 => { // u8
                let value = Immediate::U8(ptr.get_data::<u8>());
                self.input.set(index, value);
            }

            1 => { // u16
                let value = Immediate::U16(ptr.get_data::<u16>());
                self.input.set(index, value);
            }

            2 => { // u32
                let value = Immediate::U32(ptr.get_data::<u32>());
                self.input.set(index, value);
            }

            3 => { // u64
                let value = Immediate::U64(ptr.get_data::<u64>());
                self.input.set(index, value);
            }

            4 => { // i8
                let value = Immediate::I8(ptr.get_data::<i8>());
                self.input.set(index, value);
            }

            5 => { // i16
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.input.set(index, value);
            }

            6 => { // i32
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.input.set(index, value);
            }

            7 => { // i64
                let value = Immediate::I64(ptr.get_data::<i64>());
                self.input.set(index, value);
            }

            8 => { // f32
                let value = Immediate::F32(ptr.get_data::<f32>());
                self.input.set(index, value);
            }

            9 => { // f64
                let value = Immediate::F64(ptr.get_data::<f64>());
                self.input.set(index, value);
            }

            10 => { // bool
                let value = Immediate::BOOL(ptr.get_data::<bool>());
                self.input.set(index, value);
            }

            _ => {}
        }
    }

    fn get_immediate(&mut self) -> Immediate {
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
                self.ip += size - 1;
                Immediate::U16(value)
            }

            2 => { // u32
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::U32(value)
            }

            3 => { // u64
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
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
                self.ip += size - 1;
                Immediate::I16(value)
            }

            6 => { // i32
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::I32(value)
            }

            7 => { // i64
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::I64(value)
            }

            8 => { // f32
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::F32(value)
            }

            9 => { // f64
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                Immediate::F64(value)
            }

            10 => { // bool
                self.ip += 1;
                let value = self.bytecode[self.ip] != 0;
                Immediate::BOOL(value)
            }

            11 => { // maximum index of input
                let index = self.input.len();
                if index > 0 {
                    return Immediate::U64(index - 1);
                }

                Immediate::U64(index)
            }

            12 => { // maximum index of output
                let index = self.output.len();
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
                self.ip += size - 1;
                value as usize
            }

            2 => { // u32
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                value as usize
            }

            3 => { // u64
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
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
                self.ip += size - 1;
                value as usize
            }

            6 => { // i32
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                value as usize
            }

            7 => { // i64
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                value as usize
            }

            8 => { // f32
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                value as usize
            }

            9 => { // f64
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                self.ip += size - 1;
                value as usize
            }

            10 => { // bool
                self.ip += 1;
                let value = self.bytecode[self.ip] != 0;
                value as usize
            }

            11 => { // maximum index of input
                let index = self.input.len();
                if index > 0 {
                    return (index - 1) as usize;
                }

                index as usize
            }

            12 => { // maximum index of output
                let index = self.output.len();
                if index > 0 {
                    return (index - 1) as usize;
                }

                index as usize
            }

            _ => { 0 as usize}
        }
    }

    fn get_value_as_ptr(&mut self) -> Ptr {
        self.ip += 1;

        match self.bytecode[self.ip] {
            0 => { // u8
                self.ip += 1;
                let value = self.bytecode[self.ip];
                let ptr = Ptr::allocate(mem::size_of::<u8>());
                ptr.set_data(value);
                ptr
            }

            1 => { // u16
                self.ip += 1;
                let size = mem::size_of::<u16>();
                let value = u16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            2 => { // u32
                self.ip += 1;
                let size = mem::size_of::<u32>();
                let value = u32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            3 => { // u64
                self.ip += 1;
                let size = mem::size_of::<u64>();
                let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            4 => { // i8
                self.ip += 1;
                let value = self.bytecode[self.ip] as i8;
                let ptr = Ptr::allocate(mem::size_of::<i8>());
                ptr.set_data(value);
                ptr
            }

            5 => { // i16
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            6 => { // i32
                self.ip += 1;
                let size = mem::size_of::<i32>();
                let value = i32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            7 => { // i64
                self.ip += 1;
                let size = mem::size_of::<i64>();
                let value = i64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            8 => { // f32
                self.ip += 1;
                let size = mem::size_of::<f32>();
                let value = f32::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            9 => { // f64
                self.ip += 1;
                let size = mem::size_of::<f64>();
                let value = f64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
                let ptr = Ptr::allocate(size);
                ptr.set_data(value);
                self.ip += size - 1;
                ptr
            }

            10 => { // bool
                self.ip += 1;
                let value = self.bytecode[self.ip] != 0;
                let ptr = Ptr::allocate(mem::size_of::<bool>());
                ptr.set_data(value);
                ptr
            }

            11 => { // maximum index of input
                let mut index = self.input.len();
                if index > 0 {
                    index -= 1;
                }
                let ptr = Ptr::allocate(mem::size_of::<u64>());
                ptr.set_data(index);
                ptr
            }

            12 => { // maximum index of output
                let mut index = self.output.len();
                if index > 0 {
                    index -= 1;
                }
                let ptr = Ptr::allocate(mem::size_of::<u64>());
                ptr.set_data(index);
                ptr
            }

            _ => { Ptr::allocate(0) }
        }
    }

    fn get_address(&mut self) -> Address {
        self.ip += 1;
        let size = mem::size_of::<u64>();
        let value = u64::from_be_bytes(self.bytecode[self.ip..self.ip + size].try_into().unwrap());
        self.ip += size - 1;
        value as usize
    }
}
