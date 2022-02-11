use crate::tools::*;

pub struct Buffer {
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
        Immediate::None()
    }

    pub fn set(&mut self, index: usize, value: Immediate) {
        if self.data.len() > index {
            self.data[index] = value;
        } else {
            self.data.resize(index, Immediate::None());
            self.data[index] = value;
        }
    }
}