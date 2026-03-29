//! Standard polyhedral dice implementation

use crate::core::{Dice, DiceContext, DieRoll, ModifiableDice, RollResult};
use crate::dice::exploding::ExplodingDice;
use crate::error::{DiceError, DiceResult};
use std::fmt;

/// Standard polyhedral dice (e.g., d20, d6, d100)
#[derive(Debug, Clone, PartialEq)]
pub struct StandardDice {
    /// Number of dice to roll
    pub count: u32,
    /// Number of sides on each die
    pub sides: u32,
    /// Number of highest dice to keep (k/q parameter)
    pub keep_highest: Option<u32>,
    /// Number of lowest dice to keep (k/q parameter)
    pub keep_lowest: Option<u32>,
    /// Number of bonus/penalty dice (p/b parameter)
    pub bonus_dice: i32,
    /// Success threshold for dice pool mode (a parameter)
    pub success_threshold: Option<u32>,
}

impl StandardDice {
    /// Create a new standard dice
    pub fn new(count: u32, sides: u32) -> Self {
        Self {
            count,
            sides,
            keep_highest: None,
            keep_lowest: None,
            bonus_dice: 0,
            success_threshold: None,
        }
    }

    /// Set the number of highest dice to keep (k parameter)
    pub fn keep_highest(mut self, n: u32) -> Self {
        self.keep_highest = Some(n);
        self.keep_lowest = None; // Can't have both
        self
    }

    /// Set the number of lowest dice to keep (q parameter)
    pub fn keep_lowest(mut self, n: u32) -> Self {
        self.keep_lowest = Some(n);
        self.keep_highest = None; // Can't have both
        self
    }

    /// Set bonus/penalty dice (p/b parameter)
    pub fn with_bonus(mut self, bonus: i32) -> Self {
        self.bonus_dice = bonus;
        self
    }

    /// Set success threshold for dice pool mode (a parameter)
    pub fn with_success_threshold(mut self, threshold: u32) -> Self {
        self.success_threshold = Some(threshold);
        self
    }

    /// Validate the dice configuration
    pub fn validate(&self) -> DiceResult<()> {
        if self.sides == 0 {
            return Err(DiceError::InvalidDice(
                "Dice must have at least 1 side".into(),
            ));
        }

        if let Some(n) = self.keep_highest {
            if n > self.count {
                return Err(DiceError::InvalidParameter(format!(
                    "Cannot keep {} dice when only rolling {}",
                    n, self.count
                )));
            }
        }

        if let Some(n) = self.keep_lowest {
            if n > self.count {
                return Err(DiceError::InvalidParameter(format!(
                    "Cannot keep {} dice when only rolling {}",
                    n, self.count
                )));
            }
        }

        Ok(())
    }

    /// Roll the dice and return individual rolls
    fn roll_dice(&self, ctx: &mut DiceContext) -> Vec<DieRoll> {
        let total_count = (self.count as i32 + self.bonus_dice).max(0) as u32;
        ctx.roll_dice(total_count, self.sides)
    }

    /// Apply keep/drop operations to rolls
    fn apply_keep_operations(&self, rolls: Vec<DieRoll>) -> (Vec<DieRoll>, Vec<DieRoll>) {
        let mut kept = rolls;
        let mut dropped = Vec::new();

        if let Some(n) = self.keep_highest {
            kept.sort_by(|a, b| b.value.cmp(&a.value));
            dropped = kept.split_off(n as usize);
        } else if let Some(n) = self.keep_lowest {
            kept.sort_by_key(|r| r.value);
            dropped = kept.split_off(n as usize);
        }

        (kept, dropped)
    }

    /// Calculate success count for dice pool mode
    fn calculate_successes(&self, rolls: &[DieRoll]) -> (u32, u32) {
        if let Some(threshold) = self.success_threshold {
            let success_count = rolls.iter().filter(|r| r.value >= threshold).count() as u32;
            let failure_count = rolls.len() as u32 - success_count;
            (success_count, failure_count)
        } else {
            (0, 0)
        }
    }
}

