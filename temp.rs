use rand::Rng; // Use rand::Rng for the gen_range method

use crate::{quiz::Question, util::format_to_one_decimal}; // Keep format_to_one_decimal for now, but we'll change how it's used

const AVERAGE_NUMBERS_COUNT_RANGE: (u32, u32) = (4, 6);
const NUMBER_VALUE_RANGE: (i32, i32) = (0, 20); // Define range for individual numbers (0-19)

#[derive(Clone, Debug, PartialEq)]
pub struct AverageQuestion {
    numbers: Vec<i32>,
    answer: String, // Storing as String, so an integer formatted to string is fine
}

impl AverageQuestion {
    pub fn new(numbers: Vec<i32>) -> Self {
        let sum: i32 = numbers.iter().sum();
        let count = numbers.len() as i32;

        // Check if the average is an integer. If not, this 'new' function
        // will still produce a decimal answer, unless you add a constraint here too.
        // For the purpose of 'random()' producing integers, this is fine.
        let average_val: f32;
        if count == 0 {
            average_val = 0.0; // Avoid division by zero
        } else {
            average_val = sum as f32 / count as f32;
        }

        Self {
            numbers,
            answer: format_to_one_decimal(average_val), // Still uses the utility for general new() calls
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng(); // FIX: Use rand::thread_rng()

        loop {
            let number_count: u32 =
                rng.gen_range(AVERAGE_NUMBERS_COUNT_RANGE.0..AVERAGE_NUMBERS_COUNT_RANGE.1);

            let numbers: Vec<i32> = (0..number_count)
                .map(|_| rng.gen_range(NUMBER_VALUE_RANGE.0..NUMBER_VALUE_RANGE.1))
                .collect();

            let sum: i32 = numbers.iter().sum();

            // Check if the sum is perfectly divisible by the count
            // Also ensure number_count is not zero to prevent division by zero
            if number_count > 0 && sum % (number_count as i32) == 0 {
                let integer_average: i32 = sum / (number_count as i32);

                // No need for format_to_one_decimal here if it's always an integer
                return Self {
                    numbers,
                    answer: integer_average.to_string(), // Store the integer directly as a string
                };
            }
            // If not divisible, the loop continues to generate new numbers
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

// Example of how `format_to_one_decimal` might look if you define it:
// (You'd need to provide `crate::util::format_to_one_decimal`)
/*
pub mod util {
    pub fn format_to_one_decimal(value: f32) -> String {
        format!("{:.1}", value)
    }
}
*/

// Small test to demonstrate (you can add this to your main.rs or a test file)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_average_is_integer() {
        for _ in 0..100 {
            // Generate 100 random questions
            let q = AverageQuestion::random();
            println!("Numbers: {:?}, Answer: {}", q.numbers, q.answer);

            // Attempt to parse the answer as an integer
            // If it contains a decimal, it will fail or truncate incorrectly
            let parsed_answer: i32 = q
                .answer
                .parse()
                .expect("Answer should be an integer string");

            // Verify the calculation manually
            let sum: i32 = q.numbers.iter().sum();
            let count = q.numbers.len() as i32;
            assert_eq!(parsed_answer, sum / count);
        }
    }
}
