//! Dice pool implementations (infinite adding, double cross, etc.)

use crate::core::{Dice, DiceContext, RollResult};
use crate::error::{DiceError, DiceResult};
use std::fmt;

/// Infinite adding dice pool (a operator)
#[derive(Debug, Clone, PartialEq)]
pub struct InfiniteAddingPool {
    /// Initial number of dice
    pub initial_count: u32,
    /// Success threshold for adding more dice
    pub add_threshold: u32,
    /// Success threshold for counting successes
    pub success_threshold: u32,
    /// Number of sides on each die
    pub sides: u32,
    /// Maximum number of iterations (for safety)
    pub max_iterations: u32,
}

impl InfiniteAddingPool {
    /// Create a new infinite adding pool
    pub fn new(initial_count: u32, add_threshold: u32) -> Self {
        Self {
            initial_count,
            add_threshold,
            success_threshold: 8, // Default from OneDice
            sides: 10,            // Default from OneDice
            max_iterations: 100,
        }
    }

    /// Set success threshold
    pub fn with_success_threshold(mut self, threshold: u32) -> Self {
        self.success_threshold = threshold;
        self
    }

    /// Set number of sides
    pub fn with_sides(mut self, sides: u32) -> Self {
        self.sides = sides;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: u32) -> Self {
        self.max_iterations = max;
        self
    }

    /// Validate the pool configuration
    pub fn validate(&self) -> DiceResult<()> {
        if self.sides == 0 {
            return Err(DiceError::InvalidDice(
                "Dice must have at least 1 side".into(),
            ));
        }

        if self.add_threshold == 0 || self.add_threshold > self.sides {
            return Err(DiceError::InvalidParameter(format!(
                "Add threshold must be between 1 and {}",
                self.sides
            )));
        }

        if self.success_threshold == 0 || self.success_threshold > self.sides {
            return Err(DiceError::InvalidParameter(format!(
                "Success threshold must be between 1 and {}",
                self.sides
            )));
        }

        Ok(())
    }
}

impl Dice for InfiniteAddingPool {
    fn roll(&self) -> RollResult {
        self.validate().unwrap();

        let mut ctx = DiceContext::new();
        let mut all_rolls = Vec::new();
        let mut current_count = self.initial_count;
        let mut iteration = 0;

        // Roll through iterations
        while current_count > 0 && iteration < self.max_iterations {
            let rolls = ctx.roll_dice(current_count, self.sides);
            all_rolls.extend_from_slice(&rolls);

            // Count successes in CURRENT batch for next iteration
            current_count = rolls
                .iter()
                .filter(|r| r.value >= self.add_threshold)
                .count() as u32;

            iteration += 1;
        }

        // Count final successes
        let success_count = all_rolls
            .iter()
            .filter(|r| r.value >= self.success_threshold)
            .count() as u32;

        let failure_count = all_rolls.len() as u32 - success_count;
        let total = success_count as i64;

        let description = format!(
            "{}a{}k{}m{} = {} successes",
            self.initial_count,
            self.add_threshold,
            self.success_threshold,
            self.sides,
            success_count
        );

        RollResult::with_success(
            all_rolls,
            total,
            description,
            success_count > 0,
            success_count,
            failure_count,
        )
    }

    fn describe(&self) -> String {
        format!(
            "{}a{} (infinite adding pool: add on {}+, success on {}+, {} sides)",
            self.initial_count,
            self.add_threshold,
            self.add_threshold,
            self.success_threshold,
            self.sides
        )
    }

    fn expected_value(&self) -> f64 {
        // This is complex to calculate analytically
        // For now, return a rough estimate
        let prob_add = (self.sides - self.add_threshold + 1) as f64 / self.sides as f64;
        let prob_success = (self.sides - self.success_threshold + 1) as f64 / self.sides as f64;

        // Expected number of dice in geometric series
        let expected_dice = self.initial_count as f64 / (1.0 - prob_add);

        // Expected successes
        expected_dice * prob_success
    }

