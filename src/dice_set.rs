use crate::{dice::{Dice, Operation}, dice_result::{DiceSetResults, RollResult}};

use rand::Rng;

#[derive(Debug)]
pub struct DiceSet {
    dice: Vec<Dice>,
}

impl DiceSet {
    pub fn new(dice: Vec<Dice>) -> Self {
        DiceSet { dice }
    }

    #[must_use]
    pub fn roll_dice_set(&self) -> DiceSetResults {
        let mut rng = rand::thread_rng();
        self.roll_dice_set_from_rng(&mut rng)
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn roll_dice_set_from_rng<R: Rng + Sized>(&self, mut rng: R) -> DiceSetResults {
        let results: Vec<RollResult> = self.dice.iter().map(|d| d.roll_dice_from_rng(&mut rng)).collect();
        let total = results.iter().enumerate().fold(0, |acc, (index, roll)| {
            match self.dice.get(index).unwrap().operation {
                Operation::Addition => acc + roll.result,
                Operation::Subtraction => acc - roll.result,
            }
        });

        DiceSetResults::new(results, total)
    }
}
