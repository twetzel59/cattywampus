//! Parsing of raw string input
//!
//! This module provides the parser, which processes one line
//! of string input at a time, but could be expanded to process
//! files.

use super::{
    functions::{BuiltinFun, INTRINSIC_FNS},
    value::Value,
};
use lazy_static::lazy_static;
use regex::RegexSet;
use std::str::FromStr;

#[rustfmt::skip]
lazy_static! {
    // The patterns that represent different value literals
    static ref VALUE_LITERALS: RegexSet = RegexSet::new(&[
        r"^-?((\d+\.\d*)|(\d*\.\d+))f$", // Float32 literal
        r"^-?((\d+\.\d*)|(\d*\.\d+))$", // Float64 literal
        r"^-?\d+$",                     // Int32 literal
    ]).unwrap();
}

// The following are indices into the lazy static RegexSet above.
const FLOAT32_LITERAL_IDX: usize = 0;
const FLOAT64_LITERAL_IDX: usize = 1;
const INT32_LITERAL_IDX: usize = 2;

/// The result of parsing a token
#[derive(Debug, PartialEq)]
pub enum ParsedToken {
    /// Results from parsing a ``Value`` literal
    Literal(Value),

    /// Results from parsing a builtin function
    Intrinsic(&'static BuiltinFun),

    /// Results from a failed parse
    BadToken,
}

/// Parse a single line of input, returning
/// both the string tokens and their parsed
/// values, if they were valid.
///
/// It is assumed that the input does not contain any newlines.
pub fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = (&str, ParsedToken)> + 'a {
    split_tokens(line).map(|token| (token, analyze_token(token)))
}

fn split_tokens(line: &str) -> impl Iterator<Item = &str> {
    line.split(char::is_whitespace).filter(|s| !s.is_empty())
}

fn analyze_token(token: &str) -> ParsedToken {
    let matches = VALUE_LITERALS.matches(token);

    if matches.iter().any(|idx| idx == FLOAT32_LITERAL_IDX) {
        parse_float32(token)
    } else if matches.iter().any(|idx| idx == FLOAT64_LITERAL_IDX) {
        parse_float64(token)
    } else if matches.iter().any(|idx| idx == INT32_LITERAL_IDX) {
        parse_int32(token)
    } else {
        match INTRINSIC_FNS.get(token) {
            Some(fun) => ParsedToken::Intrinsic(fun),
            None => ParsedToken::BadToken,
        }
    }
}

fn parse_float32(token: &str) -> ParsedToken {
    let len = token.len();
    ParsedToken::Literal(Value::Float32(f32::from_str(&token[..(len - 1)]).unwrap()))
}

fn parse_float64(token: &str) -> ParsedToken {
    ParsedToken::Literal(Value::Float64(f64::from_str(token).unwrap()))
}

fn parse_int32(token: &str) -> ParsedToken {
    ParsedToken::Literal(Value::Int32(i32::from_str(token).unwrap()))
}