    fn min_value(&self) -> i64 {
        0
    }

    fn max_value(&self) -> i64 {
        // Theoretical maximum is infinite
        // Return a practical limit
        (self.initial_count * self.max_iterations) as i64
    }
}

impl fmt::Display for InfiniteAddingPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

/// Double cross adding pool (c operator)
#[derive(Debug, Clone, PartialEq)]
pub struct DoubleCrossPool {
    /// Initial number of dice
    pub initial_count: u32,
    /// Success threshold for adding more dice
    pub add_threshold: u32,
    /// Number of sides on each die
    pub sides: u32,
    /// Maximum number of iterations (for safety)
    pub max_iterations: u32,
}

impl DoubleCrossPool {
    /// Create a new double cross pool
    pub fn new(initial_count: u32, add_threshold: u32) -> Self {
        Self {
            initial_count,
            add_threshold,
            sides: 10, // Default from OneDice
            max_iterations: 100,
        }
    }

    /// Set number of sides
    pub fn with_sides(mut self, sides: u32) -> Self {
        self.sides = sides;
        self
    }

    /// Validate the pool configuration
    pub fn validate(&self) -> DiceResult<()> {
        if self.sides == 0 {
            return Err(DiceError::InvalidDice(
                "Dice must have at least 1 side".into(),
            ));
        }

        if self.add_threshold == 0 || self.add_threshold > self.sides {
            return Err(DiceError::InvalidParameter(format!(
                "Add threshold must be between 1 and {}",
                self.sides
            )));
        }

        Ok(())
    }
}

impl Dice for DoubleCrossPool {
    fn roll(&self) -> RollResult {
        self.validate().unwrap();

        let mut ctx = DiceContext::new();
        let mut all_rolls = Vec::new();
        let mut current_count = self.initial_count;
        let mut iteration = 0;
        let mut total = 0i64;

        // Roll through iterations
        while current_count > 0 && iteration < self.max_iterations {
            let rolls = ctx.roll_dice(current_count, self.sides);
            all_rolls.extend_from_slice(&rolls);

            // For all but last iteration, add max die value
            if iteration == 0 {
                // First iteration: find max
                if let Some(max_roll) = rolls.iter().max_by_key(|r| r.value) {
                    total += max_roll.value as i64;
                }
            } else {
                // Middle iterations: add sides (per OneDice spec)
                total += self.sides as i64;
            }

            // Count successes for next iteration
            current_count = rolls
                .iter()
                .filter(|r| r.value >= self.add_threshold)
                .count() as u32;

            iteration += 1;
        }

        // Last iteration: add max of final rolls
        if let Some(last_rolls) = all_rolls.chunks(self.initial_count as usize).last() {
            if let Some(max_roll) = last_rolls.iter().max_by_key(|r| r.value) {
                total += max_roll.value as i64;
            }
        }

        let description = format!(
            "{}c{}m{} = {}",
            self.initial_count, self.add_threshold, self.sides, total
        );

        RollResult::new(all_rolls, total, description)
    }

    fn describe(&self) -> String {
        format!(
            "{}c{} (double cross pool: add on {}+, {} sides)",
            self.initial_count, self.add_threshold, self.add_threshold, self.sides
        )
    }

    fn expected_value(&self) -> f64 {
        // Complex calculation
        // Rough estimate: similar to infinite adding but with different scoring
        let prob_add = (self.sides - self.add_threshold + 1) as f64 / self.sides as f64;
        let expected_max = (self.sides as f64 + 1.0) / 2.0; // Rough average of max

        // Very rough estimate
        expected_max * (1.0 + prob_add)
    }

    fn min_value(&self) -> i64 {
        1 // At least 1 from first max
    }

    fn max_value(&self) -> i64 {
        // Each iteration adds sides, last adds max
        (self.sides * (self.max_iterations + 1)) as i64
    }
}

impl fmt::Display for DoubleCrossPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
