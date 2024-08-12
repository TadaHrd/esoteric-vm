//! Stack overflow.

use std::{error::Error, fmt};

/// Stack overflow.
///
/// This type is meant to be used in `Result::Err` variants.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackOverflow;

impl fmt::Display for StackOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Stack overflow")
    }
}

impl Error for StackOverflow {}