impl Dice for StandardDice {
    fn roll(&self) -> RollResult {
        self.validate().unwrap();

        let mut ctx = DiceContext::new();
        let mut rolls = self.roll_dice(&mut ctx);

        // Apply keep/drop operations
        let (kept, dropped) = self.apply_keep_operations(rolls);
        rolls = kept.clone();

        // Calculate total
        let total = if self.success_threshold.is_some() {
            // In dice pool mode, total is success count
            let (success_count, _) = self.calculate_successes(&rolls);
            success_count as i64
        } else {
            // Normal mode: sum of kept rolls
            rolls.iter().map(|r| r.value as i64).sum()
        };

        // Build description
        let mut description = String::new();

        if self.count > 1 || self.bonus_dice != 0 {
            description.push_str(&format!("{}d{}", self.count, self.sides));

            if self.bonus_dice > 0 {
                description.push_str(&format!("+{}d{}", self.bonus_dice, self.sides));
            } else if self.bonus_dice < 0 {
                description.push_str(&format!("{}d{}", self.bonus_dice, self.sides));
            }

            if let Some(n) = self.keep_highest {
                description.push_str(&format!("kh{}", n));
            } else if let Some(n) = self.keep_lowest {
                description.push_str(&format!("kl{}", n));
            }

            if let Some(threshold) = self.success_threshold {
                description.push_str(&format!("a{}", threshold));
            }
        } else {
            description.push_str(&format!("d{}", self.sides));
        }

        // Add roll details
        description.push_str(" [");
        for (i, roll) in rolls.iter().enumerate() {
            if i > 0 {
                description.push_str(", ");
            }
            description.push_str(&roll.value.to_string());
        }
        description.push(']');

        if !dropped.is_empty() {
            description.push_str(" (dropped: [");
            for (i, roll) in dropped.iter().enumerate() {
                if i > 0 {
                    description.push_str(", ");
                }
                description.push_str(&roll.value.to_string());
            }
            description.push_str("])");
        }

        if self.success_threshold.is_some() {
            let (success_count, failure_count) = self.calculate_successes(&rolls);
            RollResult::with_success(
                rolls,
                total,
                description,
                success_count > 0,
                success_count,
                failure_count,
            )
        } else {
            RollResult::new(rolls, total, description)
        }
    }

    fn describe(&self) -> String {
        let mut desc = format!("{}d{}", self.count, self.sides);

        if let Some(n) = self.keep_highest {
            desc.push_str(&format!(" (keep highest {})", n));
        } else if let Some(n) = self.keep_lowest {
            desc.push_str(&format!(" (keep lowest {})", n));
        }

        if self.bonus_dice > 0 {
            desc.push_str(&format!(" + {} bonus dice", self.bonus_dice));
        } else if self.bonus_dice < 0 {
            desc.push_str(&format!(" {} penalty dice", self.bonus_dice));
        }

        if let Some(threshold) = self.success_threshold {
            desc.push_str(&format!(" (success on {}+)", threshold));
        }

        desc
    }

    fn expected_value(&self) -> f64 {
        let avg_per_die = (self.sides as f64 + 1.0) / 2.0;
        let effective_count = self.count as f64;

        // Adjust for keep operations
        let multiplier = if let Some(n) = self.keep_highest {
            n as f64 / self.count as f64
        } else if let Some(n) = self.keep_lowest {
            n as f64 / self.count as f64
        } else {
            1.0
        };

        avg_per_die * effective_count * multiplier
    }

    fn min_value(&self) -> i64 {
        if let Some(n) = self.keep_highest {
            // When keeping highest, minimum is n * 1
            n as i64
        } else if let Some(n) = self.keep_lowest {
            // When keeping lowest, minimum is n * 1
            n as i64
        } else if self.success_threshold.is_some() {
            // In dice pool mode, minimum is 0 successes
            0
        } else {
            // Normal mode: count * 1
            self.count as i64
        }
    }

    fn max_value(&self) -> i64 {
        if let Some(n) = self.keep_highest {
            // When keeping highest, maximum is n * sides
            (n * self.sides) as i64
        } else if let Some(n) = self.keep_lowest {
            // When keeping lowest, maximum is n * sides
            (n * self.sides) as i64
        } else if self.success_threshold.is_some() {
            // In dice pool mode, maximum is count successes
            self.count as i64
        } else {
            // Normal mode: count * sides
            (self.count * self.sides) as i64
        }
    }
}

impl ModifiableDice for StandardDice {
    fn keep_highest(self, n: u32) -> Box<dyn Dice> {
        Box::new(self.keep_highest(n))
    }

    fn keep_lowest(self, n: u32) -> Box<dyn Dice> {
        Box::new(self.keep_lowest(n))
    }

    fn drop_highest(self, n: u32) -> Box<dyn Dice> {
        let count = self.count;
        Box::new(self.keep_lowest(count.saturating_sub(n)))
    }

    fn drop_lowest(self, n: u32) -> Box<dyn Dice> {
        let count = self.count;
        Box::new(self.keep_highest(count.saturating_sub(n)))
    }

    fn reroll_below(self, _threshold: u32) -> Box<dyn Dice> {
        // For now, return self (implementation would require state)
        Box::new(self)
    }

    fn reroll_above(self, _threshold: u32) -> Box<dyn Dice> {
        // For now, return self (implementation would require state)
        Box::new(self)
    }

    fn explode_above(self, threshold: u32) -> Box<dyn Dice> {
        Box::new(ExplodingDice::new(self.count, self.sides, threshold))
    }

    fn explode_below(self, threshold: u32) -> Box<dyn Dice> {
        Box::new(ExplodingDice::new(self.count, self.sides, 1).with_explode_below(threshold))
    }
}

impl fmt::Display for StandardDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}
