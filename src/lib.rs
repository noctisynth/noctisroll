//! # NoctisRoll
//!
//! A modern, modular TRPG dice rolling system implementing the OneDice standard.
//!
//! ## Features
//!
//! - **Modular design**: Each dice type is implemented as a separate module
//! - **OneDice standard**: Full implementation of the OneDice specification
//! - **Type safety**: Strongly typed API with compile-time guarantees
//! - **Extensible**: Easy to add new dice types and operations
//! - **Performance**: Optimized for both single rolls and batch operations
//!
//! ## Quick Start
//!
//! ```rust
//! use noctisroll::prelude::*;
//!
//! // Roll 2d20 with advantage (keep highest)
//! let dice = StandardDice::new(2, 20).keep_highest(1);
//! let result = dice.roll();
//! println!("Roll: {}", result);
//!
//! // Parse and evaluate a dice expression
//! #[cfg(feature = "parser")]
//! {
//!     let expr = "2d20kh1 + 5";
//!     let result = noctisroll::eval(expr).unwrap();
//!     println!("Result: {}", result);
//! }
//! ```

pub mod core;
pub mod dice;
pub mod error;
pub mod utils;

#[cfg(feature = "parser")]
pub mod parser;

#[cfg(feature = "tool-call")]
pub mod tool;

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::core::*;
    pub use crate::dice::*;
    pub use crate::error::*;

    #[cfg(feature = "tool-call")]
    pub use crate::tool::*;

    #[cfg(feature = "parser")]
    pub use crate::parser::*;
}

/// Evaluate a dice expression string
#[cfg(feature = "parser")]
pub fn eval(expr: &str) -> Result<crate::core::RollResult, crate::error::DiceError> {
    use parser::Parser;
    Parser::new().eval(expr)
}

/// Roll a simple dice expression
pub fn roll(count: u32, sides: u32) -> crate::core::RollResult {
    use crate::core::Dice;
    dice::StandardDice::new(count, sides).roll()
}
