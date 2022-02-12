use std::fmt::Debug;

use crate::tools::*;

pub struct Buffer
{
    data: Vec<Immediate>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Immediate) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Immediate {
        self.data.pop().unwrap()
    }

    pub fn get(&self, index: usize) -> Immediate {
        if self.data.len() > index {
            return self.data[index];
        }
        Immediate::NONE()
    }

    pub fn set(&mut self, index: usize, value: Immediate) {
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