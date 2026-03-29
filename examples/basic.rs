//! Basic dice rolling examples

use noctisroll::prelude::*;

fn main() {
    println!("=== Basic Dice Rolling Examples ===\n");

    // 1. Standard dice
    println!("1. Standard Dice:");
    let dice = StandardDice::new(2, 6);
    let result = dice.roll();
    println!("   {} = {}", dice.describe(), result.total);
    println!("   Rolls: {:?}\n", result.values());

    // 2. Dice with advantage (keep highest)
    println!("2. Dice with Advantage:");
    let dice = StandardDice::new(2, 20).keep_highest(1);
    let result = dice.roll();
    println!("   {} = {}", dice.describe(), result.total);
    println!("   All rolls: {:?}", result.values());
    if result.has_critical_success() {
        println!("   Critical success! 🎉");
    }

    // 3. Dice with disadvantage (keep lowest)
    println!("\n3. Dice with Disadvantage:");
    let dice = StandardDice::new(2, 20).keep_lowest(1);
    let result = dice.roll();
    println!("   {} = {}", dice.describe(), result.total);
    if result.has_critical_failure() {
        println!("   Critical failure! 💀");
    }

    // 4. Multiple dice with selection
    println!("\n4. Multiple Dice Selection:");
    let dice = StandardDice::new(4, 6).keep_highest(3);
    let result = dice.roll();
    println!("   {} = {}", dice.describe(), result.total);
    println!("   Kept rolls: {:?}", result.values());

    // 5. Expected values
    println!("\n5. Expected Values:");
    let dice_types = [
        ("1d20", StandardDice::new(1, 20)),
        ("2d6", StandardDice::new(2, 6)),
        ("4d6k3", StandardDice::new(4, 6).keep_highest(3)),
        ("3d20kh1", StandardDice::new(3, 20).keep_highest(1)),
    ];

    for (desc, dice) in dice_types {
        println!(
            "   {}: avg = {:.2}, min = {}, max = {}",
            desc,
            dice.expected_value(),
            dice.min_value(),
            dice.max_value()
        );
    }

    // 6. Batch rolling
    println!("\n6. Batch Rolling (10d6):");
    let dice = StandardDice::new(10, 6);
    let results = noctisroll::utils::batch_roll(&dice, 5);

    for (i, result) in results.iter().enumerate() {
        println!(
            "   Roll {}: {} = {}",
            i + 1,
            result.description,
            result.total
        );
    }

    // 7. Statistics
    println!("\n7. Statistics:");
    let dice = StandardDice::new(100, 6);
    let result = dice.roll();
    let stats = noctisroll::utils::RollStatistics::from_result(&result);

    println!("   100d6 Statistics:");
    println!("   - Min: {}", stats.min);
    println!("   - Max: {}", stats.max);
    println!("   - Mean: {:.2}", stats.mean);
    println!("   - Median: {:.2}", stats.median);
    println!("   - Std Dev: {:.2}", stats.std_dev);
    println!("   - Critical Successes: {}", stats.critical_successes);
    println!("   - Critical Failures: {}", stats.critical_failures);

    println!("\n=== Examples Complete ===");
}
