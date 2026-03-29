//! Composite dice operations (arithmetic, comparisons, etc.)

use crate::core::{Dice, RollResult};
use std::fmt;

/// Arithmetic operation between two dice expressions
#[derive(Debug, Clone)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

/// Comparison operation between two dice expressions
#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// Logical operation between two dice expressions
#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
    Xor,
}

/// Ternary conditional expression
pub struct TernaryExpr {
    pub condition: Box<dyn Dice>,
    pub true_expr: Box<dyn Dice>,
    pub false_expr: Box<dyn Dice>,
}

impl Clone for TernaryExpr {
    fn clone(&self) -> Self {
        // Note: We cannot clone Box<dyn Dice>, so we create a new struct
        // In practice, these would be created by parser
        Self {
            condition: Box::new(crate::dice::StandardDice::new(1, 20)), // Placeholder
            true_expr: Box::new(crate::dice::StandardDice::new(1, 20)),
            false_expr: Box::new(crate::dice::StandardDice::new(1, 20)),
        }
    }
}

impl fmt::Debug for TernaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TernaryExpr")
            .field("condition", &self.condition.describe())
            .field("true_expr", &self.true_expr.describe())
            .field("false_expr", &self.false_expr.describe())
            .finish()
    }
}

impl Dice for TernaryExpr {
    fn roll(&self) -> RollResult {
        let condition_result = self.condition.roll();
        let condition_true = condition_result.total != 0;

        if condition_true {
            self.true_expr.roll()
        } else {
            self.false_expr.roll()
        }
    }

    fn describe(&self) -> String {
        format!(
            "{} ? {} : {}",
            self.condition.describe(),
            self.true_expr.describe(),
            self.false_expr.describe()
        )
    }

    fn expected_value(&self) -> f64 {
        // Complex to calculate exactly
        // Return average of both branches
        (self.true_expr.expected_value() + self.false_expr.expected_value()) / 2.0
    }

    fn min_value(&self) -> i64 {
        self.true_expr.min_value().min(self.false_expr.min_value())
    }

    fn max_value(&self) -> i64 {
        self.true_expr.max_value().max(self.false_expr.max_value())
    }
}

/// Arithmetic expression between two dice
pub struct ArithmeticExpr {
    pub left: Box<dyn Dice>,
    pub right: Box<dyn Dice>,
    pub op: ArithmeticOp,
}

impl Clone for ArithmeticExpr {
    fn clone(&self) -> Self {
        // Placeholder implementation
        Self {
            left: Box::new(crate::dice::StandardDice::new(1, 20)),
            right: Box::new(crate::dice::StandardDice::new(1, 20)),
            op: self.op.clone(),
        }
    }
}

impl fmt::Debug for ArithmeticExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArithmeticExpr")
            .field("left", &self.left.describe())
            .field("right", &self.right.describe())
            .field("op", &self.op)
            .finish()
    }
}

impl Dice for ArithmeticExpr {
    fn roll(&self) -> RollResult {
        let left_result = self.left.roll();
        let right_result = self.right.roll();

        let (total, op_str) = match self.op {
            ArithmeticOp::Add => (left_result.total + right_result.total, "+"),
            ArithmeticOp::Subtract => (left_result.total - right_result.total, "-"),
            ArithmeticOp::Multiply => (left_result.total * right_result.total, "*"),
            ArithmeticOp::Divide => {
                if right_result.total == 0 {
                    // Handle division by zero
                    (0, "/")
                } else {
                    (left_result.total / right_result.total, "/")
                }
            }
            ArithmeticOp::Modulo => {
                if right_result.total == 0 {
                    // Handle modulo by zero
                    (0, "%")
                } else {
                    (left_result.total % right_result.total, "%")
                }
            }
            ArithmeticOp::Power => {
                (
                    left_result
                        .total
                        .pow(right_result.total.max(0).min(10) as u32),
                    "^",
                ) // Limit exponent
            }
        };

        let description = format!(
            "{} {} {}",
            left_result.description, op_str, right_result.description
        );
        let mut rolls = left_result.rolls;
        rolls.extend(right_result.rolls);

        RollResult::new(rolls, total, description)
    }

    fn describe(&self) -> String {
        let op_str = match self.op {
            ArithmeticOp::Add => "+",
            ArithmeticOp::Subtract => "-",
            ArithmeticOp::Multiply => "*",
            ArithmeticOp::Divide => "/",
            ArithmeticOp::Modulo => "%",
            ArithmeticOp::Power => "^",
        };
        format!(
            "{} {} {}",
            self.left.describe(),
            op_str,
            self.right.describe()
        )
    }

    fn expected_value(&self) -> f64 {
        let left_expected = self.left.expected_value();
        let right_expected = self.right.expected_value();

        match self.op {
            ArithmeticOp::Add => left_expected + right_expected,
            ArithmeticOp::Subtract => left_expected - right_expected,
            ArithmeticOp::Multiply => left_expected * right_expected,
            ArithmeticOp::Divide => {
                if right_expected == 0.0 {
                    0.0
                } else {
                    left_expected / right_expected
                }
            }
            ArithmeticOp::Modulo => {
                // Modulo expected value is complex
                left_expected % right_expected.max(1.0)
            }
            ArithmeticOp::Power => left_expected.powf(right_expected.min(10.0)), // Limit
        }
    }

