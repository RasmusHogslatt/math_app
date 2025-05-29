use crate::quiz::Question;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct NegativeValuesQuestion {
    first_number: i32,
    second_number: i32,
    operation: char,
    answer_text: String,
}

impl NegativeValuesQuestion {
    pub fn new(first: i32, second: i32, operation: char) -> Self {
        let result = match operation {
            '+' => first + second,
            '-' => first - second,
            _ => panic!("Invalid operation: {}", operation),
        };
        Self {
            first_number: first,
            second_number: second,
            operation,
            answer_text: result.to_string(),
        }
    }

    pub fn random() -> Self {
        // Local constants for value ranges
        const LEFT_VAL_RANGE_ABS: i32 = 10; // Range for first number: -10 to 10
        const RIGHT_VAL_RANGE_ABS: i32 = 10; // Range for second number: -10 to 10

        let mut rng = rand::rng();

        // Randomly pick an operation
        let operation = if rng.random_bool(0.5) { '+' } else { '-' };

        let first: i32;
        let second: i32;

        match operation {
            '+' => {
                // Scenarios for addition with negative numbers
                match rng.random_range(0..3) {
                    0 => {
                        // Positive + Negative
                        first = rng.random_range(1..=LEFT_VAL_RANGE_ABS);
                        second = rng.random_range(-RIGHT_VAL_RANGE_ABS..=-1);
                    }
                    1 => {
                        // Negative + Positive
                        first = rng.random_range(-LEFT_VAL_RANGE_ABS..=-1);
                        second = rng.random_range(1..=RIGHT_VAL_RANGE_ABS);
                    }
                    _ => {
                        // Negative + Negative
                        first = rng.random_range(-LEFT_VAL_RANGE_ABS..=-1);
                        second = rng.random_range(-RIGHT_VAL_RANGE_ABS..=-1);
                    }
                }
            }
            '-' => {
                // Scenarios for subtraction involving negative numbers or resulting in negative
                match rng.random_range(0..4) {
                    0 => {
                        // Positive - Negative (e.g., 5 - (-3) = 5 + 3)
                        first = rng.random_range(1..=LEFT_VAL_RANGE_ABS);
                        second = rng.random_range(-RIGHT_VAL_RANGE_ABS..=-1);
                    }
                    1 => {
                        // Negative - Positive (e.g., -5 - 3 = -8)
                        first = rng.random_range(-LEFT_VAL_RANGE_ABS..=-1);
                        second = rng.random_range(1..=RIGHT_VAL_RANGE_ABS);
                    }
                    2 => {
                        // Negative - Negative (e.g., -5 - (-3) = -5 + 3 = -2)
                        first = rng.random_range(-LEFT_VAL_RANGE_ABS..=-1);
                        second = rng.random_range(-RIGHT_VAL_RANGE_ABS..=-1);
                    }
                    _ => {
                        // Positive - Larger Positive (to get a negative result, e.g., 3 - 5 = -2)
                        second = rng.random_range(2..=RIGHT_VAL_RANGE_ABS);
                        first = rng.random_range(1..second.min(LEFT_VAL_RANGE_ABS));
                    }
                }
            }
            _ => unreachable!("Invalid operation generated"),
        }
        Self::new(first, second, operation)
    }
}

impl Question for NegativeValuesQuestion {
    fn prompt(&self) -> String {
        // Format nicely: "5 + (-3)" or "5 - (-3)"
        if self.second_number < 0 {
            format!(
                "Beräkna {} {} ({})?",
                self.first_number, self.operation, self.second_number
            )
        } else {
            format!(
                "Beräkna {} {} {}?",
                self.first_number, self.operation, self.second_number
            )
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
