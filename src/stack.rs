//! The calculator's virtual stack
//!
//! This module provides the declaration for the data
//! structure for the calculator's emulated stack.

use super::value::Value;

/// Represents the virtual stack
///
/// This is a LIFO stack.
#[derive(Debug)]
pub struct Stack {
    elements: Vec<Value>,
}

impl Stack {
    /// Create a new, empty ``Stack``.
    pub fn new() -> Stack {
        Stack {
            elements: Vec::new(),
        }
    }

    /// Push a ``Value`` onto the stack,
    /// making it the top element.
    pub fn push(&mut self, val: Value) {
        self.elements.push(val);
    }

    /// Push the series of ``Values`` onto
    /// the stack, making them the highest
    /// elements.
    ///
    /// The last element in the provided
    /// collection will be the very top of
    /// the stack.
    pub fn extend(&mut self, values: Vec<Value>) {
        self.elements.extend(values);
    }

    /// Tries to pop a ``Value`` off the
    /// stack and return it.
    ///
    /// If the value exists, it is returned
    /// as ``Some(value)``.
    ///
    /// If the ``Stack`` is empty, ``None``
    /// is returned.
    pub fn pop(&mut self) -> Option<Value> {
        self.elements.pop()
    }
}

impl Default for Stack {
    fn default() -> Stack {
        Stack::new()
    }
}
