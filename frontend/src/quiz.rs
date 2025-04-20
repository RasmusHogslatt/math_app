// src/quiz.rs (or src/game/quiz.rs)
use rand::{prelude::*, rng, thread_rng}; // Use prelude for easier access
use std::fmt::{self, Display};

// Re-export needed types if they were in main.rs
// (Assuming Quiz and Question were defined here)

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)] // Added Eq, Hash if needed
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

// Convert from string if needed later
impl std::str::FromStr for Quiz {
    type Err = (); // Simple error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Addition" => Ok(Quiz::Addition),
            "Subtraction" => Ok(Quiz::Subtraction),
            "Multiplication" => Ok(Quiz::Multiplication),
            _ => Ok(Quiz::NoCourse), // Default or Err(())
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Question {
    pub first_operand: i32,
    pub second_operand: i32,
    pub operator: String,
    pub answer: i32,
}

impl Question {
    pub fn new(first: i32, second: i32, op: &str, ans: i32) -> Self {
        Question {
            first_operand: first,
            second_operand: second,
            operator: op.to_string(),
            answer: ans,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "{} {} {} = ?",
            self.first_operand, self.operator, self.second_operand
        )
    }
}

// Function to generate quiz questions based on the selected course
pub fn generate_questions(course: Quiz, count: usize) -> Vec<Question> {
    let mut rng = rng(); // Use thread_rng for convenience
    let mut questions = Vec::with_capacity(count);

    match course {
        Quiz::Addition => {
            for _ in 0..count {
                let a = rng.random_range(1..=5);
                let b = rng.random_range(1..=5);
                questions.push(Question::new(a, b, "+", a + b));
            }
        }
        Quiz::Subtraction => {
            for _ in 0..count {
                let a = rng.random_range(5..=6);
                let b = rng.random_range(1..=a); // Ensure b is not greater than a
                questions.push(Question::new(a, b, "-", a - b));
            }
        }
        Quiz::Multiplication => {
            for _ in 0..count {
                let a = rng.random_range(1..=2);
                let b = rng.random_range(1..=2);
                questions.push(Question::new(a, b, "*", a * b));
            }
        }
        Quiz::NoCourse => {
            // Return empty questions if no course is selected
        }
    }
    questions
}
