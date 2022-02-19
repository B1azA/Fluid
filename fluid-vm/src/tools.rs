use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div};

pub type Address = usize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Immediate {
    NONE(),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    BOOL(bool),
    ADDRESS(Address),
}

impl Add for Immediate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let val1 = self;
        let val2 = other;
        let mut v = Immediate::NONE();
        match (val1, val2) {
            (Immediate::U8(v1), Immediate::U8(v2)) => { v = Immediate::U8(v1 + v2); },
            (Immediate::U16(v1), Immediate::U16(v2)) => { v = Immediate::U16(v1 + v2); },
            (Immediate::U32(v1), Immediate::U32(v2)) => { v = Immediate::U32(v1 + v2); },
            (Immediate::U64(v1), Immediate::U64(v2)) => { v = Immediate::U64(v1 + v2); },
            (Immediate::I8(v1), Immediate::I8(v2)) => { v = Immediate::I8(v1 + v2); },
            (Immediate::I16(v1), Immediate::I16(v2)) => { v = Immediate::I16(v1 + v2); },
            (Immediate::I32(v1), Immediate::I32(v2)) => { v = Immediate::I32(v1 + v2); },
            (Immediate::I64(v1), Immediate::I64(v2)) => { v = Immediate::I64(v1 + v2); },
            (Immediate::F32(v1), Immediate::F32(v2)) => { v = Immediate::F32(v1 + v2); },
            (Immediate::F64(v1), Immediate::F64(v2)) => { v = Immediate::F64(v1 + v2); },
            (Immediate::BOOL(v1), Immediate::BOOL(v2)) => { v = Immediate::BOOL((v1 as u8 + v2 as u8) == 1); },
            (Immediate::ADDRESS(v1), Immediate::ADDRESS(v2)) => { v = Immediate::ADDRESS(v1 + v2); },

            _ => { v = Immediate::NONE(); }
        }

        return v;
    }
}

impl Sub for Immediate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let val1 = self;
        let val2 = other;
        let mut v = Immediate::NONE();
        match (val1, val2) {
            (Immediate::U8(v1), Immediate::U8(v2)) => { v = Immediate::U8(v1 - v2); },
            (Immediate::U16(v1), Immediate::U16(v2)) => { v = Immediate::U16(v1 - v2); },
            (Immediate::U32(v1), Immediate::U32(v2)) => { v = Immediate::U32(v1 - v2); },
            (Immediate::U64(v1), Immediate::U64(v2)) => { v = Immediate::U64(v1 - v2); },
            (Immediate::I8(v1), Immediate::I8(v2)) => { v = Immediate::I8(v1 - v2); },
            (Immediate::I16(v1), Immediate::I16(v2)) => { v = Immediate::I16(v1 - v2); },
            (Immediate::I32(v1), Immediate::I32(v2)) => { v = Immediate::I32(v1 - v2); },
            (Immediate::I64(v1), Immediate::I64(v2)) => { v = Immediate::I64(v1 - v2); },
            (Immediate::F32(v1), Immediate::F32(v2)) => { v = Immediate::F32(v1 - v2); },
            (Immediate::F64(v1), Immediate::F64(v2)) => { v = Immediate::F64(v1 - v2); },
            (Immediate::BOOL(v1), Immediate::BOOL(v2)) => { v = Immediate::BOOL((v1 as u8 - v2 as u8) == 1); },
            (Immediate::ADDRESS(v1), Immediate::ADDRESS(v2)) => { v = Immediate::ADDRESS(v1 - v2); },

            _ => { v = Immediate::NONE(); }
        }

        return v;
    }
}

impl Mul for Immediate {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let val1 = self;
        let val2 = other;
        let mut v = Immediate::NONE();
        match (val1, val2) {
            (Immediate::U8(v1), Immediate::U8(v2)) => { v = Immediate::U8(v1 * v2); },
            (Immediate::U16(v1), Immediate::U16(v2)) => { v = Immediate::U16(v1 * v2); },
            (Immediate::U32(v1), Immediate::U32(v2)) => { v = Immediate::U32(v1 * v2); },
            (Immediate::U64(v1), Immediate::U64(v2)) => { v = Immediate::U64(v1 * v2); },
            (Immediate::I8(v1), Immediate::I8(v2)) => { v = Immediate::I8(v1 * v2); },
            (Immediate::I16(v1), Immediate::I16(v2)) => { v = Immediate::I16(v1 * v2); },
            (Immediate::I32(v1), Immediate::I32(v2)) => { v = Immediate::I32(v1 * v2); },
            (Immediate::I64(v1), Immediate::I64(v2)) => { v = Immediate::I64(v1 * v2); },
            (Immediate::F32(v1), Immediate::F32(v2)) => { v = Immediate::F32(v1 * v2); },
            (Immediate::F64(v1), Immediate::F64(v2)) => { v = Immediate::F64(v1 * v2); },
            (Immediate::BOOL(v1), Immediate::BOOL(v2)) => { v = Immediate::BOOL((v1 as u8 * v2 as u8) == 1); },
            (Immediate::ADDRESS(v1), Immediate::ADDRESS(v2)) => { v = Immediate::ADDRESS(v1 * v2); },

            _ => { v = Immediate::NONE(); }
        }

        return v;
    }
}

impl Div for Immediate {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let val1 = self;
        let val2 = other;
        let mut v = Immediate::NONE();
        match (val1, val2) {
            (Immediate::U8(v1), Immediate::U8(v2)) => { v = Immediate::U8(v1 / v2); },
            (Immediate::U16(v1), Immediate::U16(v2)) => { v = Immediate::U16(v1 / v2); },
            (Immediate::U32(v1), Immediate::U32(v2)) => { v = Immediate::U32(v1 / v2); },
            (Immediate::U64(v1), Immediate::U64(v2)) => { v = Immediate::U64(v1 / v2); },
            (Immediate::I8(v1), Immediate::I8(v2)) => { v = Immediate::I8(v1 / v2); },
            (Immediate::I16(v1), Immediate::I16(v2)) => { v = Immediate::I16(v1 / v2); },
            (Immediate::I32(v1), Immediate::I32(v2)) => { v = Immediate::I32(v1 / v2); },
            (Immediate::I64(v1), Immediate::I64(v2)) => { v = Immediate::I64(v1 / v2); },
            (Immediate::F32(v1), Immediate::F32(v2)) => { v = Immediate::F32(v1 / v2); },
            (Immediate::F64(v1), Immediate::F64(v2)) => { v = Immediate::F64(v1 / v2); },
            (Immediate::BOOL(v1), Immediate::BOOL(v2)) => { v = Immediate::BOOL((v1 as u8 / v2 as u8) == 1); },
            (Immediate::ADDRESS(v1), Immediate::ADDRESS(v2)) => { v = Immediate::ADDRESS(v1 / v2); },

            _ => { v = Immediate::NONE(); }
        }

        return v;
    }
}