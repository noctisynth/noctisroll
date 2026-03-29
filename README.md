# NoctisRoll

A modern, modular TRPG dice rolling system implementing the [OneDice](https://github.com/OlivOS-Team/onedice) standard.

## Features

- **Full OneDice support**: Implements all dice types from the OneDice specification
- **Modular design**: Each dice type is implemented as a separate, reusable module
- **Type safety**: Strongly typed API with compile-time guarantees
- **Extensible**: Easy to add new dice types and operations
- **Performance**: Optimized for both single rolls and batch operations
- **Comprehensive error handling**: Detailed error types for all failure cases
- **Serialization support**: All core types support Serde serialization

## Supported Dice Types

- **Standard polyhedral dice** (`d`): `2d20`, `d6`, `4d10k3`, etc.
- **FATE/Fudge dice** (`f`/`df`): `4dF`, `8df`, etc.
- **Exploding dice** (`!`): `2d6!6`, `4d10!≥8`, etc.
- **Infinite adding pools** (`a`): `5a8k6m10` (World of Darkness style)
- **Double cross pools** (`c`): `5c8m10` (Double Cross style)
- **Bonus/penalty dice** (`p`/`b`): `d100p2`, `d20b1` (CoC style)
- **Success-based pools**: Count successes instead of summing

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
noctisroll = "0.2"
```

For parser support (optional):

```toml
[dependencies]
noctisroll = { version = "0.2", features = ["parser"] }
```

## Quick Start

### Basic Usage

```rust
use noctisroll::prelude::*;

// Roll 2d20 with advantage (keep highest)
let dice = StandardDice::new(2, 20).keep_highest(1);
let result = dice.roll();
println!("Roll: {}", result); // e.g., "2d20kh1 [17, 12] = 17"

// Roll FATE dice
let fate = FateDice::new(4);
let result = fate.roll();
println!("FATE roll: {}", result); // e.g., "4dF = +1"

// Roll exploding dice
let exploding = ExplodingDice::new(2, 6, 6); // Explode on 6
let result = exploding.roll();
println!("Exploding: {}", result); // e.g., "2d6!6 [6!, 4, 3] = 13"
```

### Using Convenience Functions

```rust
use noctisroll::dice::{d, f, x};

// Standard dice
let result = d(2, 20).keep_highest(1).roll();

// FATE dice
let result = f(4).roll();

// Exploding dice
let result = x(2, 6, 6).roll();
```

### Dice Pools

```rust
use noctisroll::dice::{InfiniteAddingPool, DoubleCrossPool};

// Infinite adding pool (World of Darkness style)
let pool = InfiniteAddingPool::new(5, 8) // 5 dice, add on 8+
    .with_success_threshold(6); // Success on 6+
let result = pool.roll();
println!("Successes: {}", result.success_count.unwrap());

// Double cross pool
let pool = DoubleCrossPool::new(5, 8); // 5 dice, add on 8+
let result = pool.roll();
println!("Total: {}", result.total);
```

### Parsing Dice Expressions (Optional Feature)

```rust
use noctisroll::prelude::*;

// Enable parser feature in Cargo.toml first
let expr = "2d20kh1 + 5";
let result = noctisroll::eval(expr).unwrap();
println!("Result: {}", result); // e.g., "2d20kh1 + 5 = 22"

// Or parse without evaluating
let dice = noctisroll::parser::Parser::new().parse("4d6k3").unwrap();
let result = dice.roll();
```

### Advanced Usage

```rust
use noctisroll::prelude::*;
use noctisroll::utils::{RollStatistics, format_detailed};

// Batch rolling
let dice = StandardDice::new(1, 20);
let results = noctisroll::utils::batch_roll(&dice, 100);

// Calculate statistics
let stats = RollStatistics::from_rolls(
    &results.iter().flat_map(|r| &r.rolls).cloned().collect::<Vec<_>>()
);
println!("Average: {:.2}", stats.mean);
println!("Critical rate: {:.1}%", 
    stats.critical_successes as f64 / results.len() as f64 * 100.0);

// Format detailed output
for result in &results[..3] {
    println!("{}", format_detailed(result));
}
```

## API Overview

### Core Types

- `Dice`: Trait for all dice types
- `ModifiableDice`: Trait for dice that support operations (keep, drop, explode, etc.)
- `DieRoll`: Represents a single die roll with metadata
- `RollResult`: Complete result of a dice roll
- `DiceContext`: Context for rolling with configuration

### Error Handling

```rust
use noctisroll::error::{DiceError, DiceResult};

fn roll_safely(dice: &dyn Dice) -> DiceResult<RollResult> {
    // All dice operations return Result types
    Ok(dice.roll())
}

match roll_safely(&dice) {
    Ok(result) => println!("Success: {}", result),
    Err(DiceError::InvalidDice(msg)) => println!("Invalid dice: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

### Configuration

```rust
use noctisroll::core::{DiceConfig, DiceContext};

let config = DiceConfig {
    default_sides: 100, // CoC default instead of D&D
    detect_criticals: true,
    seed: Some(42), // Fixed seed for reproducible rolls
    max_dice: 10000,
    max_sides: 1000,
};

let mut ctx = DiceContext::with_config(config);
let roll = ctx.roll_die(20);
```

## OneDice Compatibility

NoctisRoll implements the complete OneDice V1 specification:

### Supported Operators
- `d`: Standard polyhedral dice
- `a`: Infinite adding pool
- `c`: Double cross pool  
- `f`/`df`: FATE/Fudge dice
- `k`/`q`: Keep highest/lowest
- `p`/`b`: Bonus/penalty dice
- `!`: Exploding dice
- `a`: Success threshold (dice pool mode)

### Supported Expressions
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
- Comparisons: `>`, `<`, `>=`, `<=`, `==`, `!=`
- Ternary: `? :`
- Parentheses for grouping
- Functions: `min()`, `max()`, `abs()`, etc.

## Examples

See the `examples/` directory for more complete examples:

```bash
cargo run --example basic
cargo run --example fate
cargo run --example pool
cargo run --features parser --example parse
```

## Performance

NoctisRoll is optimized for performance:

- Zero allocations for simple dice rolls
- Lazy evaluation where possible
- Batch operations for multiple rolls
- Thread-safe implementation

Benchmarks are available in the `benches/` directory.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

AGPL-3.0-only - See [LICENSE](LICENSE) for details.

## Acknowledgments

- [OneDice](https://github.com/OlivOS-Team/onedice) for the comprehensive standard
- All TRPG communities for inspiring this library
- Contributors and testers