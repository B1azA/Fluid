use std::fmt::Debug;

use crate::tools::*;

pub struct Buffer<T> 
where T: Debug
{
    data: Vec<Immediate<T>>,
}

impl<T> Buffer<T> 
where T: Clone, T: Copy, T: Debug
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Immediate<T>) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Immediate<T> {
        self.data.pop().unwrap()
    }

    pub fn get(&self, index: usize) -> Immediate<T> {
        if self.data.len() > index {
            return self.data[index];
        }
        Immediate::NONE()
    }

    pub fn set(&mut self, index: usize, value: Immediate<T>) {
        if self.data.len() > index {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, Immediate::NONE());
            self.data[index] = value;
        }
    }

    pub fn clear(&mut self) {
        self.data = Vec::new();
    }

    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }
}