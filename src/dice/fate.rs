//! FATE/Fudge dice implementation

use crate::core::{Dice, DiceContext, DieRoll, RollResult};
use crate::error::{DiceError, DiceResult};
use rand::RngExt;
use std::fmt;

/// FATE/Fudge dice (dF)
/// Each die has faces: -1, 0, +1
#[derive(Debug, Clone, PartialEq)]
pub struct FateDice {
    /// Number of dice to roll
    pub count: u32,
}

impl FateDice {
    /// Create new FATE dice
    pub fn new(count: u32) -> Self {
        Self { count }
    }

    /// Validate the dice configuration
    pub fn validate(&self) -> DiceResult<()> {
        if self.count == 0 {
            return Err(DiceError::InvalidDice("Must roll at least 1 die".into()));
        }
        Ok(())
    }
}

impl Dice for FateDice {
    fn roll(&self) -> RollResult {
        self.validate().unwrap();

        let mut ctx = DiceContext::new();
        let mut rolls = Vec::with_capacity(self.count as usize);
        let mut total = 0i64;

        for _ in 0..self.count {
            // FATE dice have 3 faces: -1, 0, +1
            // We'll roll a d3 and map: 1 -> -1, 2 -> 0, 3 -> +1
            let roll_value = ctx.rng.random_range(1..=3);
            let fate_value = match roll_value {
                1 => -1,
                2 => 0,
                3 => 1,
                _ => 0, // Should never happen
            };

            let roll = DieRoll::new(roll_value, 3);
            rolls.push(roll);
            total += fate_value as i64;
        }

        let description = if self.count == 4 {
            "4dF".to_string()
        } else {
            format!("{}dF", self.count)
        };

        RollResult::new(rolls, total, description)
    }

    fn describe(&self) -> String {
        if self.count == 4 {
            "4dF (standard FATE dice)".to_string()
        } else {
            format!("{}dF (FATE dice)", self.count)
        }
    }

    fn expected_value(&self) -> f64 {
        // Expected value per die: (-1 + 0 + 1) / 3 = 0
        0.0
    }

    fn min_value(&self) -> i64 {
        -(self.count as i64)
    }

    fn max_value(&self) -> i64 {
        self.count as i64
    }
}

impl fmt::Display for FateDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
