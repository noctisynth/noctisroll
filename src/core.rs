//! Core types and traits for the dice rolling system

use serde::{Deserialize, Serialize};
use std::fmt;

/// A single dice roll result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DieRoll {
    /// The face value of the die
    pub value: u32,
    /// The number of sides on the die
    pub sides: u32,
    /// Whether this roll is a critical success (natural max)
    pub is_critical_success: bool,
    /// Whether this roll is a critical failure (natural 1)
    pub is_critical_failure: bool,
}

impl DieRoll {
    /// Create a new die roll
    pub fn new(value: u32, sides: u32) -> Self {
        Self {
            value,
            sides,
            is_critical_success: value == sides,
            is_critical_failure: sides > 1 && value == 1,
        }
    }

    /// Check if the roll meets a target number
    pub fn meets_target(&self, target: u32) -> bool {
        self.value >= target
    }

    /// Check if the roll is within a range
    pub fn in_range(&self, min: u32, max: u32) -> bool {
        self.value >= min && self.value <= max
    }
}

impl fmt::Display for DieRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Result of a dice roll operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RollResult {
    /// Individual die rolls
    pub rolls: Vec<DieRoll>,
    /// Total value (sum of selected rolls)
    pub total: i64,
    /// Detailed description of the roll
    pub description: String,
    /// Whether the roll was successful (for success-based systems)
    pub success: Option<bool>,
    /// Number of successes (for success-based systems)
    pub success_count: Option<u32>,
    /// Number of failures (for success-based systems)
    pub failure_count: Option<u32>,
}

impl RollResult {
    /// Create a new roll result
    pub fn new(rolls: Vec<DieRoll>, total: i64, description: String) -> Self {
        Self {
            rolls,
            total,
            description,
            success: None,
            success_count: None,
            failure_count: None,
        }
    }

    /// Create a roll result with success information
    pub fn with_success(
        rolls: Vec<DieRoll>,
        total: i64,
        description: String,
        success: bool,
        success_count: u32,
        failure_count: u32,
    ) -> Self {
        Self {
            rolls,
            total,
            description,
            success: Some(success),
            success_count: Some(success_count),
            failure_count: Some(failure_count),
        }
    }

    /// Get the values of all rolls
    pub fn values(&self) -> Vec<u32> {
        self.rolls.iter().map(|r| r.value).collect()
    }

    /// Get the number of dice rolled
    pub fn dice_count(&self) -> usize {
        self.rolls.len()
    }

    /// Check if any roll was a critical success
    pub fn has_critical_success(&self) -> bool {
        self.rolls.iter().any(|r| r.is_critical_success)
    }

    /// Check if any roll was a critical failure
    pub fn has_critical_failure(&self) -> bool {
        self.rolls.iter().any(|r| r.is_critical_failure)
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.description, self.total)
    }
}

/// Trait for all dice types
pub trait Dice: Send + Sync {
    /// Roll the dice and return a result
    fn roll(&self) -> RollResult;

    /// Get a description of the dice
    fn describe(&self) -> String;

    /// Get the expected value (average) of the dice
    fn expected_value(&self) -> f64;

    /// Get the minimum possible roll
    fn min_value(&self) -> i64;

    /// Get the maximum possible roll
    fn max_value(&self) -> i64;
}

/// Trait for dice that can be modified with operations
pub trait ModifiableDice: Dice {
    /// Keep the highest N rolls
    fn keep_highest(self, n: u32) -> Box<dyn Dice>;

    /// Keep the lowest N rolls
    fn keep_lowest(self, n: u32) -> Box<dyn Dice>;

    /// Drop the highest N rolls
    fn drop_highest(self, n: u32) -> Box<dyn Dice>;

    /// Drop the lowest N rolls
    fn drop_lowest(self, n: u32) -> Box<dyn Dice>;

    /// Reroll rolls below a threshold
    fn reroll_below(self, threshold: u32) -> Box<dyn Dice>;

    /// Reroll rolls above a threshold
    fn reroll_above(self, threshold: u32) -> Box<dyn Dice>;

    /// Explode rolls above a threshold
    fn explode_above(self, threshold: u32) -> Box<dyn Dice>;

    /// Explode rolls below a threshold
    fn explode_below(self, threshold: u32) -> Box<dyn Dice>;
}

/// Configuration for dice rolling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceConfig {
    /// Default number of sides for a d (e.g., 20 for D&D, 100 for CoC)
    pub default_sides: u32,
    /// Whether to enable critical success/failure detection
    pub detect_criticals: bool,
    /// Random number generator seed (None for random)
    pub seed: Option<u64>,
    /// Maximum number of dice to roll (for safety)
    pub max_dice: u32,
    /// Maximum number of sides per die (for safety)
    pub max_sides: u32,
}

impl Default for DiceConfig {
    fn default() -> Self {
        Self {
            default_sides: 20, // D&D default
            detect_criticals: true,
            seed: None,
            max_dice: 1000,
            max_sides: 10000,
        }
    }
}

/// Context for dice rolling operations
#[derive(Debug, Clone)]
pub struct DiceContext {
    /// Configuration
    pub config: DiceConfig,
    /// Random number generator
    pub rng: rand::rngs::ThreadRng,
}

impl DiceContext {
    /// Create a new context with default configuration
    pub fn new() -> Self {
        Self {
            config: DiceConfig::default(),
            rng: rand::thread_rng(),
        }
    }

    /// Create a new context with custom configuration
    pub fn with_config(config: DiceConfig) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
        }
    }

    /// Roll a single die with the given number of sides
    pub fn roll_die(&mut self, sides: u32) -> DieRoll {
        use rand::Rng;

        if sides == 0 {
            return DieRoll::new(0, 0);
        }

        let value = self.rng.gen_range(1..=sides);
        DieRoll::new(value, sides)
    }

    /// Roll multiple dice with the same number of sides
    pub fn roll_dice(&mut self, count: u32, sides: u32) -> Vec<DieRoll> {
        (0..count).map(|_| self.roll_die(sides)).collect()
    }
}

impl Default for DiceContext {
    fn default() -> Self {
        Self::new()
    }
}
