//! Rudimentary type-checking.
//!
//! cattywampus uses a dynamic, but strong, type system.
//! This module provides the dynamic checking facility.

use super::{
    functions::{Function, FunctionResult},
    stack::Stack,
};

/// Represents various kinds of type errors.
#[derive(Debug)]
pub enum TypeError {
    /// Arises when the stack is too small to call
    /// the function.
    WrongArity,

    /// Arises when the ``Values`` on the stack are
    /// of the wrong type.
    TypeMismatch,

    /// Arises when the function returns a ``Value``
    /// of an incorrect type.
    BrokenCallee,
}

/// Apply the function to the stack, if possible.
///
/// Returns ``false`` on failure.
pub fn checked_apply<'a>(fun: &Function<'a>, stack: &mut Stack) -> Result<(), TypeError> {
    // For development purposes, functions can only return
    // one scalar, and must do so.
    // This will change in the future.
    assert!(fun.signiture.1.len() == 1);

    // First, check the arity of the function.
    // It needs to be less than or equal to the stack height.
    let arity = fun.signiture.0.len();
    let arity_ok = arity <= stack.height();

    if !arity_ok {
        return Err(TypeError::WrongArity);
    }

    // Now, check the types of the function arguments
    // against the values on the stack.
    for (idx, arg_type) in fun.signiture.0.iter().enumerate() {
        //println!("{} {:?} {:?}", idx, stack.peek_n(arity - idx - 1), arg_type);

        if !stack.peek_n(arity - idx - 1).matches(arg_type) {
            return Err(TypeError::TypeMismatch);
        }
    }

    // Finally, if we're here, the argument types match and we can apply
    // the function.

    // Borrow the arguments.
    let args_slice = stack.slice_n(arity - 1);
    //println!("{:?}", args_slice);

    // Next, run the function.
    let fun_result = (fun.implementation)(args_slice);

    // Make sure the function returned what it said it would.
    let val = match fun_result {
        FunctionResult::Scalar(val) => {
            if !val.matches(&fun.signiture.1[0]) {
                return Err(TypeError::BrokenCallee);
            }

            val
        }
    };

    // Now, remove the arguments from the stack.
    stack.chop_n(arity);

    // ...and push the result to the stack.
    stack.push(val);

    return Ok(());
}
