use std::fmt::{self, Display};

use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
    SquareArea,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
            Quiz::SquareArea => write!(f, "Square Area"),
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

#[derive(Clone, Debug, PartialEq)]
pub enum QuestionBox {
    Math(MathQuestion),
    Area(AreaQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Math(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Math(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Math(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
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
            Quiz::SquareArea => 0,
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
            Quiz::SquareArea => Self::new(0, 0, operation),
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
            Quiz::SquareArea => "Not applicable".to_string(),
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}
// Square area question implementation
#[derive(Clone, Debug, PartialEq)]
pub struct AreaQuestion {
    side_length: u32,
    unit: String,
    answer_text: String,
}

impl AreaQuestion {
    pub fn new(side: u32, unit: &str) -> Self {
        let area = side * side;
        Self {
            side_length: side,
            unit: unit.to_string(),
            answer_text: area.to_string(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let side = rng.random_range(1..20);
        let units = ["centimeters", "meters", "inches"];
        let unit = units[rng.random_range(0..units.len())];
        Self::new(side, unit)
    }
}

impl Question for AreaQuestion {
    fn prompt(&self) -> String {
        format!(
            "What is the area of a square with sides of {} {}?",
            self.side_length, self.unit
        )
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}

// Function to generate questions based on quiz type
pub fn generate_questions(quiz_type: Quiz, count: usize) -> Vec<QuestionBox> {
    let mut questions = Vec::with_capacity(count);

    for _ in 0..count {
        let question = match quiz_type {
            Quiz::Addition | Quiz::Subtraction | Quiz::Multiplication => {
                QuestionBox::Math(MathQuestion::random(quiz_type))
            }
            Quiz::SquareArea => QuestionBox::Area(AreaQuestion::random()),
            Quiz::NoCourse => QuestionBox::Math(MathQuestion::new(0, 0, Quiz::NoCourse)),
        };

        questions.push(question);
    }

    questions
}
