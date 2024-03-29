//! The calculator's virtual stack
//!
//! This module provides the declaration for the data
//! structure for the calculator's emulated stack.

use super::value::Value;
use std::slice;

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

    /// Return the current height of the
    /// stack, or how many elements it has.
    pub fn height(&self) -> usize {
        self.elements.len()
    }

    /// Return the element ``n`` places from
    /// the top of the stack. Panics if the
    /// provided n runs off the stack.
    pub fn peek_n(&self, n: usize) -> &Value {
        let len = self.elements.len();
        &self.elements[len - n - 1]
    }

    /// Return a slice starting ``n`` places
    /// from the top of stack. Panics if the
    /// provided n runs off the stack.
    pub fn slice_n(&self, n: usize) -> &[Value] {
        let len = self.elements.len();
        &self.elements[(len - n - 1)..]
    }

    /// Remove and drop the last ``n``
    /// elements. Panics if the provided ``n``
    /// runs off the stack.
    pub fn chop_n(&mut self, n: usize) {
        let len = self.elements.len();
        // no ``- 1``, truncate is by count, not index.
        self.elements.truncate(len - n);
    }

    /// Reset the stack to completely empty,
    /// dropping all ``Value``s in the stack.
    pub fn clear(&mut self) {
        self.elements.clear();
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

    /// Returns an iterator over the items
    /// in the stack, *oldest* first.
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, Value> {
        (&self.elements).into_iter()
    }
}

impl<'a> IntoIterator for &'a Stack {
    type Item = &'a Value;
    type IntoIter = slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Default for Stack {
    fn default() -> Stack {
        Stack::new()
    }
}
