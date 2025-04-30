use rand::Rng;

use crate::quiz::Question;

const AVERAGE_NUMBERS_COUNT_RANGE: (u32, u32) = (4, 6);

#[derive(Clone, Debug, PartialEq)]
pub struct SixAverageQuestion {
    numbers: Vec<i32>,
    answer: String,
}

impl SixAverageQuestion {
    pub fn new(numbers: Vec<i32>) -> Self {
        let average: f32 = numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
        Self {
            numbers,
            answer: average.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let number_count: u32 =
            rng.random_range(AVERAGE_NUMBERS_COUNT_RANGE.0..AVERAGE_NUMBERS_COUNT_RANGE.1);
        let numbers: Vec<i32> = (0..number_count).map(|_| rng.random_range(0..20)).collect();
        Self::new(numbers)
    }
}

impl Question for SixAverageQuestion {
    fn prompt(&self) -> String {
        format!("What is the average of: {:?}", self.numbers)
    }

    fn answer(&self) -> &str {
        &self.answer
    }
}