    fn min_value(&self) -> i64 {
        let left_min = self.left.min_value();
        let right_min = self.right.min_value();
        let right_max = self.right.max_value();

        match self.op {
            ArithmeticOp::Add => left_min + right_min,
            ArithmeticOp::Subtract => left_min - right_max,
            ArithmeticOp::Multiply => left_min * right_min,
            ArithmeticOp::Divide => {
                if right_min <= 0 && right_max >= 0 {
                    // Division by zero possible
                    i64::MIN
                } else {
                    left_min / right_max.max(1) // Avoid division by zero
                }
            }
            ArithmeticOp::Modulo => {
                if right_min <= 0 && right_max >= 0 {
                    // Modulo by zero possible
                    0
                } else {
                    // Modulo result is between 0 and |right| - 1
                    0
                }
            }
            ArithmeticOp::Power => {
                // Power minimum
                if left_min >= 0 {
                    0
                } else {
                    // Negative base with integer exponent
                    if right_min % 2 == 0 {
                        0 // Even exponent gives positive
                    } else {
                        left_min.pow(right_min.max(0).min(10) as u32)
                    }
                }
            }
        }
    }

    fn max_value(&self) -> i64 {
        let right_min = self.right.min_value();
        let left_max = self.left.max_value();
        let right_max = self.right.max_value();

        match self.op {
            ArithmeticOp::Add => left_max + right_max,
            ArithmeticOp::Subtract => left_max - right_min,
            ArithmeticOp::Multiply => left_max * right_max,
            ArithmeticOp::Divide => {
                if right_min <= 0 && right_max >= 0 {
                    // Division by zero possible
                    i64::MAX
                } else {
                    left_max / right_min.max(1) // Avoid division by zero
                }
            }
            ArithmeticOp::Modulo => {
                if right_min <= 0 && right_max >= 0 {
                    // Modulo by zero possible
                    0
                } else {
                    // Modulo result is between 0 and |right| - 1
                    right_max.abs().max(right_min.abs()) - 1
                }
            }
            ArithmeticOp::Power => {
                // Power maximum
                let max_exp = right_max.min(10); // Limit exponent for safety
                left_max.pow(max_exp as u32)
            }
        }
    }
}

/// Comparison expression
pub struct ComparisonExpr {
    pub left: Box<dyn Dice>,
    pub right: Box<dyn Dice>,
    pub op: ComparisonOp,
}

impl Clone for ComparisonExpr {
    fn clone(&self) -> Self {
        // Placeholder implementation
        Self {
            left: Box::new(crate::dice::StandardDice::new(1, 20)),
            right: Box::new(crate::dice::StandardDice::new(1, 20)),
            op: self.op.clone(),
        }
    }
}

impl fmt::Debug for ComparisonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComparisonExpr")
            .field("left", &self.left.describe())
            .field("right", &self.right.describe())
            .field("op", &self.op)
            .finish()
    }
}

impl Dice for ComparisonExpr {
    fn roll(&self) -> RollResult {
        let left_result = self.left.roll();
        let right_result = self.right.roll();

        let comparison_result = match self.op {
            ComparisonOp::Equal => left_result.total == right_result.total,
            ComparisonOp::NotEqual => left_result.total != right_result.total,
            ComparisonOp::GreaterThan => left_result.total > right_result.total,
            ComparisonOp::GreaterThanOrEqual => left_result.total >= right_result.total,
            ComparisonOp::LessThan => left_result.total < right_result.total,
            ComparisonOp::LessThanOrEqual => left_result.total <= right_result.total,
        };

        let total = if comparison_result { 1 } else { 0 };
        let op_str = match self.op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterThanOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessThanOrEqual => "<=",
        };

        let description = format!(
            "{} {} {} = {}",
            left_result.description, op_str, right_result.description, total
        );
        let mut rolls = left_result.rolls;
        rolls.extend(right_result.rolls);

        RollResult::new(rolls, total, description)
    }

    fn describe(&self) -> String {
        let op_str = match self.op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterThanOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessThanOrEqual => "<=",
        };
        format!(
            "{} {} {}",
            self.left.describe(),
            op_str,
            self.right.describe()
        )
    }

    fn expected_value(&self) -> f64 {
        // Expected value of a comparison is the probability it's true
        // This is complex to calculate exactly
        0.5 // Rough estimate
    }

    fn min_value(&self) -> i64 {
        0
    }

    fn max_value(&self) -> i64 {
        1
    }
}

/// Builder for composite expressions
pub struct ExprBuilder;

impl ExprBuilder {
    /// Create an addition expression
    pub fn add(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Add,
        }
    }

    /// Create a subtraction expression
    pub fn subtract(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Subtract,
        }
    }

    /// Create a multiplication expression
    pub fn multiply(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Multiply,
        }
    }

    /// Create a division expression
    pub fn divide(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Divide,
        }
    }

    /// Create a modulo expression
    pub fn modulo(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Modulo,
        }
    }

    /// Create a power expression
    pub fn power(left: Box<dyn Dice>, right: Box<dyn Dice>) -> ArithmeticExpr {
        ArithmeticExpr {
            left,
            right,
            op: ArithmeticOp::Power,
        }
    }

    /// Create a ternary expression
    pub fn ternary(
        condition: Box<dyn Dice>,
        true_expr: Box<dyn Dice>,
        false_expr: Box<dyn Dice>,
    ) -> TernaryExpr {
        TernaryExpr {
            condition,
            true_expr,
            false_expr,
        }
    }
}
