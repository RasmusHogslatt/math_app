use rand::Rng;

use crate::{quiz::Question, util::format_to_one_decimal};

const MEDIAN_NUMBERS_COUNT_RANGE: (u32, u32) = (4, 6);

#[derive(Clone, Debug, PartialEq)]
pub struct SixMedianQuestion {
    numbers: Vec<i32>,
    answer: String,
}

impl SixMedianQuestion {
    pub fn new(numbers: Vec<i32>) -> Self {
        let median: f32 = {
            let mut sorted_numbers = numbers.clone();
            sorted_numbers.sort_unstable();
            let len = sorted_numbers.len();
            if len % 2 == 0 {
                (sorted_numbers[len / 2 - 1] + sorted_numbers[len / 2]) as f32 / 2.0
            } else {
                sorted_numbers[len / 2] as f32
            }
        };
        Self {
            numbers,
            answer: format_to_one_decimal(median),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let number_count: u32 =
            rng.random_range(MEDIAN_NUMBERS_COUNT_RANGE.0..MEDIAN_NUMBERS_COUNT_RANGE.1);
        let numbers: Vec<i32> = (0..number_count).map(|_| rng.random_range(0..20)).collect();
        Self::new(numbers)
    }
}

impl Question for SixMedianQuestion {
    fn prompt(&self) -> String {
        format!("What is the median of: {:?}", self.numbers)
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
