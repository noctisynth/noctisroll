# AGENTS.md - NoctisRoll Development Guide

This document provides essential information for AI agents working on the NoctisRoll project.

## Project Overview

NoctisRoll is a modern, modular TRPG dice rolling system implementing the OneDice standard. It's written in Rust with a focus on type safety, performance, and extensibility.

## Build Commands

### Basic Build & Test
```bash
# Build the project
cargo build

# Build with all features (including parser)
cargo build --all-features

# Run all tests
cargo test --all --all-features --all-targets

# Run tests with verbose output
cargo test --all --all-features --all-targets --verbose
```

### Running Specific Tests
```bash
# Run a specific test file
cargo test --test basic

# Run a specific test by name
cargo test test_standard_dice

# Run tests with pattern matching
cargo test --test '*' -- --test-threads=1

# Run integration tests
cargo test --tests
```

### Development Builds
```bash
# Debug build (default)
cargo build

# Release build (optimized)
cargo build --release

# Check for compilation errors without building
cargo check

# Check with all features
cargo check --all-features
```

## Lint & Format Commands

### Code Quality
```bash
# Run clippy (Rust linter)
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting without applying
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Spell checking (requires cspell)
npx cspell "**" --config .cspell.json
```

### CI/CD Commands
The project uses GitHub Actions for CI. Local equivalents:
```bash
# Full CI check (build + test + lint)
cargo build --verbose
cargo test --all --all-features --all-targets --verbose
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Code Style Guidelines

### Imports & Modules
```rust
// Group imports: std, external crates, internal modules
use std::fmt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Internal imports use crate:: prefix
use crate::core::{Dice, DiceContext, DieRoll, RollResult};
use crate::error::{DiceError, DiceResult};

// Module declarations
pub mod core;
pub mod dice;
pub mod error;
pub mod utils;
```

### Naming Conventions
- **Structs/Traits/Enums**: `PascalCase` (e.g., `StandardDice`, `DiceError`)
- **Functions/Methods**: `snake_case` (e.g., `keep_highest`, `roll`)
- **Variables/Fields**: `snake_case` (e.g., `is_critical_success`, `bonus_dice`)
- **Constants**: `SCREAMING_SNAKE_CASE` (not commonly used in this codebase)
- **Type parameters**: `PascalCase` single letters (e.g., `T`, `E`)

### Error Handling
- Use `thiserror` crate for error definitions
- Error types implement `Error`, `Debug`, `Clone`, `PartialEq`
- Return `DiceResult<T>` alias for `Result<T, DiceError>`
- Provide descriptive error messages with context

Example:
```rust
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DiceError {
    #[error("Invalid dice: {0}")]
    InvalidDice(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type DiceResult<T> = Result<T, DiceError>;
```

### Type Definitions
- Use `u32` for dice counts and values (never negative)
- Use `i32` for bonuses/modifiers (can be negative)
- Mark public API with `pub` explicitly
- Derive common traits: `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`

Example:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DieRoll {
    pub value: u32,
    pub sides: u32,
    pub is_critical_success: bool,
    pub is_critical_failure: bool,
}
```

### Documentation
- Use `//!` for module-level documentation
- Use `///` for item-level documentation
- Include examples in documentation when helpful
- Document public API thoroughly

Example:
```rust
//! Core types and traits for the dice rolling system

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
```

### Testing Patterns
- Tests go in `tests/` directory for integration tests
- Unit tests can be in same file with `#[cfg(test)]`
- Use `proptest` for property-based testing
- Test edge cases thoroughly (min/max values, zero, etc.)

Example test structure:
```rust
#[test]
fn test_standard_dice() {
    let dice = StandardDice::new(2, 6);
    let result = dice.roll();
    
    assert!(result.total >= 2 && result.total <= 12);
    assert_eq!(result.rolls.len(), 2);
}
```

### Feature Flags
- `parser` feature enables dice expression parsing
- Always use `#[cfg(feature = "parser")]` for parser-related code
- Default features include `parser`

Example:
```rust
#[cfg(feature = "parser")]
pub mod parser;

#[cfg(feature = "parser")]
pub fn eval(expr: &str) -> Result<RollResult, DiceError> {
    // implementation
}
```

## Project Structure
```
noctisroll/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── core.rs         # Core types and traits
│   ├── error.rs        # Error definitions
│   ├── dice/           # Dice implementations
│   │   ├── mod.rs
│   │   ├── standard.rs
│   │   ├── fate.rs
│   │   └── ...
│   └── parser.rs       # Dice expression parser
├── tests/
│   └── basic.rs        # Integration tests
├── examples/           # Example usage
├── Cargo.toml         # Project configuration
└── .cspell.json       # Spell checking config
```

## Workflow Guidelines

1. **Before making changes**:
   - Run `cargo check` to ensure clean state
   - Run existing tests: `cargo test`

2. **During development**:
   - Write tests for new functionality
   - Use `cargo test --test <name>` to run specific tests
   - Format code: `cargo fmt --all`

3. **Before committing**:
   - Run full test suite: `cargo test --all --all-features --all-targets`
   - Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
   - Check formatting: `cargo fmt --all -- --check`
   - Verify no spelling errors

4. **Performance considerations**:
   - Use `criterion` for benchmarks (dev-dependency)
   - Profile with `cargo bench` when needed
   - Consider memory usage for batch operations

## Common Pitfalls to Avoid

1. **Integer overflow**: Use checked arithmetic for dice operations
2. **Panic avoidance**: Return `Result` instead of panicking
3. **Feature gating**: Don't forget `#[cfg(feature = "parser")]` for parser code
4. **Serialization**: Ensure all public types derive `Serialize`/`Deserialize` when appropriate
5. **Error messages**: Provide clear, actionable error messages

## Tool Versions
- Rust: 1.94.0+ (edition 2021)
- Cargo: 1.94.0+
- Clippy: Latest stable
- Rustfmt: Latest stable

## Additional Resources
- [OneDice Standard](https://github.com/noctisynth/noctisroll) - Dice specification
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - General Rust best practices
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo reference