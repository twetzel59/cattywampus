//! User values and their types
//!
//! This module declares the ``Value`` type, which
//! is used to represent each distinct element on
//! the calculator's stack.

use std::fmt;

/// Represents a single value on the calculator stack
#[derive(Debug)]
pub enum Value {
    /// A 32-bit signed integer
    Int32(i32),
    
    /// A 64-bit floating point number
    Float64(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int32(x) => write!(f, "{}", x),
            Value::Float64(x) => write!(f, "{}", x),
        }
    }
}
