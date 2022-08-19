use std::str::FromStr;

use dice_parser::parse_line;
use rand::Rng;

use crate::{dice::Dice, dice_result::DiceSetResults, dice_set::DiceSet, error::DiceError};

#[derive(Debug)]
pub struct Roll {
    dice_sets: Vec<DiceSet>,
}

impl Roll {
    pub fn new(dice_sets: Vec<DiceSet>) -> Self {
        Roll { dice_sets }
    }

    #[must_use]
    pub fn roll(&self) -> Vec<DiceSetResults> {
        let mut rng = rand::thread_rng();
        self.roll_from_rng(&mut rng)
    }

    pub fn roll_from_rng<R: Rng + Sized>(&self, mut rng: R) -> Vec<DiceSetResults> {
        self.dice_sets.iter().map(|d| d.roll_dice_set_from_rng(&mut rng)).collect()
    }
}

impl FromStr for Roll {
    type Err = DiceError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let roll = parse_line(&input)?.iter().map(|dice| {
            DiceSet::new(dice.iter().map(|d| Dice::from_parsed_dice_roll(d)).collect())
        }).collect();
        Ok(Self::new(roll))
    }
}
