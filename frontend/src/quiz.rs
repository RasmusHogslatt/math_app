use std::fmt::{self, Display};

use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
        }
    }
}

impl std::str::FromStr for Quiz {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Addition" => Ok(Quiz::Addition),
            "Subtraction" => Ok(Quiz::Subtraction),
            "Multiplication" => Ok(Quiz::Multiplication),
            _ => Ok(Quiz::NoCourse),
        }
    }
}

pub trait Question {
    fn prompt(&self) -> String;
    fn answer(&self) -> &str;
    fn check_answer(&self, answer: &str) -> bool {
        self.answer() == answer
    }

    // Helper method to display the question nicely
    fn display(&self) -> String {
        self.prompt()
    }
}

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
            Quiz::NoCourse => 0, // Default case
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
            Quiz::NoCourse => Self::new(0, 0, operation),
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
            Quiz::NoCourse => "No question available".to_string(),
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}

// Function to generate questions based on quiz type
pub fn generate_questions(quiz_type: Quiz, count: usize) -> Vec<MathQuestion> {
    let mut questions = Vec::with_capacity(count);

    for _ in 0..count {
        questions.push(MathQuestion::random(quiz_type));
    }

    questions
}
