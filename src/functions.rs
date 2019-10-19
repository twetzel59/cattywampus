//! Provides the functions available in the calculator.
//!
//! This module contains builtin functions and their
//! implementations, but could eventually support user-
//! defined lambda closures.

use lazy_static::lazy_static;
use super::value::{Type, Value};
use std::{
    collections::HashMap,
    fmt,
};

lazy_static! {
    /// The intrinsic built-in functions supported in the calculator.
    pub static ref INTRINSIC_FNS: HashMap<&'static str, BuiltinFun> = {
        use Type::*;
        use builtins::*;
        
        let mut fns = HashMap::new();
        
        fns.insert("sin", BuiltinFun::new("sine", (&[AnyFloat64], &[AnyFloat64]), sin_impl));
        
        fns
    };
}

/// The type signiture for a function
///
/// The zeroth element is the inputs, or arguments,
/// of the function. There can be zero or more inputs.
///
/// The first element is the outputs, of which there
/// can be zero or more.
pub type Signiture<'a> = (&'a [Type], &'a [Type]);

/// The Rust type that all calculator function
/// implementations return. It is optimized for
/// functions that return single values.
pub enum FunctionResult {
    Scalar(Value),
    //List(Vec<Value>),
}

/// The type of a function's Rust implementation.
pub type Implementation = fn(&[Value]) -> FunctionResult;

/// The type of a built-in function
///
/// The ``Function`` knows the number and type of
/// its arguments, if it takes any. It also states
/// its return type, which will be enforced.
pub struct Function<'a> {
    pub name: String,
    pub signiture: Signiture<'a>,
    pub implementation: Implementation,
}

impl<'a> Function<'a> {
    /// Create a new function with the provided
    /// human-readable name, type signiture, and
    /// backing implmentation.
    pub fn new<S>(name: S, signiture: Signiture<'a>, implementation: Implementation) -> Function<'a>
        where S: Into<String>
    {
        Function {
            name: name.into(),
            signiture,
            implementation,
        }
    }
}

impl<'a> fmt::Debug for Function<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {:?} -> {:?}", self.name, self.signiture.0, self.signiture.1)
    }
}

// Note: implementation doesn't check pointers to implementations,
// because unequal pointers don't mean the function isn't the same
// mathematically.
//
// Functions with the same name and signiture are assumed to be equal.
impl<'a> PartialEq for Function<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.signiture == other.signiture
    }
}

/// The type of an intrinic function - that is,
/// one that is declared in Rust code and has a
/// signiture of the ``'static`` lifetime.
pub type BuiltinFun = Function<'static>;

mod builtins {
    use super::FunctionResult::{self, *};
    use crate::value::Value;
    
    pub fn sin_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.sin(),
            _ => unreachable!(),
        }))
    }
}
