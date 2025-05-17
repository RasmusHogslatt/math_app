use crate::quiz::*;
use rand::Rng;

const RANGE: (i32, i32) = (1, 10);

#[derive(Clone, Debug, PartialEq)]
pub struct DivisionQuestion1_10 {
    first_number: i32,
    second_number: i32,
    answer_text: String,
}

impl DivisionQuestion1_10 {
    pub fn new(first: i32, second: i32) -> Self {
        let result: f32 = first as f32 / second as f32;

        Self {
            first_number: first,
            second_number: second,
            answer_text: result.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let numerator = rng.random_range(RANGE.0..=RANGE.1);

        Self::new(numerator, rng.random_range(RANGE.0..=numerator))
    }
}

impl Question for DivisionQuestion1_10 {
    fn prompt(&self) -> String {
        format!("Beräkna {} / {}?", self.first_number, self.second_number)
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
