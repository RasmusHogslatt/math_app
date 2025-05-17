use crate::quiz::*;
use rand::Rng;

const RANGE: (i32, i32) = (1, 10);

#[derive(Clone, Debug, PartialEq)]
pub struct MultiplicationQuestion1_10 {
    first_number: i32,
    second_number: i32,
    answer_text: String,
}

impl MultiplicationQuestion1_10 {
    pub fn new(first: i32, second: i32) -> Self {
        let result: f32 = (first * second) as f32;

        Self {
            first_number: first,
            second_number: second,
            answer_text: result.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(RANGE.0..RANGE.1),
            rng.random_range(RANGE.0..RANGE.1),
        )
    }
}

impl Question for MultiplicationQuestion1_10 {
    fn prompt(&self) -> String {
        format!("Beräkna {} × {}?", self.first_number, self.second_number)
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
