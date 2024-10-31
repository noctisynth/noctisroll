lalrpop_util::lalrpop_mod!(pub roll);

use lalrpop_util::{lexer::Token, ParseError};
use rand::{rngs::OsRng, Rng};

pub enum Filter {
    MaxN(u32),
    MinN(u32),
}

impl Filter {
    pub fn max(n: u32) -> Filter {
        Filter::MaxN(n)
    }

    pub fn min(n: u32) -> Filter {
        Filter::MinN(n)
    }

    pub fn filter(&self, points: &mut [u32]) -> Vec<u32> {
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
        }
    }
}

#[derive(Default)]
pub struct Dice {
    count: u32,
    sides: u32,
    rolled: Vec<u32>,
    filtered: Vec<u32>,
    result: u32,
    actions: Vec<Filter>,
    with_actions: bool,
    has_rolled: bool,
    has_filtered: bool,
}

impl Dice {
    pub fn new(count: u32, sides: u32) -> Self {
        Self {
            count,
            sides,
            ..Default::default()
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.actions.push(filter);
        self.with_actions = true;
        self
    }

    pub fn roll(mut self) -> Self {
        if !self.has_rolled {
            let mut rng = OsRng;
            for _ in 0..self.count {
                let roll = rng.gen_range(1..=self.sides);
                self.rolled.push(roll);
            }
        }
        self.has_rolled = true;

        if !self.has_filtered {
            self.filtered = self.rolled.clone();
            while let Some(action) = self.actions.pop() {
                self.filtered = action.filter(&mut self.filtered);
            }
            self.result = self.filtered.iter().sum();
        }
        self.has_filtered = true;

        self
    }

    pub fn roll_str(&self) -> String {
        let mut result = String::from("[");
        for (idx, roll) in self.rolled.iter().enumerate() {
            if idx == self.count as usize - 1 {
                result.push_str(&roll.to_string());
            } else {
                result.push_str(&roll.to_string());
                result.push_str(", ");
            }
        }
        result.push(']');
        if !self.with_actions {
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

pub fn roll_dice(num_dice: u32, sides: u32) -> u32 {
    Dice::new(num_dice, sides).roll().result
}

pub fn roll_inline<T: AsRef<str>>(
    input: T,
) -> Result<i32, ParseError<usize, Token<'static>, &'static str>> {
    Ok(roll::ExprParser::new()
        .parse(input.as_ref().to_string().leak())?
        .0)
}

pub fn roll<T: AsRef<str>>(
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
        assert_eq!(dice.rolled, vec![1, 1, 1, 1, 1, 1]);
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
        assert_eq!(roll_inline("6d1k3k2").unwrap(), 2);
        assert!(roll_inline("1k2").is_err())
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
        assert_eq!(dice.rolled, vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(dice.filtered, vec![1, 1, 1]);
        assert_eq!(dice.result, 3);
    }
}
