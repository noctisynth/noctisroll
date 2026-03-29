//! Dice pool examples (infinite adding, double cross, etc.)

use noctisroll::prelude::*;

fn main() {
    println!("=== Dice Pool Examples ===\n");

    // 1. Infinite Adding Pool (World of Darkness style)
    println!("1. Infinite Adding Pool (WoD/Storyteller):");
    println!("   Format: AaBkCmE");
    println!("   A = initial dice, B = add threshold,");
    println!("   C = success threshold, E = sides\n");

    // Example: 5 dice, add on 8+, success on 6+, d10s
    let pool = InfiniteAddingPool::new(5, 8)
        .with_success_threshold(6)
        .with_sides(10);

    let result = pool.roll();
    println!("   Pool: {}", pool.describe());
    println!("   Result: {}", result.description);
    println!("   Successes: {}", result.success_count.unwrap());
    println!("   Failures: {}", result.failure_count.unwrap());
    println!("   Total dice rolled: {}", result.rolls.len());

    // Show progression
    println!("\n   Roll progression:");
    let mut current_batch = Vec::new();
    let mut batch_start = 0;

    while batch_start < result.rolls.len() {
        // Find batch size (look for rolls >= add threshold)
        let mut batch_end = batch_start;
        while batch_end < result.rolls.len() {
            if result.rolls[batch_end].value >= 8 {
                batch_end += 1;
            } else {
                break;
            }
        }

        if batch_end == batch_start {
            // No explosions, this is the final batch
            batch_end = result.rolls.len();
        }

        let batch = &result.rolls[batch_start..batch_end];
        let successes = batch.iter().filter(|r| r.value >= 6).count();

        println!(
            "   - Batch {}: {:?} ({} successes)",
            current_batch.len() + 1,
            batch.iter().map(|r| r.value).collect::<Vec<_>>(),
            successes
        );

        current_batch.extend_from_slice(batch);
        batch_start = batch_end;

        if batch_end == result.rolls.len() {
            break;
        }
    }

    // 2. Double Cross Pool
    println!("\n2. Double Cross Pool:");
    println!("   Format: AcBmC");
    println!("   A = initial dice, B = add threshold, C = sides\n");

    let pool = DoubleCrossPool::new(5, 8).with_sides(10);

    let result = pool.roll();
    println!("   Pool: {}", pool.describe());
    println!("   Result: {}", result.description);
    println!("   Total: {}", result.total);
    println!("   Total dice rolled: {}", result.rolls.len());

    // 3. Success-based dice pool
    println!("\n3. Success-Based Pool (Shadowrun, etc.):");
    println!("   Using standard dice in pool mode (a parameter)\n");

    let pool = StandardDice::new(10, 6).with_success_threshold(5); // Success on 5+

    let result = pool.roll();
    println!("   Pool: {}a{}", 10, 5);
    println!("   Result: {}", result.description);
    println!("   Successes: {}", result.success_count.unwrap());
    println!("   Individual rolls: {:?}", result.values());

    // Calculate probabilities
    println!("\n4. Pool Probability Analysis:");

    let pools = [
        (
            "Easy pool",
            InfiniteAddingPool::new(3, 8).with_success_threshold(6),
        ),
        (
            "Standard pool",
            InfiniteAddingPool::new(5, 8).with_success_threshold(6),
        ),
        (
            "Large pool",
            InfiniteAddingPool::new(8, 8).with_success_threshold(6),
        ),
        (
            "Hard pool",
            InfiniteAddingPool::new(5, 9).with_success_threshold(7),
        ),
    ];

    println!("   Running 1000 simulations each...");

    for (name, pool) in pools {
        let mut success_counts = Vec::new();

        for _ in 0..1000 {
            let result = pool.roll();
            success_counts.push(result.success_count.unwrap() as i32);
        }

        let avg_successes = success_counts.iter().sum::<i32>() as f64 / success_counts.len() as f64;
        let success_rate = success_counts.iter().filter(|&&c| c > 0).count() as f64
            / success_counts.len() as f64
            * 100.0;

        println!("   {}:", name);
        println!("     - Avg. successes: {:.2}", avg_successes);
        println!("     - Success rate: {:.1}%", success_rate);
        println!(
            "     - Max successes: {}",
            success_counts.iter().max().unwrap()
        );
    }

    // 5. Custom pool configurations
    println!("\n5. Custom Pool Configurations:");

    // Vampire: The Masquerade (difficulty variable)
    println!("   Vampire: The Masquerade");
    for difficulty in [6, 7, 8, 9] {
        let pool = StandardDice::new(7, 10).with_success_threshold(difficulty);
        let result = pool.roll();
        println!(
            "     {} dice, difficulty {}: {} successes",
            7,
            difficulty,
            result.success_count.unwrap()
        );
    }

    // Shadowrun (exploding 6s)
    println!("\n   Shadowrun (exploding 6s):");
    let pool = ExplodingDice::new(12, 6, 6); // 12d6, explode on 6
    let result = pool.roll();
    let successes = result.rolls.iter().filter(|r| r.value >= 5).count();
    println!(
        "     {}: {} successes ({} total dice)",
        pool.describe(),
        successes,
        result.rolls.len()
    );

    // 6. Pool comparison
    println!("\n6. Pool Type Comparison (5 dice):");

    let dice_count = 5;
    let sides = 10;
    let threshold = 8;

    let standard = StandardDice::new(dice_count, sides).with_success_threshold(threshold);
    let infinite = InfiniteAddingPool::new(dice_count, threshold)
        .with_success_threshold(threshold)
        .with_sides(sides);
    let double_cross = DoubleCrossPool::new(dice_count, threshold).with_sides(sides);

    let results = [
        ("Standard pool", standard.roll()),
        ("Infinite adding", infinite.roll()),
        ("Double cross", double_cross.roll()),
    ];

    for (name, result) in results {
        println!("   {}:", name);
        println!("     - Result: {}", result.description);

        if let Some(successes) = result.success_count {
            println!("     - Successes: {}", successes);
        }

        println!("     - Total: {}", result.total);
        println!("     - Dice rolled: {}", result.rolls.len());
    }

    println!("\n=== Pool Examples Complete ===");
}
