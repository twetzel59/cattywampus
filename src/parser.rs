//! Parsing of raw string input
//!
//! This module provides the parser, which processes one line
//! of string input at a time, but could be expanded to process
//! files.

use super::value::Value;
use lazy_static::lazy_static;
use regex::RegexSet;
use std::str::FromStr;

// The patterns that represent different value literals
#[rustfmt::skip]
lazy_static! {
    static ref VALUE_LITERALS: RegexSet = RegexSet::new(&[
        r"^-?((\d+\.\d*)|(\d*\.\d+))$", // Float64 literal
        r"^-?\d+$",                     // Int32 literal
    ]).unwrap();
}

// The following are indices into the lazy static RegexSet above.
const FLOAT64_LITERAL_IDX: usize = 0;
const INT32_LITERAL_IDX: usize = 1;

/// The result of parsing a token
#[derive(Debug)]
pub enum ParsedToken {
    /// Results from parsing a ``Value`` literal
    Literal(Value),

    /// Results from parsing a builtin function
    Intrinsic,

    /// Results from a failed parse
    BadToken,
}

/// Parse a single line of input
///
/// It is assumed that the input does not contain any newlines.
pub fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = ParsedToken> + 'a {
    split_tokens(line).map(analyze_token)
}

fn split_tokens(line: &str) -> impl Iterator<Item = &str> {
    line.split(char::is_whitespace).filter(|s| !s.is_empty())
}

fn analyze_token(token: &str) -> ParsedToken {
    let matches = VALUE_LITERALS.matches(token);

    if matches.iter().any(|idx| idx == FLOAT64_LITERAL_IDX) {
        parse_float64(token)
    } else if matches.iter().any(|idx| idx == INT32_LITERAL_IDX) {
        parse_int32(token)
    } else {
        ParsedToken::BadToken
    }
}

fn parse_float64(token: &str) -> ParsedToken {
    ParsedToken::Literal(Value::Float64(f64::from_str(token).unwrap()))
}

fn parse_int32(token: &str) -> ParsedToken {
    ParsedToken::Literal(Value::Int32(i32::from_str(token).unwrap()))
}
