use crate::allocator::{ Ptr, Allocation };
use crate::tools::*;

pub struct Heap {
    data: Vec<Ptr>,
    empty: Vec<usize>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            empty: Vec::new(),
        }
    }

    /// adds pointer to heap
    pub fn add(&mut self, ptr: Ptr) -> Address {
        let len = self.empty.len();
        if len == 0 {
            self.data.push(ptr);
            self.data.len() - 1
        } else {
            let address = self.empty.pop().unwrap();
            self.data[address] = ptr;
            address
        }
    }

    /// removes pointer from heap but won't deallocate it
    pub fn remove(&mut self, index: Address) -> Ptr {
        self.empty.push(index);
        self.data[index]
    } 

    /// deallocates and deletes pointer from heap
    pub fn delete(&mut self, index: Address, size: usize) {
        self.empty.push(index);
        self.data[index].deallocate(size);
    }
}