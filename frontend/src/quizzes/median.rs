use rand::Rng;

use crate::{quiz::Question, util::format_to_one_decimal};

const MEDIAN_NUMBERS_COUNT_RANGE: (u32, u32) = (4, 6);
const NUMBER_VALUE_RANGE: (i32, i32) = (0, 20);

#[derive(Clone, Debug, PartialEq)]
pub struct MedianQuestion {
    numbers: Vec<i32>,
    answer: String,
}

impl MedianQuestion {
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
        loop {
            let number_count: u32 =
                rng.random_range(MEDIAN_NUMBERS_COUNT_RANGE.0..MEDIAN_NUMBERS_COUNT_RANGE.1);
            let numbers: Vec<i32> = (0..number_count)
                .map(|_| rng.random_range(NUMBER_VALUE_RANGE.0..NUMBER_VALUE_RANGE.1))
                .collect();

            let mut sorted_numbers = numbers.clone();
            sorted_numbers.sort_unstable();
            let len = sorted_numbers.len();

            let integer_median = if len % 2 == 0 {
                // For even count, check if the two middle numbers sum to an even number
                let middle_sum = sorted_numbers[len / 2 - 1] + sorted_numbers[len / 2];
                if middle_sum % 2 == 0 {
                    Some(middle_sum / 2)
                } else {
                    None
                }
            } else {
                // For odd count, median is always an integer (the middle element)
                Some(sorted_numbers[len / 2])
            };

            if let Some(median_value) = integer_median {
                return Self {
                    numbers,
                    answer: median_value.to_string(),
                };
            }
        }
    }
}

impl Question for MedianQuestion {
    fn prompt(&self) -> String {
        format!("Beräkna medianen: {:?}", self.numbers)
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
