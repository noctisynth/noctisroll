lalrpop_util::lalrpop_mod!(pub roll);

use lalrpop_util::{lexer::Token, ParseError};
use rand::{rngs::OsRng, Rng};

pub struct Dice {
    count: usize,
    sides: i32,
    rolls: Vec<i32>,
    result: i32,
}

impl Dice {
    pub fn new(count: usize, sides: i32) -> Self {
        Self {
            count,
            sides,
            rolls: vec![],
            result: 0,
        }
    }

    pub fn roll(mut self) -> Self {
        let mut rng = OsRng;
        let mut sum = 0;
        for _ in 0..self.count {
            let roll = rng.gen_range(1..=self.sides);
            sum += roll;
            self.rolls.push(roll);
        }
        self.result = sum;
        self
    }

    pub fn roll_str(&self) -> String {
        let mut result = String::from("[");
        for (idx, roll) in self.rolls.iter().enumerate() {
            if idx == self.count - 1 {
                result.push_str(&roll.to_string());
            } else {
                result.push_str(&roll.to_string());
                result.push_str(", ");
            }
        }
        result.push_str("]");
        result
    }
}

pub fn roll_dice(num_dice: usize, sides: i32) -> i32 {
    Dice::new(num_dice, sides).roll().result
}

pub fn roll_inline<T: AsRef<str> + 'static>(
    input: T,
) -> Result<i32, ParseError<usize, Token<'static>, &'static str>> {
    Ok(roll::ExprParser::new()
        .parse(input.as_ref().to_string().leak())?
        .0)
}

pub fn roll<T: AsRef<str> + 'static>(
    input: T,
) -> Result<(i32, String), ParseError<usize, Token<'static>, &'static str>> {
    roll::ExprParser::new().parse(input.as_ref().to_string().leak())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice() {
        let dice = Dice::new(6, 1).roll();
        assert_eq!(dice.rolls, vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(dice.result, 6);
        assert_eq!(dice.roll_str(), "[1, 1, 1, 1, 1, 1]");
    }

    #[test]
    fn test_roll_dice() {
        assert_eq!(roll_dice(6, 1), 6);
        assert_eq!(roll_dice(3, 1), 3);
    }

    #[test]
    fn test_roll_inline() {
        assert_eq!(roll_inline("6d1").unwrap(), 6);
        assert_eq!(roll_inline("3d1+3d1").unwrap(), 6);
    }

    #[test]
    fn test_roll() {
        assert_eq!(roll("6d1").unwrap(), (6, "[1, 1, 1, 1, 1, 1]".to_string()));

        assert_eq!(roll("3d1+3d1").unwrap().1, "[1, 1, 1] + [1, 1, 1]");
    }
}
