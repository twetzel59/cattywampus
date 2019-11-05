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
        fns.insert("inc", BuiltinFun::new("inc", (&[AnyInt32], &[AnyInt32]), inc_impl));
        fns.insert("dec", BuiltinFun::new("dec", (&[AnyInt32], &[AnyInt32]), dec_impl));

        // Algebraic
        fns.insert("recip", BuiltinFun::new("recip", (&[AnyFloat64], &[AnyFloat64]), recip_impl));
        fns.insert("sqrt", BuiltinFun::new("sqrt", (&[AnyFloat64], &[AnyFloat64]), sqrt_impl));
        fns.insert("cbrt", BuiltinFun::new("cbrt", (&[AnyFloat64], &[AnyFloat64]), cbrt_impl));

        // Exponential & Logarithmic
        fns.insert("exp", BuiltinFun::new("exp", (&[AnyFloat64], &[AnyFloat64]), exp_impl));
        fns.insert("ln",  BuiltinFun::new("ln",  (&[AnyFloat64], &[AnyFloat64]), ln_impl));

        // Trigonometry
        fns.insert("sin", BuiltinFun::new("sin", (&[AnyFloat64], &[AnyFloat64]), sin_impl));
        fns.insert("cos", BuiltinFun::new("cos", (&[AnyFloat64], &[AnyFloat64]), cos_impl));
        fns.insert("tan", BuiltinFun::new("tan", (&[AnyFloat64], &[AnyFloat64]), tan_impl));

        // Trigonometry - reciprocals
        fns.insert("csc", BuiltinFun::new("csc", (&[AnyFloat64], &[AnyFloat64]), csc_impl));
        fns.insert("sec", BuiltinFun::new("sec", (&[AnyFloat64], &[AnyFloat64]), sec_impl));
        fns.insert("cot", BuiltinFun::new("cot", (&[AnyFloat64], &[AnyFloat64]), cot_impl));

        // Trigonometry - principal inverses
        fns.insert("arcsin", BuiltinFun::new("arcsin", (&[AnyFloat64], &[AnyFloat64]), arcsin_impl));
        fns.insert("arccos", BuiltinFun::new("arccos", (&[AnyFloat64], &[AnyFloat64]), arccos_impl));
        fns.insert("arctan", BuiltinFun::new("arctan", (&[AnyFloat64], &[AnyFloat64]), arctan_impl));

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
        Scalar(Value::Int32(match args {
            [Value::Int32(x)] => x + 1,
            _ => unreachable!(),
        }))
    }
    
    pub fn dec_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Int32(match args {
            [Value::Int32(x)] => x - 1,
            _ => unreachable!(),
        }))
    }

    // Algebraic
    pub fn recip_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.recip(),
            _ => unreachable!(),
        }))
    }
    
    pub fn sqrt_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.sqrt(),
            _ => unreachable!(),
        }))
    }
    
    pub fn cbrt_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.cbrt(),
            _ => unreachable!(),
        }))
    }

    // Exponential & Logarithmic
    pub fn exp_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.exp(),
            _ => unreachable!(),
        }))
    }
    
    pub fn ln_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.ln(),
            _ => unreachable!(),
        }))
    }

    // Trigonometry
    pub fn sin_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.sin(),
            _ => unreachable!(),
        }))
    }

    pub fn cos_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.cos(),
            _ => unreachable!(),
        }))
    }

    pub fn tan_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.tan(),
            _ => unreachable!(),
        }))
    }

    // Trigonometry - reciprocals
    pub fn csc_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.sin().recip(),
            _ => unreachable!(),
        }))
    }

    pub fn sec_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.cos().recip(),
            _ => unreachable!(),
        }))
    }

    pub fn cot_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.tan().recip(),
            _ => unreachable!(),
        }))
    }
    
    // Trigonometry - principal inverses
    pub fn arcsin_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.asin(),
            _ => unreachable!(),
        }))
    }

    pub fn arccos_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.acos(),
            _ => unreachable!(),
        }))
    }

    pub fn arctan_impl(args: &[Value]) -> FunctionResult {
        Scalar(Value::Float64(match args {
            [Value::Float64(x)] => x.atan(),
            _ => unreachable!(),
        }))
    }
}
