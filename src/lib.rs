lalrpop_util::lalrpop_mod!(pub roll);

use rand::{rngs::OsRng, Rng};

pub struct Dice {
    count: i32,
    sides: i32,
}

impl Dice {
    pub fn new(count: i32, sides: i32) -> Self {
        Self { count, sides }
    }

    pub fn roll(&self) -> i32 {
        let mut rng = OsRng;
        let mut sum = 0;
        for _ in 0..self.count {
            sum += rng.gen_range(1..=self.sides);
        }
        sum
    }
}

pub fn roll_dice(num_dice: i32, sides: i32) -> i32 {
    Dice::new(num_dice, sides).roll()
}
