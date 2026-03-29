//! FATE dice examples

use noctisroll::prelude::*;

fn main() {
    println!("=== FATE Dice Examples ===\n");

    // 1. Standard FATE dice (4dF)
    println!("1. Standard FATE Dice (4dF):");
    let fate = FateDice::new(4);
    let result = fate.roll();
    println!("   {} = {}", fate.describe(), result.total);

    // Show individual FATE results
    print!("   Individual: ");
    for (i, roll) in result.rolls.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        match roll.value {
            1 => print!("-"),
            2 => print!("0"),
            3 => print!("+"),
            _ => print!("?"),
        }
    }
    println!();

    // 2. Different numbers of FATE dice
    println!("\n2. Different FATE Dice Counts:");
    for count in [2, 4, 6, 8] {
        let fate = FateDice::new(count);
        let result = fate.roll();
        println!("   {} = {}", fate.describe(), result.total);
    }

    // 3. Expected values and ranges
    println!("\n3. FATE Dice Statistics:");
    println!("   Each dF has values: -1, 0, +1");
    println!("   Expected value per die: 0.0");
    println!("   Range for 4dF: -4 to +4");
    println!("   Most common result: 0 (37.5% chance)");

    // 4. Batch rolling for distribution
    println!("\n4. FATE Dice Distribution (1000 rolls):");
    let fate = FateDice::new(4);
    let results = noctisroll::utils::batch_roll(&fate, 1000);

    let mut distribution = std::collections::HashMap::new();
    for result in &results {
        *distribution.entry(result.total).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = distribution.iter().collect();
    sorted.sort_by_key(|(&total, _)| total);

    for (&total, &count) in sorted {
        let percentage = (count as f64 / 1000.0) * 100.0;
        println!("   {:3}: {:4} rolls ({:5.1}%)", total, count, percentage);
    }

    // 5. FATE ladder interpretation
    println!("\n5. FATE Ladder Interpretation:");
    let result = FateDice::new(4).roll();
    let total = result.total;

    let ladder = match total {
        i64::MIN..=-4 => "Terrible (-4)",
        -3 => "Poor (-3)",
        -2 => "Mediocre (-2)",
        -1 => "Average (-1)",
        0 => "Fair (0)",
        1 => "Good (+1)",
        2 => "Great (+2)",
        3 => "Superb (+3)",
        4..=i64::MAX => "Legendary (+4)",
    };

    println!("   Roll: {} = {}", result.description, total);
    println!("   On FATE ladder: {}", ladder);

    // 6. FATE with skills
    println!("\n6. FATE Skill Check:");
    let skill_level = 2; // Good (+2)
    let fate_roll = FateDice::new(4).roll();
    let total = skill_level + fate_roll.total;

    println!("   Skill: +{}", skill_level);
    println!(
        "   FATE roll: {} = {}",
        fate_roll.description, fate_roll.total
    );
    println!(
        "   Total: {} + {} = {}",
        skill_level, fate_roll.total, total
    );

    let outcome = match total {
        i64::MIN..=0 => "Failure",
        1..=2 => "Tie / Succeed at minor cost",
        3..=5 => "Success",
        6..=i64::MAX => "Success with style",
    };

    println!("   Outcome: {}", outcome);

    println!("\n=== FATE Examples Complete ===");
}
