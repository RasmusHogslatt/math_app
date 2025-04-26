use crate::quiz::*;
use rand::Rng;

// Generic math question implementation
#[derive(Clone, Debug, PartialEq)]
pub struct MathQuestion {
    first_number: i32,
    second_number: i32,
    operation: Quiz,
    answer_text: String,
}

impl MathQuestion {
    pub fn new(first: i32, second: i32, operation: Quiz) -> Self {
        let result = match operation {
            Quiz::Addition => first + second,
            Quiz::Subtraction => first - second,
            Quiz::Multiplication => first * second,
            _ => 0,
        };

        Self {
            first_number: first,
            second_number: second,
            operation,
            answer_text: result.to_string(),
        }
    }

    pub fn random(operation: Quiz) -> Self {
        let mut rng = rand::rng();

        match operation {
            Quiz::Addition => {
                let a = rng.random_range(1..3);
                let b = rng.random_range(1..3);
                Self::new(a, b, operation)
            }
            Quiz::Subtraction => {
                // Ensure positive result for subtraction
                let b = rng.random_range(1..3);
                let a = rng.random_range(b..3);
                Self::new(a, b, operation)
            }
            Quiz::Multiplication => {
                let a = rng.random_range(1..2);
                let b = rng.random_range(1..2);
                Self::new(a, b, operation)
            }
            _ => Self::new(0, 0, operation),
        }
    }
}

impl Question for MathQuestion {
    fn prompt(&self) -> String {
        match self.operation {
            Quiz::Addition => format!("What is {} + {}?", self.first_number, self.second_number),
            Quiz::Subtraction => format!("What is {} - {}?", self.first_number, self.second_number),
            Quiz::Multiplication => {
                format!("What is {} × {}?", self.first_number, self.second_number)
            }
            _ => "Not applicable".to_string(),
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
