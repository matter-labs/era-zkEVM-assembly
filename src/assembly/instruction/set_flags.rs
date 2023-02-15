//!
//! Condition to apply the operation
//!

use super::*;
use crate::InstructionReadError;

///
/// The control flow jump instruction flag.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetFlags(pub bool);

impl std::default::Default for SetFlags {
    fn default() -> Self {
        SetFlags(false)
    }
}

impl SetFlags {
    pub const ALL_CANONICAL_MODIFIERS: [&'static str; 1] = ["set_flags"];

    pub fn from_modifier(modifier: &str) -> Result<Self, InstructionReadError> {
        match modifier {
            "set_flags" => Ok(SetFlags(true)),
            _ => Err(InstructionReadError::UnknownArgument(modifier.to_owned())),
        }
    }
}
