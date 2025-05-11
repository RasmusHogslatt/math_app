use rand::Rng;

use crate::{quiz::Question, util::format_to_one_decimal};

const DENOMINATOR_RANGE: (i32, i32) = (1, 12);

#[derive(Clone, Debug, PartialEq)]
pub struct FractionToDegree {
    nominator: i32,
    denominator: i32,
    answer: String,
}

impl FractionToDegree {
    pub fn new(nominator: i32, denominator: i32) -> Self {
        let answer = (nominator as f32 / denominator as f32) * 360.0;
        Self {
            nominator,
            denominator,
            answer: format_to_one_decimal(answer as f32),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let denominator: i32 = rng.random_range(DENOMINATOR_RANGE.0..=DENOMINATOR_RANGE.1);
        let nominator: i32 = rng.random_range(1..=denominator);
        Self::new(nominator, denominator)
    }
}

impl Question for FractionToDegree {
    fn prompt(&self) -> String {
        format!(
            "Hur många grader är {}/{}-delar av en cirkel?",
            self.nominator, self.denominator
        )
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
