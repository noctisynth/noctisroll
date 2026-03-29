//! Exploding dice implementation

use crate::core::{Dice, DiceContext, DieRoll, RollResult};
use crate::error::{DiceError, DiceResult};
use std::fmt;

/// Exploding dice (e.g., "explode on 6+")
#[derive(Debug, Clone, PartialEq)]
pub struct ExplodingDice {
    /// Number of dice to roll initially
    pub count: u32,
    /// Number of sides on each die
    pub sides: u32,
    /// Threshold for explosion (rolls >= threshold explode)
    pub explode_threshold: u32,
    /// Maximum number of explosions (None for unlimited)
    pub max_explosions: Option<u32>,
    /// Whether to explode below threshold instead of above
    pub explode_below: bool,
}

impl ExplodingDice {
    /// Create new exploding dice
    pub fn new(count: u32, sides: u32, explode_threshold: u32) -> Self {
        Self {
            count,
            sides,
            explode_threshold,
            max_explosions: None,
            explode_below: false,
        }
    }

    /// Set maximum number of explosions
    pub fn with_max_explosions(mut self, max: u32) -> Self {
        self.max_explosions = Some(max);
        self
    }

    /// Set to explode below threshold instead of above
    pub fn with_explode_below(mut self, threshold: u32) -> Self {
        self.explode_threshold = threshold;
        self.explode_below = true;
        self
    }

    /// Validate the dice configuration
    pub fn validate(&self) -> DiceResult<()> {
        if self.sides == 0 {
            return Err(DiceError::InvalidDice(
                "Dice must have at least 1 side".into(),
            ));
        }

        if self.explode_threshold == 0 || self.explode_threshold > self.sides {
            return Err(DiceError::InvalidParameter(format!(
                "Explode threshold must be between 1 and {}",
                self.sides
            )));
        }

        Ok(())
    }

    /// Roll a single die with explosion
    fn roll_exploding_die(&self, ctx: &mut DiceContext) -> Vec<DieRoll> {
        let mut rolls = Vec::new();
        let mut explosion_count = 0;

        loop {
            let roll = ctx.roll_die(self.sides);
            rolls.push(roll);

            // Check if we should explode
            let should_explode = if self.explode_below {
                roll.value <= self.explode_threshold
            } else {
                roll.value >= self.explode_threshold
            };

            if !should_explode {
                break;
            }

            // Check max explosions
            explosion_count += 1;
            if let Some(max) = self.max_explosions {
                if explosion_count >= max {
                    break;
                }
            }
        }

        rolls
    }
}

impl Dice for ExplodingDice {
    fn roll(&self) -> RollResult {
        self.validate().unwrap();

        let mut ctx = DiceContext::new();
        let mut all_rolls = Vec::new();
        let mut total = 0i64;

        // Roll initial dice
        for _ in 0..self.count {
            let die_rolls = self.roll_exploding_die(&mut ctx);
            let die_total: u32 = die_rolls.iter().map(|r| r.value).sum();

            all_rolls.extend(die_rolls);
            total += die_total as i64;
        }

        // Build description
        let direction = if self.explode_below { "≤" } else { "≥" };
        let mut description = format!(
            "{}d{}!{}{}",
            self.count, self.sides, direction, self.explode_threshold
        );

        if let Some(max) = self.max_explosions {
            description.push_str(&format!(" (max {} explosions)", max));
        }

        // Add roll details
        description.push_str(" [");
        let mut current_die = 0;
        let mut die_start = 0;

        while die_start < all_rolls.len() {
            if current_die > 0 {
                description.push_str(" | ");
            }

            // Find rolls for this die
            let mut die_end = die_start;
            while die_end < all_rolls.len() {
                if die_end + 1 < all_rolls.len() {
                    let next_roll = &all_rolls[die_end + 1];
                    let _current_roll = &all_rolls[die_end];
                    // If next roll is from explosion of same die, continue
                    if next_roll.value >= self.explode_threshold && !self.explode_below
                        || next_roll.value <= self.explode_threshold && self.explode_below
                    {
                        die_end += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let die_rolls = &all_rolls[die_start..=die_end];
            for (i, roll) in die_rolls.iter().enumerate() {
                if i > 0 {
                    description.push('!');
                }
                description.push_str(&roll.value.to_string());
            }

            die_start = die_end + 1;
            current_die += 1;
        }

        description.push(']');

        RollResult::new(all_rolls, total, description)
    }

    fn describe(&self) -> String {
        let direction = if self.explode_below { "≤" } else { "≥" };
        let mut desc = format!(
            "{}d{} exploding on {} {}",
            self.count, self.sides, direction, self.explode_threshold
        );

        if let Some(max) = self.max_explosions {
            desc.push_str(&format!(" (max {} explosions)", max));
        }

        desc
    }

    fn expected_value(&self) -> f64 {
        // Expected value per initial die
        let avg_per_roll = (self.sides as f64 + 1.0) / 2.0;

        // Probability of explosion
        let explosion_prob = if self.explode_below {
            self.explode_threshold as f64 / self.sides as f64
        } else {
            (self.sides - self.explode_threshold + 1) as f64 / self.sides as f64
        };

        // Expected number of rolls per die (geometric series)
        let expected_rolls_per_die = 1.0 / (1.0 - explosion_prob);

        // Apply max explosions limit
        let expected_rolls = if let Some(max) = self.max_explosions {
            let max_rolls = (max + 1) as f64; // initial + explosions
            expected_rolls_per_die.min(max_rolls)
        } else {
            expected_rolls_per_die
        };

        avg_per_roll * expected_rolls * self.count as f64
    }

    fn min_value(&self) -> i64 {
        // Minimum is count * 1 (no explosions at minimum)
        self.count as i64
    }

    fn max_value(&self) -> i64 {
        // Theoretical maximum is infinite without limit
        if let Some(max_explosions) = self.max_explosions {
            let total_rolls = self.count * (max_explosions + 1);
            (total_rolls * self.sides) as i64
        } else {
            i64::MAX // Practical limit
        }
    }
}

impl fmt::Display for ExplodingDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
