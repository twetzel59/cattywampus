//! Provides the functions available in the calculator.
//!
//! This module contains builtin functions and their
//! implementations, but could eventually support user-
//! defined lambda closures.

use super::value::{Type, Value};
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {
    /// The intrinsic built-in functions supported in the calculator.
    pub static ref INTRINSIC_FNS: HashMap<&'static str, BuiltinFun> = {
        use Type::*;
        use builtins::*;

        let mut fns = HashMap::new();

        // Integer operations
        fns.insert("inc", BuiltinFun::new("inc", (&[Int32], &[Int32]), inc_impl));
        fns.insert("dec", BuiltinFun::new("dec", (&[Int32], &[Int32]), dec_impl));

        // Algebraic
        fns.insert("recip", BuiltinFun::new("recip", (&[Fractional], &[Fractional]), recip_impl));
        fns.insert("sqrt", BuiltinFun::new("sqrt", (&[Fractional], &[Fractional]), sqrt_impl));
        fns.insert("cbrt", BuiltinFun::new("cbrt", (&[Fractional], &[Fractional]), cbrt_impl));

        // Exponential & Logarithmic
        fns.insert("exp", BuiltinFun::new("exp", (&[Fractional], &[Fractional]), exp_impl));
        fns.insert("ln",  BuiltinFun::new("ln",  (&[Fractional], &[Fractional]), ln_impl));

        // Trigonometry
        fns.insert("sin", BuiltinFun::new("sin", (&[Fractional], &[Fractional]), sin_impl));
        fns.insert("cos", BuiltinFun::new("cos", (&[Fractional], &[Fractional]), cos_impl));
        fns.insert("tan", BuiltinFun::new("tan", (&[Fractional], &[Fractional]), tan_impl));

        // Trigonometry - reciprocals
        fns.insert("csc", BuiltinFun::new("csc", (&[Fractional], &[Fractional]), csc_impl));
        fns.insert("sec", BuiltinFun::new("sec", (&[Fractional], &[Fractional]), sec_impl));
        fns.insert("cot", BuiltinFun::new("cot", (&[Fractional], &[Fractional]), cot_impl));

        // Trigonometry - principal inverses
        fns.insert("asin", BuiltinFun::new("asin", (&[Fractional], &[Fractional]), asin_impl));
        fns.insert("acos", BuiltinFun::new("acos", (&[Fractional], &[Fractional]), acos_impl));
        fns.insert("atan", BuiltinFun::new("atan", (&[Fractional], &[Fractional]), atan_impl));

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
    where
        S: Into<String>,
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
        write!(
            f,
            "{} :: {:?} -> {:?}",
            self.name, self.signiture.0, self.signiture.1
        )
    }
}

// Note: implementation doesn't check pointers to implementations,
// because unequal pointers don't mean the function isn't the same
// mathematically.
//
// Functions with the same name and signiture are assumed to be equal.
impl<'a> PartialEq for Function<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signiture == other.signiture
    }
}

/// The type of an intrinic function - that is,
/// one that is declared in Rust code and has a
/// signiture of the ``'static`` lifetime.
pub type BuiltinFun = Function<'static>;

mod builtins {
    use super::FunctionResult::{self, *};
    use crate::value::Value;

    // Integer operations
    pub fn inc_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Int32(x)] => Scalar(Value::Int32(x + 1)),
            _ => unreachable!(),
        }
    }
    
    pub fn dec_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Int32(x)] => Scalar(Value::Int32(x - 1)),
            _ => unreachable!(),
        }
    }

    // Algebraic
    pub fn recip_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.recip())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.recip())),
            _ => unreachable!(),
        }
    }
    
    pub fn sqrt_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.sqrt())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.sqrt())),
            _ => unreachable!(),
        }
    }
    
    pub fn cbrt_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.cbrt())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.cbrt())),
            _ => unreachable!(),
        }
    }

    // Exponential & Logarithmic
    pub fn exp_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.exp())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.exp())),
            _ => unreachable!(),
        }
    }
    
    pub fn ln_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.ln())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.ln())),
            _ => unreachable!(),
        }
    }

    // Trigonometry
    pub fn sin_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.sin())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.sin())),
            _ => unreachable!(),
        }
    }

    pub fn cos_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.cos())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.cos())),
            _ => unreachable!(),
        }
    }

    pub fn tan_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.tan())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.tan())),
            _ => unreachable!(),
        }
    }

    // Trigonometry - reciprocals
    pub fn csc_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.sin().recip())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.sin().recip())),
            _ => unreachable!(),
        }
    }

    pub fn sec_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.cos().recip())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.cos().recip())),
            _ => unreachable!(),
        }
    }

    pub fn cot_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.tan().recip())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.tan().recip())),
            _ => unreachable!(),
        }
    }
    
    // Trigonometry - principal inverses
    pub fn asin_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.asin())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.asin())),
            _ => unreachable!(),
        }
    }

    pub fn acos_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.acos())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.acos())),
            _ => unreachable!(),
        }
    }

    pub fn atan_impl(args: &[Value]) -> FunctionResult {
        match args {
            [Value::Float32(x)] => Scalar(Value::Float32(x.atan())),
            [Value::Float64(x)] => Scalar(Value::Float64(x.atan())),
            _ => unreachable!(),
        }
    }
}
