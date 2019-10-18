//! Parsing of raw string input
//!
//! This module provides the parser, which processes one line
//! of string input at a time, but could be expanded to process
//! files.

pub fn find_tokens(line: &str) -> Vec<&str> {
    line.split(char::is_whitespace).filter(|s| !s.is_empty()).collect()
}
