//! Error types for the dice rolling system

use thiserror::Error;

/// Main error type for dice operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DiceError {
    /// Invalid dice specification
    #[error("Invalid dice: {0}")]
    InvalidDice(String),

    /// Invalid parameter value
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Arithmetic error (overflow, division by zero, etc.)
    #[error("Arithmetic error: {0}")]
    ArithmeticError(String),

    /// Parse error for dice expressions
    #[cfg(feature = "parser")]
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// Invalid dice expression
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
}

/// Result type for dice operations
pub type DiceResult<T> = Result<T, DiceError>;

/// Conversion from `DiceError` to `rig::tool::ToolError`
#[cfg(feature = "tool-call")]
impl From<DiceError> for rig::tool::ToolError {
    fn from(err: DiceError) -> Self {
        rig::tool::ToolError::ToolCallError(err.into())
    }
}
