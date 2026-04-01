use rig::tool::ToolDyn;
use rig_derive::rig_tool;

use crate::{core, dice};

/// Roll a number of standard dice with a given number of sides
#[rig_tool(
    description = "Roll a number of standard dice with a given number of sides",
    params(
        count = "The number of dice to roll",
        sides = "The number of sides on each die",
        kl = "Number of lowest dice to keep (default: 1)",
        kh = "Number of highest dice to keep (default: 1)",
        bonus = "Number of bonus dice (positive) or penalty dice (negative)",
        threshold = "Success threshold (rolls >= threshold count as successes)",
    ),
    required(count, sides)
)]
pub fn roll_dice(
    count: u32,
    sides: u32,
    kl: Option<u32>,
    kh: Option<u32>,
    bonus: Option<i32>,
    threshold: Option<u32>,
) -> Result<core::RollResult, rig::tool::ToolError> {
    use crate::core::Dice;
    let mut dice = dice::StandardDice::new(count, sides);
    if let Some(kl) = kl {
        dice = dice.keep_lowest(kl);
    }
    if let Some(kh) = kh {
        dice = dice.keep_highest(kh);
    }
    if let Some(bonus) = bonus {
        dice = dice.with_bonus(bonus);
    }
    if let Some(threshold) = threshold {
        dice = dice.with_success_threshold(threshold);
    }
    Ok(dice.roll())
}

/// Roll FATE/Fudge dice
#[rig_tool(
    description = "Roll FATE/Fudge dice (dF)",
    params(count = "The number of FATE dice to roll (default: 4)",),
    required(count)
)]
pub fn roll_fate_dice(count: Option<u32>) -> Result<core::RollResult, rig::tool::ToolError> {
    use crate::core::Dice;
    let count = count.unwrap_or(4);
    let dice = dice::FateDice::new(count);
    Ok(dice.roll())
}

/// Roll exploding dice
#[rig_tool(
    description = "Roll exploding dice (rolls that meet threshold explode and add another die)",
    params(
        count = "The number of dice to roll",
        sides = "The number of sides on each die",
        threshold = "Threshold for explosion (rolls >= threshold explode)",
        max_explosions = "Maximum number of explosions per die (optional)",
        explode_below = "Explode on rolls below threshold instead of above (default: false)",
    ),
    required(count, sides, threshold)
)]
pub fn roll_exploding_dice(
    count: u32,
    sides: u32,
    threshold: u32,
    max_explosions: Option<u32>,
    explode_below: Option<bool>,
) -> Result<core::RollResult, rig::tool::ToolError> {
    use crate::core::Dice;
    let mut dice = dice::ExplodingDice::new(count, sides, threshold);

    if let Some(max) = max_explosions {
        dice = dice.with_max_explosions(max);
    }

    if explode_below.unwrap_or(false) {
        dice = dice.with_explode_below(threshold);
    }

    Ok(dice.roll())
}

/// Parse and evaluate a dice expression string
#[cfg(feature = "parser")]
#[rig_tool(
    description = "Parse and evaluate a dice expression string",
    params(expression = "Dice expression to evaluate (e.g., '2d20kh1 + 5', '4d6dl1', '3d10!8')",),
    required(expression)
)]
pub fn eval_dice_expression(expression: String) -> Result<core::RollResult, rig::tool::ToolError> {
    use crate::parser::Parser;

    let parser = Parser::new();
    parser.eval(&expression).map_err(|e| e.into())
}

pub fn tools() -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(RollDice),
        Box::new(RollFateDice),
        Box::new(RollExplodingDice),
        #[cfg(feature = "parser")]
        Box::new(EvalDiceExpression),
    ]
}
