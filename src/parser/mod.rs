//! Parser for dice expressions

#[cfg(feature = "parser")]
mod pest_parser;

#[cfg(feature = "parser")]
pub use pest_parser::*;

/// Parser for dice expressions
#[derive(Debug, Clone)]
pub struct Parser {
    /// Parser configuration
    config: ParserConfig,
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Default number of sides for 'd' without explicit sides
    pub default_sides: u32,
    /// Whether to allow unlimited dice pools
    pub allow_unlimited_pools: bool,
    /// Maximum expression depth
    pub max_depth: u32,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            default_sides: 20, // D&D default
            allow_unlimited_pools: false,
            max_depth: 100,
        }
    }
}

impl Parser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a new parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse and evaluate a dice expression
    #[cfg(feature = "parser")]
    pub fn eval(&self, expr: &str) -> Result<crate::core::RollResult, crate::error::DiceError> {
        use pest_parser::PestParser;

        let parser = PestParser::new(self.config.clone());
        parser.eval(expr)
    }

    /// Parse a dice expression without evaluating it
    #[cfg(feature = "parser")]
    pub fn parse(&self, expr: &str) -> Result<Box<dyn crate::core::Dice>, crate::error::DiceError> {
        use pest_parser::PestParser;

        let parser = PestParser::new(self.config.clone());
        parser.parse(expr)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
