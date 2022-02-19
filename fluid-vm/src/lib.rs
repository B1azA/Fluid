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
    jmp: bool,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let mut vm = VM {
            ip: 0,
            input: Buffer::new(),
            output: Buffer::new(),
            heap: Heap::new(),
            instructions: Vec::new(),
            bytecode,
            jmp: false,
        };

        vm.generate_instructions();
        vm
    }

    pub fn execute(&mut self) {
        while self.ip < self.bytecode.len() {
            let instruction = self.bytecode[self.ip];
            self.execute_instruction(instruction);
            if !self.jmp { self.ip += 1; }
            else { self.jmp = false; }
        }
    }

    pub fn clear(&mut self) {
        self.input = Buffer::new();
        self.output = Buffer::new();
        self.ip = 0;
        self.heap = Heap::new();
        self.jmp = false;
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
            VM::gen,        // 9
            VM::save,       // 10
            VM::savei,      // 11
            VM::load,       // 12
            VM::loadi,      // 13
            VM::less,       // 14
            VM::great,      // 15
            VM::eq,         // 16
            VM::jmp,        // 17
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

    /// args: type_of_elements, address
    /// 
    /// generates address of length and pushes it to output
    fn gen(&mut self) {
        self.ip += 1;
        let element_type = self.bytecode[self.ip];
        let length = self.get_address();

        match element_type {
            0 => { // u8
                let ptr = Ptr::allocate(length * mem::size_of::<u8>());
                let data: Vec<u8> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            1 => { // u16
                let ptr = Ptr::allocate(length * mem::size_of::<u16>());
                let data: Vec<u16> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            2 => { // u32
                let ptr = Ptr::allocate(length * mem::size_of::<u32>());
                let data: Vec<u32> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            3 => { // u64
                let ptr = Ptr::allocate(length * mem::size_of::<u64>());
                let data: Vec<u64> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            4 => { // i8
                let ptr = Ptr::allocate(length * mem::size_of::<i8>());
                let data: Vec<i8> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            5 => { // i16
                let ptr = Ptr::allocate(length * mem::size_of::<i16>());
                let data: Vec<i16> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            6 => { // i32
                let ptr = Ptr::allocate(length * mem::size_of::<i32>());
                let data: Vec<i32> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            7 => { // i64
                let ptr = Ptr::allocate(length * mem::size_of::<i64>());
                let data: Vec<i64> = vec![0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            8 => { // f32
                let ptr = Ptr::allocate(length * mem::size_of::<f32>());
                let data: Vec<f32> = vec![0.0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            9 => { // f64
                let ptr = Ptr::allocate(length * mem::size_of::<f64>());
                let data: Vec<f64> = vec![0.0;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            10 => { // bool
                let ptr = Ptr::allocate(length * mem::size_of::<bool>());
                let data: Vec<bool> = vec![false;length];
                ptr.set_data(data);
                self.output.push(Immediate::ADDRESS(self.heap.add(ptr)));
            }

            _ => {}
        }
    }

    /// args: type, value
    /// 
    /// saves value to heap and pushes its address to output
    fn save(&mut self) {
        let ptr = self.get_value_as_ptr();
        let address = self.heap.add(ptr);
        self.output.push(Immediate::U64(address as u64));
    }

    /// args: type_of_value, value, type_of_index, index
    /// 
    /// saves value to heap and sets its address to output at index
    fn savei(&mut self) {
        let ptr = self.get_value_as_ptr();
        let address = self.heap.add(ptr);
        let index = self.get_index();
        self.output.set(index, Immediate::U64(address as u64));
    }

    /// args: address (u64), type_of_value
    /// 
    /// loads value from address at heap and pushes it to output
    fn load(&mut self) {
        let address = self.get_address();
        let ptr = self.heap.get(address);
        self.ip += 1;
        match self.bytecode[self.ip] {
            0 => { // u8
                let value = Immediate::U8(ptr.get_data::<u8>());
                self.output.push(value);
            }

            1 => { // u16
                let value = Immediate::U16(ptr.get_data::<u16>());
                self.output.push(value);
            }

            2 => { // u32
                let value = Immediate::U32(ptr.get_data::<u32>());
                self.output.push(value);
            }

            3 => { // u64
                let value = Immediate::U64(ptr.get_data::<u64>());
                self.output.push(value);
            }

            4 => { // i8
                let value = Immediate::I8(ptr.get_data::<i8>());
                self.output.push(value);
            }

            5 => { // i16
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.output.push(value);
            }

            6 => { // i32
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.output.push(value);
            }

            7 => { // i64
                let value = Immediate::I64(ptr.get_data::<i64>());
                self.output.push(value);
            }

            8 => { // f32
                let value = Immediate::F32(ptr.get_data::<f32>());
                self.output.push(value);
            }

            9 => { // f64
                let value = Immediate::F64(ptr.get_data::<f64>());
                self.output.push(value);
            }

            10 => { // bool
                let value = Immediate::BOOL(ptr.get_data::<bool>());
                self.output.push(value);
            }

            _ => {}
        }
    }

    /// args: address (u64), type_of_index, index, type_of_value,
    /// 
    /// loads value from address at heap and sets it to output at index
    fn loadi(&mut self) {
        let address = self.get_address();
        let ptr = self.heap.get(address);
        let index = self.get_index();
        self.ip += 1;
        match self.bytecode[self.ip] {
            0 => { // u8
                let value = Immediate::U8(ptr.get_data::<u8>());
                self.output.set(index, value);
            }

            1 => { // u16
                let value = Immediate::U16(ptr.get_data::<u16>());
                self.output.set(index, value);
            }

            2 => { // u32
                let value = Immediate::U32(ptr.get_data::<u32>());
                self.output.set(index, value);
            }

            3 => { // u64
                let value = Immediate::U64(ptr.get_data::<u64>());
                self.output.set(index, value);
            }

            4 => { // i8
                let value = Immediate::I8(ptr.get_data::<i8>());
                self.output.set(index, value);
            }

            5 => { // i16
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.output.set(index, value);
            }

            6 => { // i32
                let value = Immediate::I16(ptr.get_data::<i16>());
                self.output.set(index, value);
            }

            7 => { // i64
                let value = Immediate::I64(ptr.get_data::<i64>());
                self.output.set(index, value);
            }

            8 => { // f32
                let value = Immediate::F32(ptr.get_data::<f32>());
                self.output.set(index, value);
            }

            9 => { // f64
                let value = Immediate::F64(ptr.get_data::<f64>());
                self.output.set(index, value);
            }

            10 => { // bool
                let value = Immediate::BOOL(ptr.get_data::<bool>());
                self.output.set(index, value);
            }

            _ => {}
        }
    }

    /// pops values from input and compares them, pushes result to output
    fn less(&mut self) {
        let v1 = self.input.pop();
        let v2 = self.input.pop();
        let v = v1 < v2;
        self.output.push(Immediate::BOOL(v));
    }

    /// pops values from input and compares them, pushes result to output
    fn great(&mut self) {
        let v1 = self.input.pop();
        let v2 = self.input.pop();
        let v = v1 > v2;
        self.output.push(Immediate::BOOL(v));
    }

    /// pops values from input and compares them, pushes result to output
    fn eq(&mut self) {
        let v1 = self.input.pop();
        let v2 = self.input.pop();
        let v = v1 == v2;
        self.output.push(Immediate::BOOL(v));
    }
 
    /// pops number from input and jumps to its value  
    fn jmp(&mut self) {
        let v = self.input.pop();
        let mut index = 0;
        match v {
            Immediate::U8(v) => { index = v as usize; }
            Immediate::U16(v) => { index = v as usize; }
            Immediate::U32(v) => { index = v as usize; }
            Immediate::U64(v) => { index = v as usize; }
            Immediate::I8(v) => { index = v as usize; }
            Immediate::I16(v) => { index = v as usize; }
            Immediate::I32(v) => { index = v as usize; }
            Immediate::I64(v) => { index = v as usize; }
            Immediate::F32(v) => { index = v as usize; }
            Immediate::F64(v) => { index = v as usize; }
            Immediate::BOOL(v) => { index = v as usize; }
            Immediate::ADDRESS(v) => { index = v as usize; }

            _ => { index = self.ip + 1; }
        }

        self.ip = index;
        self.jmp = true;
    }

    /// pops numbers from input, adds two numbers and pushes result to output
    fn add(&mut self) {
        let num1 = self.input.pop();
        let num2 = self.input.pop();
        let num = num1 + num2;
        self.output.push(num);
    }

    /// pops numbers from input, substracts two numbers and pushes result to output
    fn sub(&mut self) {
        let num1 = self.input.pop();
        let num2 = self.input.pop();
        let num = num1 - num2;
        self.output.push(num);
    }

    /// pops numbers from input, multiplies two numbers and pushes result to output
    fn mul(&mut self) {
        let num1 = self.input.pop();
        let num2 = self.input.pop();
        let num = num1 * num2;
        self.output.push(num);
    }

    /// pops numbers from input, divides two numbers and pushes result to output
    fn div(&mut self) {
        let num1 = self.input.pop();
        let num2 = self.input.pop();
        let num = num1 / num2;
        self.output.push(num);
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
