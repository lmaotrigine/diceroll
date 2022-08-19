use std::fmt;

#[derive(PartialEq, Debug)]
pub struct DiceSetResults {
    pub dice_results: Vec<RollResult>,
    pub final_result: i32,
}

impl DiceSetResults {
    pub(crate) fn new(results: Vec<RollResult>, final_result: i32) -> Self {
        DiceSetResults {
            dice_results: results,
            final_result,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RollResult {
    pub first_roll: Vec<u32>,
    pub second_roll: Option<Vec<u32>>,
    pub result: i32,
}

impl RollResult {
    pub(crate) fn new(first_roll: Vec<u32>, second_roll: Option<Vec<u32>>, result: i32) -> Self {
        RollResult {
            first_roll,
            second_roll,
            result,
        }
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.second_roll {
            None => write!(f, "{:?}", self.first_roll),
            Some(second_roll) => write!(f, "[{:?}, {:?}]", self.first_roll, second_roll),
        }
    }
}
