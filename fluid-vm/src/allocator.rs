use std::alloc::{ alloc, dealloc, Layout, };
use std::mem;

pub type Ptr = *mut u8;

pub trait Allocation {
    fn allocate(size: usize) -> Self;
    fn allocate_fill<T>(data: T) -> Self;
    fn set_data<T>(&self, data: T);
    fn get_data<T: Copy>(&self) -> T;
    fn deallocate(&self, size: usize);
}

impl Allocation for Ptr {
    fn allocate(size: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);
            let ptr = alloc(layout);
            if ptr.is_null() {
                panic!("Veles-vm, allocator: pointer is null");
            }
            ptr
        }
    }

    fn allocate_fill<T>(data: T) -> Self {
        let size = mem::size_of::<T>();
        let ptr = Ptr::allocate(size);
        ptr.set_data(data);
        ptr
    }

    fn set_data<T>(&self, data: T) {
        unsafe {
            *(*self as *mut T) = data;
        }   
    }

    fn get_data<T: Copy>(&self) -> T {
        unsafe {
            *(*self as *mut T) 
        }
    }

    fn deallocate(&self, size: usize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);
            dealloc(*self, layout);
        }
    }
}