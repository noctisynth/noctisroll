use noctisroll::prelude::*;

#[test]
fn test_standard_dice() {
    // Test basic dice
    let dice = StandardDice::new(2, 6);
    let result = dice.roll();

    assert!(result.total >= 2 && result.total <= 12);
    assert_eq!(result.rolls.len(), 2);

    // Test with keep highest
    let dice = StandardDice::new(4, 6).keep_highest(2);
    let result = dice.roll();

    assert!(result.total >= 2 && result.total <= 12);
    assert_eq!(result.rolls.len(), 2); // Only kept rolls

    // Test with keep lowest
    let dice = StandardDice::new(4, 6).keep_lowest(2);
    let result = dice.roll();

    assert!(result.total >= 2 && result.total <= 12);
    assert_eq!(result.rolls.len(), 2);
}

#[test]
fn test_fate_dice() {
    let dice = FateDice::new(4);
    let result = dice.roll();

    // 4dF ranges from -4 to +4
    assert!(result.total >= -4 && result.total <= 4);
    assert_eq!(result.rolls.len(), 4);
}

#[test]
fn test_exploding_dice() {
    let dice = ExplodingDice::new(2, 6, 6); // Explode on 6
    let result = dice.roll();

    // At least 2 dice rolled
    assert!(result.rolls.len() >= 2);

    // All rolls should be between 1 and 6
    for roll in &result.rolls {
        assert!(roll.value >= 1 && roll.value <= 6);
    }
}

#[test]
fn test_dice_validation() {
    // Invalid dice should fail validation
    let dice = StandardDice::new(2, 0).keep_lowest(12);
    assert!(dice.validate().is_err());

    // Valid dice should pass
    let dice = StandardDice::new(2, 6);
    assert!(dice.validate().is_ok());

    // Dice with 0 count is actually valid (empty roll)
    let dice = StandardDice::new(0, 6);
    assert!(dice.validate().is_ok());
}

#[test]
fn test_roll_statistics() {
    use noctisroll::utils::RollStatistics;

    let dice = StandardDice::new(100, 6);
    let result = dice.roll();
    let stats = RollStatistics::from_result(&result);

    assert!(stats.min >= 1 && stats.min <= 6);
    assert!(stats.max >= 1 && stats.max <= 6);
    assert!(stats.mean >= 1.0 && stats.mean <= 6.0);
}

#[test]
fn test_convenience_functions() {
    // Test d() function
    let dice = d(2, 20);
    let result = dice.roll();
    assert!(result.total >= 2 && result.total <= 40);

    // Test f() function
    let dice = f(4);
    let result = dice.roll();
    assert!(result.total >= -4 && result.total <= 4);

    // Test x() function
    let dice = x(2, 6, 6);
    let result = dice.roll();
    assert!(result.rolls.len() >= 2);
}

#[test]
fn test_dice_pool() {
    // Test infinite adding pool
    let pool = InfiniteAddingPool::new(5, 8); // 5 dice, add on 8+
    let result = pool.roll();

    assert!(result.total >= 0);
    // success_count is always >= 0 by definition, no need to assert

    // Test double cross pool
    let pool = DoubleCrossPool::new(5, 8); // 5 dice, add on 8+
    let result = pool.roll();

    assert!(result.total >= 1); // At least max of first roll
}

#[test]
fn test_parse_dice_notation() {
    use noctisroll::utils::parse_dice_notation;

    // Test dX notation
    let dice = parse_dice_notation("d20").unwrap();
    let result = dice.roll();
    assert!(result.total >= 1 && result.total <= 20);

    // Test XdY notation
    let dice = parse_dice_notation("2d6").unwrap();
    let result = dice.roll();
    assert!(result.total >= 2 && result.total <= 12);

    // Test FATE dice
    let dice = parse_dice_notation("4dF").unwrap(); // uppercase F
    let result = dice.roll();
    assert!(result.total >= -4 && result.total <= 4);

    // Test invalid notation
    assert!(parse_dice_notation("invalid").is_err());
}
