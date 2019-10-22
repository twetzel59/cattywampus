//! User values and their types
//!
//! This module declares the ``Value`` type, which
//! is used to represent each distinct element on
//! the calculator's stack.

use std::fmt;

/// Stores a type without a concrete value
///
/// For now, these correlate directly to the
/// ``Value`` constructors, but in the future,
/// there may be more than one ``Type`` per
/// ``Value``. For example, more specific
/// types could be defined that mandate that
/// an integer is zonzero or a that a real is
/// in a certain range.
///
/// Types are unit for now, but may eventually
/// themselves be parameterized to allow for
/// generics.
#[derive(Debug, PartialEq)]
pub enum Type {
    /// The most general ``Int32``
    AnyInt32,

    /// The most general ``Float64``
    AnyFloat64,
}

/// Represents a single value on the calculator stack
#[derive(Debug, PartialEq)]
pub enum Value {
    /// A 32-bit signed integer
    Int32(i32),

    /// A 64-bit floating point number
    Float64(f64),
}

impl Value {
    /// Returns ``true`` if the ``Value`` can be treated
    /// as the specified ``Type``.
    pub fn matches(&self, candiate: &Type) -> bool {
        use Type::*;
        use Value::*;

        match self {
            Int32(_) => *candiate == AnyInt32,
            Float64(_) => *candiate == AnyFloat64,
        }
    }
    
    /// Returns a string representation of the ``Value``'s
    /// type.
    pub fn type_str(&self) -> &str {
        match self {
            Value::Int32(_) => "Int32",
            Value::Float64(_) => "Float64",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int32(x) => write!(f, "{}", x),
            Value::Float64(x) => write!(f, "{}", x),
        }
    }
}
