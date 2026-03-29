//! Pest-based parser for dice expressions

use pest::Parser as PestParserTrait;
use pest_derive::Parser;

use super::ParserConfig;
use crate::core::{Dice, RollResult};
use crate::dice::*;
use crate::error::{DiceError, DiceResult};

#[derive(Parser)]
#[grammar = "src/parser/dice.pest"]
struct DiceParser;

/// Pest-based parser implementation
pub struct PestParser {
    config: ParserConfig,
}

impl PestParser {
    /// Create a new parser
    pub fn new(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse and evaluate a dice expression
    pub fn eval(&self, expr: &str) -> DiceResult<RollResult> {
        let dice = self.parse(expr)?;
        Ok(dice.roll())
    }

    /// Parse a dice expression without evaluating it
    pub fn parse(&self, expr: &str) -> DiceResult<Box<dyn Dice>> {
        let pairs = DiceParser::parse(Rule::expression, expr)
            .map_err(|e| DiceError::ParseError(e.to_string()))?;

        self.parse_expression(pairs)
    }

    /// Parse an expression from pest pairs
    fn parse_expression(&self, pairs: pest::iterators::Pairs<Rule>) -> DiceResult<Box<dyn Dice>> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::expression => return self.parse_expression(pair.into_inner()),
                Rule::term => return self.parse_term(pair.into_inner()),
                _ => {}
            }
        }

        Err(DiceError::ParseError("Empty expression".into()))
    }

    /// Parse a term from pest pairs
    fn parse_term(&self, pairs: pest::iterators::Pairs<Rule>) -> DiceResult<Box<dyn Dice>> {
        let mut current: Option<Box<dyn Dice>> = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::factor => {
                    let factor = self.parse_factor(pair.into_inner())?;
                    current = Some(factor);
                }
                Rule::op => {
                    let op = pair.as_str();
                    // TODO: Implement operator handling
                    // This will require building expression trees
                    // For now, we'll just note the operator exists
                    println!("DEBUG: Found operator: {}", op);
                }
                _ => {}
            }
        }

        current.ok_or_else(|| DiceError::ParseError("No valid term found".into()))
    }

    /// Parse a factor from pest pairs
    fn parse_factor(&self, pairs: pest::iterators::Pairs<Rule>) -> DiceResult<Box<dyn Dice>> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::dice_expr => return self.parse_dice_expr(pair.into_inner()),
                Rule::number => {
                    let num = pair
                        .as_str()
                        .parse::<i64>()
                        .map_err(|e| DiceError::ParseError(e.to_string()))?;
                    // Create a constant dice
                    return Ok(Box::new(ConstantDice::new(num)));
                }
                Rule::paren_expr => return self.parse_expression(pair.into_inner()),
                _ => {}
            }
        }

        Err(DiceError::ParseError("No valid factor found".into()))
    }

    /// Parse a dice expression from pest pairs
    fn parse_dice_expr(&self, pairs: pest::iterators::Pairs<Rule>) -> DiceResult<Box<dyn Dice>> {
        let mut dice_type = None;
        let mut count = 1u32;
        let mut sides = self.config.default_sides;
        let mut params = DiceParams::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::dice_type => {
                    dice_type = Some(pair.as_str());
                }
                Rule::dice_count => {
                    count = pair
                        .as_str()
                        .parse::<u32>()
                        .map_err(|e| DiceError::ParseError(e.to_string()))?;
                }
                Rule::dice_sides => {
                    sides = pair
                        .as_str()
                        .parse::<u32>()
                        .map_err(|e| DiceError::ParseError(e.to_string()))?;
                }
                Rule::dice_param => {
                    self.parse_dice_param(pair.into_inner(), &mut params)?;
                }
                _ => {}
            }
        }

        let dice_type =
            dice_type.ok_or_else(|| DiceError::ParseError("No dice type specified".into()))?;

        match dice_type {
            "d" => self.build_standard_dice(count, sides, params),
            "a" => self.build_infinite_pool(count, sides, params),
            "c" => self.build_double_cross_pool(count, sides, params),
            "f" | "df" => self.build_fate_dice(count),
            _ => Err(DiceError::ParseError(format!(
                "Unknown dice type: {}",
                dice_type
            ))),
        }
    }

    /// Parse dice parameters
    fn parse_dice_param(
        &self,
        pairs: pest::iterators::Pairs<Rule>,
        params: &mut DiceParams,
    ) -> DiceResult<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::keep_highest => {
                    let n = pair
                        .into_inner()
                        .next()
                        .and_then(|p| p.as_str().parse::<u32>().ok())
                        .ok_or_else(|| {
                            DiceError::ParseError("Invalid keep highest parameter".into())
                        })?;
                    params.keep_highest = Some(n);
                }
                Rule::keep_lowest => {
                    let n = pair
                        .into_inner()
                        .next()
                        .and_then(|p| p.as_str().parse::<u32>().ok())
                        .ok_or_else(|| {
                            DiceError::ParseError("Invalid keep lowest parameter".into())
                        })?;
                    params.keep_lowest = Some(n);
                }
                Rule::explode => {
                    let threshold = pair
                        .into_inner()
                        .next()
                        .and_then(|p| p.as_str().parse::<u32>().ok())
                        .unwrap_or(params.sides);
                    params.explode_threshold = Some(threshold);
                }
                Rule::success_threshold => {
                    let threshold = pair
                        .into_inner()
                        .next()
                        .and_then(|p| p.as_str().parse::<u32>().ok())
                        .ok_or_else(|| DiceError::ParseError("Invalid success threshold".into()))?;
                    params.success_threshold = Some(threshold);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Build a standard dice from parameters
    fn build_standard_dice(
        &self,
        count: u32,
        sides: u32,
        params: DiceParams,
    ) -> DiceResult<Box<dyn Dice>> {
        let mut dice = StandardDice::new(count, sides);

        if let Some(n) = params.keep_highest {
            dice = dice.keep_highest(n);
        }

        if let Some(n) = params.keep_lowest {
            dice = dice.keep_lowest(n);
        }

        if let Some(threshold) = params.success_threshold {
            dice = dice.with_success_threshold(threshold);
        }

        if let Some(threshold) = params.explode_threshold {
            let exploding = ExplodingDice::new(count, sides, threshold);
            return Ok(Box::new(exploding));
        }

        Ok(Box::new(dice))
    }

    /// Build an infinite adding pool
    fn build_infinite_pool(
        &self,
        count: u32,
        sides: u32,
        params: DiceParams,
    ) -> DiceResult<Box<dyn Dice>> {
        let add_threshold = params.success_threshold.ok_or_else(|| {
            DiceError::ParseError("Infinite pool requires success threshold".into())
        })?;

        let mut pool = InfiniteAddingPool::new(count, add_threshold).with_sides(sides);

        if let Some(threshold) = params.success_threshold {
            pool = pool.with_success_threshold(threshold);
        }

        Ok(Box::new(pool))
    }

    /// Build a double cross pool
    fn build_double_cross_pool(
        &self,
        count: u32,
        sides: u32,
        params: DiceParams,
    ) -> DiceResult<Box<dyn Dice>> {
        let add_threshold = params.success_threshold.ok_or_else(|| {
            DiceError::ParseError("Double cross pool requires success threshold".into())
        })?;

        let pool = DoubleCrossPool::new(count, add_threshold).with_sides(sides);

        Ok(Box::new(pool))
    }

    /// Build fate dice
    fn build_fate_dice(&self, count: u32) -> DiceResult<Box<dyn Dice>> {
        Ok(Box::new(FateDice::new(count)))
    }
}

/// Dice parameters parsed from expression
#[derive(Debug, Default)]
struct DiceParams {
    keep_highest: Option<u32>,
    keep_lowest: Option<u32>,
    explode_threshold: Option<u32>,
    success_threshold: Option<u32>,
    sides: u32,
}

impl DiceParams {
    fn new() -> Self {
        Self::default()
    }
}

/// Constant dice (for numbers in expressions)
struct ConstantDice {
    value: i64,
}

impl ConstantDice {
    fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Dice for ConstantDice {
    fn roll(&self) -> RollResult {
        RollResult::new(Vec::new(), self.value, self.value.to_string())
    }

    fn describe(&self) -> String {
        self.value.to_string()
    }

    fn expected_value(&self) -> f64 {
        self.value as f64
    }

    fn min_value(&self) -> i64 {
        self.value
    }

    fn max_value(&self) -> i64 {
        self.value
    }
}
