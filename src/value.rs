//! User values and their types
//!
//! This module declares the ``Value`` type, which
//! is used to represent each distinct element on
//! the calculator's stack.

use std::fmt;

/// Represents a single value on the calculator stack
#[derive(Debug)]
pub enum Value {
    Integer32(i32),
    Float32(f32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer32(x) => write!(f, "{}", x),
            Value::Float32(x) => write!(f, "{}", x),
        }
    }
}
