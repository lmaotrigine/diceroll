use crate::dice_result::RollResult;

use dice_parser::{dice_roll::Operation as CommandOperation, dice_roll::RollType as CommandRollType, dice_roll_with_op::DiceRollWithOp};
use rand::Rng;

use std::cmp::{max, min};

#[derive(PartialEq, Debug)]
pub struct Dice {
    pub number_of_dice_to_roll: u32,
    pub sides: u32,
    pub modifier: Option<i32>,
    pub roll_type: RollType,
    pub operation: Operation,
}

#[derive(PartialEq, Debug)]
pub enum RollType {
    Advantage,
    Disadvantage,
    Regular,
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Addition,
    Subtraction,
}

impl Dice {
    pub(crate) fn from_parsed_dice_roll(parsed_roll: &DiceRollWithOp) -> Self {
        let roll_type = match parsed_roll.dice_roll.roll_type {
            CommandRollType::Regular => RollType::Regular,
            CommandRollType::WithAdvantage => RollType::Advantage,
            CommandRollType::WithDisadvantage => RollType::Disadvantage,
        };

        let operation = match parsed_roll.operation {
            CommandOperation::Addition => Operation::Addition,
            CommandOperation::Subtraction => Operation::Subtraction,
        };

        Dice {
            number_of_dice_to_roll: parsed_roll.dice_roll.number_of_dice_to_roll,
            sides: parsed_roll.dice_roll.dice_sides,
            modifier: parsed_roll.dice_roll.modifier,
            roll_type,
            operation,
        }
    }

    #[must_use]
    pub fn new(number_of_dice: u32, number_of_sides: u32, modifier: Option<i32>, roll_type: RollType, operation: Operation) -> Self {
        Dice {
            number_of_dice_to_roll: number_of_dice,
            sides: number_of_sides,
            modifier,
            roll_type,
            operation,
        }
    }

    #[must_use]
    pub fn roll_dice(&self) -> RollResult {
        let mut rng = rand::thread_rng();
        self.roll_dice_from_rng(&mut rng)
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn roll_dice_from_rng<R: Rng + Sized>(&self, mut rng: R) -> RollResult {
        let current_roll_set_size = self.number_of_dice_to_roll as usize;
        let mut first_roll_results: Vec<u32> = Vec::with_capacity(current_roll_set_size);
        for _ in 0..self.number_of_dice_to_roll {
            first_roll_results.push(rng.gen_range(1..=self.sides));
        }

        let second_roll_results: Option<Vec<u32>> = match self.roll_type {
            RollType::Advantage | RollType::Disadvantage => {
                let mut second_roll_results: Vec<u32> = Vec::with_capacity(current_roll_set_size);
                for _ in 0..self.number_of_dice_to_roll {
                    second_roll_results.push(rng.gen_range(1..=self.sides));
                }
                Some(second_roll_results)
            }
            RollType::Regular => None,
        };

        let result = match self.roll_type {
            RollType::Regular => {
                first_roll_results.iter().sum::<u32>() as i32 + self.modifier.unwrap_or(0)
            }
            RollType::Advantage => {
                let modifier = self.modifier.unwrap_or(0);
                let first_result = first_roll_results.iter().sum::<u32>() as i32;
                let second_result = second_roll_results.as_ref().expect("Expect advantage roll to have second roll results").iter().sum::<u32>() as i32;
                max(first_result + modifier, second_result + modifier)
            }
            RollType::Disadvantage => {
                let modifier = self.modifier.unwrap_or(0);
                let first_result = first_roll_results.iter().sum::<u32>() as i32;
                let second_result = second_roll_results.as_ref().expect("Expect disadvantage roll to have second roll results").iter().sum::<u32>() as i32;
                min(first_result + modifier, second_result + modifier)
            }
        };

        RollResult::new(first_roll_results, second_roll_results, result)
    }
}
