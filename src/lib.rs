lalrpop_util::lalrpop_mod!(pub roll);

use lalrpop_util::{lexer::Token, ParseError};
use rand::{rngs::OsRng, Rng};

pub enum Filter {
    MaxN(i32),
    MinN(i32),
    None,
}

impl Filter {
    pub fn max(n: i32) -> Filter {
        Filter::MaxN(n)
    }

    pub fn min(n: i32) -> Filter {
        Filter::MinN(n)
    }

    pub fn filter(&self, points: &mut [i32]) -> Vec<i32> {
        let mut points = points.to_vec();
        let length = points.len();
        match self {
            Filter::MaxN(n) => {
                points.sort_by(|a, b| b.cmp(a));
                points.split_at((*n as usize).min(length)).0.to_vec()
            }
            Filter::MinN(n) => {
                points.sort();
                points.split_at((*n as usize).min(length)).0.to_vec()
            }
            Filter::None => points,
        }
    }
}

pub struct Dice {
    count: usize,
    sides: i32,
    rolls: Vec<i32>,
    filtered: Vec<i32>,
    result: i32,
    filter: Filter,
}

impl Dice {
    pub fn new(count: usize, sides: i32) -> Self {
        Self {
            count,
            sides,
            rolls: vec![],
            filtered: vec![],
            result: 0,
            filter: Filter::None,
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    pub fn roll(mut self) -> Self {
        let mut rng = OsRng;
        for _ in 0..self.count {
            let roll = rng.gen_range(1..=self.sides);
            self.rolls.push(roll);
        }

        self.filtered = self.filter.filter(&mut self.rolls);

        let sum = self.filtered.iter().sum();
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
        result.push(']');
        if let Filter::None = self.filter {
            result
        } else {
            result.push('(');
            let filtered_count = self.filtered.len();
            for (idx, roll) in self.filtered.iter().enumerate() {
                if idx == filtered_count - 1 {
                    result.push_str(&roll.to_string());
                } else {
                    result.push_str(&roll.to_string());
                    result.push_str(", ");
                }
            }
            result.push(')');
            result
        }
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
        assert_eq!(roll_inline("-6d1").unwrap(), -6);
        assert_eq!(roll_inline("+6d1").unwrap(), 6);
        assert_eq!(roll_inline("3d1+3d1").unwrap(), 6);
        assert_eq!(roll_inline("3d1k2").unwrap(), 2);
    }

    #[test]
    fn test_roll() {
        assert_eq!(roll("6d1").unwrap(), (6, "[1, 1, 1, 1, 1, 1]".to_string()));
        assert_eq!(roll("3d1+3d1").unwrap().1, "[1, 1, 1] + [1, 1, 1]");
        assert_eq!(roll("3d1k2").unwrap().1, "[1, 1, 1](1, 1)");
    }

    #[test]
    fn test_math() {
        assert_eq!(roll_inline("min(6d1, 3d1)").unwrap(), 3);
        assert_eq!(roll_inline("max(6d1, 3d1)").unwrap(), 6);
        assert_eq!(roll_inline("-6d1").unwrap(), -6);
        assert_eq!(roll_inline("abs(-6d1)").unwrap(), 6);
    }

    #[test]
    fn test_filter() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Filter::min(3).filter(&mut arr), vec![1, 2, 3]);
        assert_eq!(Filter::max(3).filter(&mut arr), vec![6, 5, 4]);
        assert_eq!(Filter::min(0).filter(&mut arr), vec![]);

        let dice = Dice::new(6, 1).filter(Filter::max(3)).roll();
        assert_eq!(dice.rolls, vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(dice.filtered, vec![1, 1, 1]);
        assert_eq!(dice.result, 3);
    }
}
