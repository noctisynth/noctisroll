//! Dice type implementations

mod composite;
mod exploding;
mod fate;
mod pool;
mod standard;

pub use composite::*;
pub use exploding::*;
pub use fate::*;
pub use pool::*;
pub use standard::*;

/// Convenience function to create a standard dice
pub fn d(count: u32, sides: u32) -> StandardDice {
    StandardDice::new(count, sides)
}

/// Convenience function to create a fate dice
pub fn f(count: u32) -> FateDice {
    FateDice::new(count)
}

/// Convenience function to create an exploding dice
pub fn x(count: u32, sides: u32, threshold: u32) -> ExplodingDice {
    ExplodingDice::new(count, sides, threshold)
}
