//! Utilities for parsing Python docstrings.

#[allow(unused_imports)]
use alloc::{borrow::ToOwned, boxed::Box, format, string::{String, ToString}, vec, vec::Vec};

/// Extract the leading whitespace from a line of text within a Python docstring.
pub fn leading_space(line: &str) -> &str {
    line.find(|char: char| !char.is_whitespace())
        .map_or(line, |index| &line[..index])
}

/// Replace any non-whitespace characters from an indentation string within a Python docstring.
pub fn clean_space(indentation: &str) -> String {
    indentation
        .chars()
        .map(|char| if char.is_whitespace() { char } else { ' ' })
        .collect()
}
