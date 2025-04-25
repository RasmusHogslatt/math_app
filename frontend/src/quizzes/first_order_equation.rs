use rand::Rng;

use crate::quiz::Question;

#[derive(Clone, Debug, PartialEq)]
pub struct FirstOrderEquationQuestion {
    coefficient: i32,
    constant_left: i32,
    constant_right: i32,
    operation: char,
    answer_text: String,
}

impl FirstOrderEquationQuestion {
    pub fn new(coefficient: i32, constant_left: i32, constant_right: i32, operation: char) -> Self {
        // Calculate the answer (x value)
        let x = match operation {
            '+' => (constant_right - constant_left) as f64 / coefficient as f64,
            '-' => (constant_right + constant_left) as f64 / coefficient as f64,
            _ => 0.0, // Default case for unsupported operations
        };

        // Format the answer - if it's a whole number, show as integer
        let answer = if x.fract() == 0.0 {
            x.to_string().trim_end_matches(".0").to_string()
        } else {
            format!("{:.2}", x) // Show 2 decimal places for fractions
        };

        Self {
            coefficient,
            constant_left,
            constant_right,
            operation,
            answer_text: answer,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        // Generate values that will likely result in integer answers for simplicity
        let coefficient = rng.random_range(1..10); // Coefficient between 1-9
        let answer = rng.random_range(1..10); // The actual answer (x value)

        // Randomly choose operation (+ or -)
        let operations = ['+', '-'];
        let operation = operations[rng.random_range(0..operations.len())];

        // Generate the constant values based on the desired answer
        let constant_left = rng.random_range(1..15);
        let constant_right = match operation {
            '+' => (coefficient * answer) + constant_left,
            '-' => (coefficient * answer) - constant_left,
            _ => 0, // Won't happen with our limited operations
        };

        Self::new(coefficient, constant_left, constant_right, operation)
    }
}

impl Question for FirstOrderEquationQuestion {
    fn prompt(&self) -> String {
        // Format the equation nicely
        format!(
            "Solve for x: {}x {} {} = {}",
            self.coefficient, self.operation, self.constant_left, self.constant_right
        )
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }

    // Override to handle answers that could be formatted differently
    fn check_answer(&self, answer: &str) -> bool {
        // Try to parse the answer as a float for comparison
        if let Ok(user_answer) = answer.trim().parse::<f64>() {
            if let Ok(correct_answer) = self.answer_text.parse::<f64>() {
                // Allow small floating-point differences
                return (user_answer - correct_answer).abs() < 0.01;
            }
        }

        // Fall back to string comparison
        self.answer() == answer.trim()
    }
}
