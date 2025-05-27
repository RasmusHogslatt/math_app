use rand::Rng;

use crate::{quiz::Question, util::format_to_one_decimal};

const AVERAGE_NUMBERS_COUNT_RANGE: (u32, u32) = (4, 6);
const NUMBER_VALUE_RANGE: (i32, i32) = (0, 20);

#[derive(Clone, Debug, PartialEq)]
pub struct AverageQuestion {
    numbers: Vec<i32>,
    answer: String,
}

impl AverageQuestion {
    pub fn new(numbers: Vec<i32>) -> Self {
        let average: f32 = numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
        Self {
            numbers,
            answer: format_to_one_decimal(average),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        loop {
            let number_count: u32 =
                rng.random_range(AVERAGE_NUMBERS_COUNT_RANGE.0..AVERAGE_NUMBERS_COUNT_RANGE.1);
            let numbers: Vec<i32> = (0..number_count)
                .map(|_| rng.random_range(NUMBER_VALUE_RANGE.0..NUMBER_VALUE_RANGE.1))
                .collect();
            let sum: i32 = numbers.iter().sum();
            if number_count > 0 && sum % (number_count as i32) == 0 {
                let integer_average: i32 = sum / (number_count as i32);
                return Self {
                    numbers,
                    answer: integer_average.to_string(),
                };
            }
        }
    }
}

impl Question for AverageQuestion {
    fn prompt(&self) -> String {
        format!("Beräkna medelvärdet: {:?}", self.numbers)
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
