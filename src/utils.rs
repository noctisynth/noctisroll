//! Utility functions for dice rolling

use crate::core::{DieRoll, RollResult};
use crate::error::DiceError;

/// Calculate statistics for a series of rolls
#[derive(Debug, Clone)]
pub struct RollStatistics {
    /// Minimum roll value
    pub min: u32,
    /// Maximum roll value
    pub max: u32,
    /// Average roll value
    pub mean: f64,
    /// Median roll value
    pub median: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Number of critical successes
    pub critical_successes: u32,
    /// Number of critical failures
    pub critical_failures: u32,
}

impl RollStatistics {
    /// Calculate statistics from a roll result
    pub fn from_result(result: &RollResult) -> Self {
        Self::from_rolls(&result.rolls)
    }

    /// Calculate statistics from individual die rolls
    pub fn from_rolls(rolls: &[DieRoll]) -> Self {
        if rolls.is_empty() {
            return Self {
                min: 0,
                max: 0,
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                critical_successes: 0,
                critical_failures: 0,
            };
        }

        let values: Vec<u32> = rolls.iter().map(|r| r.value).collect();
        let min = *values.iter().min().unwrap_or(&0);
        let max = *values.iter().max().unwrap_or(&0);

        let sum: u32 = values.iter().sum();
        let mean = sum as f64 / values.len() as f64;

        // Calculate variance
        let variance = values
            .iter()
            .map(|&v| {
                let diff = v as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / values.len() as f64;
        let std_dev = variance.sqrt();

        // Calculate median
        let mut sorted = values.clone();
        sorted.sort();
        let median = if sorted.len().is_multiple_of(2) {
            let mid = sorted.len() / 2;
            (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
        } else {
            sorted[sorted.len() / 2] as f64
        };

        let critical_successes = rolls.iter().filter(|r| r.is_critical_success).count() as u32;
        let critical_failures = rolls.iter().filter(|r| r.is_critical_failure).count() as u32;

        Self {
            min,
            max,
            mean,
            median,
            std_dev,
            critical_successes,
            critical_failures,
        }
    }
}

/// Format a roll result as a detailed string
pub fn format_detailed(result: &RollResult) -> String {
    let mut output = String::new();

    output.push_str(&format!("Roll: {}\n", result.description));
    output.push_str(&format!("Total: {}\n", result.total));

    if !result.rolls.is_empty() {
        output.push_str("Individual rolls: ");
        for (i, roll) in result.rolls.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            output.push_str(&roll.value.to_string());

            if roll.is_critical_success {
                output.push_str(" (crit!)");
            } else if roll.is_critical_failure {
                output.push_str(" (fumble!)");
            }
        }
        output.push('\n');
    }

    if let Some(success) = result.success {
        output.push_str(&format!("Success: {}\n", success));
    }

    if let Some(success_count) = result.success_count {
        output.push_str(&format!("Successes: {}\n", success_count));
    }

    if let Some(failure_count) = result.failure_count {
        output.push_str(&format!("Failures: {}\n", failure_count));
    }

    output
}

/// Parse a dice notation string (e.g., "2d20", "d6", "4dF")
pub fn parse_dice_notation(notation: &str) -> Result<Box<dyn crate::core::Dice>, DiceError> {
    // Simple parser for common dice notations
    let notation = notation.trim();

    // Check for FATE dice first (before to_lowercase)
    if notation.to_lowercase().ends_with("df") || notation.to_lowercase().ends_with('f') {
        let lower = notation.to_lowercase();
        let count_str = if lower.ends_with("df") {
            &lower[..lower.len() - 2]
        } else {
            &lower[..lower.len() - 1]
        };

        let count = if count_str.is_empty() {
            4 // Default 4dF
        } else {
            count_str
                .parse::<u32>()
                .map_err(|e| DiceError::ParseError(e.to_string()))?
        };

        return Ok(Box::new(crate::dice::FateDice::new(count)));
    }

    let notation = notation.to_lowercase();

    if let Some(stripped) = notation.strip_prefix('d') {
        // Handle dX notation
        let sides = stripped
            .parse::<u32>()
            .map_err(|e| DiceError::ParseError(e.to_string()))?;
        Ok(Box::new(crate::dice::StandardDice::new(1, sides)))
    } else if notation.contains('d') {
        // Handle XdY notation
        let parts: Vec<&str> = notation.split('d').collect();
        if parts.len() != 2 {
            return Err(DiceError::ParseError("Invalid dice notation".into()));
        }

        let count = parts[0]
            .parse::<u32>()
            .map_err(|e| DiceError::ParseError(e.to_string()))?;
        let sides = parts[1]
            .parse::<u32>()
            .map_err(|e| DiceError::ParseError(e.to_string()))?;

        Ok(Box::new(crate::dice::StandardDice::new(count, sides)))
    } else {
        Err(DiceError::ParseError("Unrecognized dice notation".into()))
    }
}

/// Roll multiple dice and combine results
pub fn roll_multiple(dice: &[Box<dyn crate::core::Dice>]) -> Vec<RollResult> {
    dice.iter().map(|d| d.roll()).collect()
}

/// Batch roll the same dice multiple times
pub fn batch_roll(dice: &dyn crate::core::Dice, times: u32) -> Vec<RollResult> {
    (0..times).map(|_| dice.roll()).collect()
}

/// Calculate probability distribution for a dice type
pub fn probability_distribution(
    dice: &dyn crate::core::Dice,
    samples: u32,
) -> std::collections::HashMap<i64, u32> {
    let mut distribution = std::collections::HashMap::new();

    for _ in 0..samples {
        let result = dice.roll();
        *distribution.entry(result.total).or_insert(0) += 1;
    }

    distribution
}
