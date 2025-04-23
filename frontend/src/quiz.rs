use std::fmt::{self, Display};

use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
    SquareArea,
    FirstOrderEquationQuestion,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
            Quiz::SquareArea => write!(f, "Square Area"),
            Quiz::FirstOrderEquationQuestion => {
                write!(f, "First Order Equation")
            }
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
            "Square Area" => Ok(Quiz::SquareArea),
            "First Order Equation" => Ok(Quiz::FirstOrderEquationQuestion),
            _ => Ok(Quiz::NoCourse),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuestionBox {
    Math(MathQuestion),
    Area(AreaQuestion),
    FirstOrderEquationQuestion(FirstOrderEquationQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Math(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Math(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Math(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
            QuestionBox::FirstOrderEquationQuestion(q) => q.check_answer(answer),
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
            Quiz::NoCourse | Quiz::SquareArea | Quiz::FirstOrderEquationQuestion => 0,
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
            Quiz::NoCourse | Quiz::SquareArea | Quiz::FirstOrderEquationQuestion => {
                Self::new(0, 0, operation)
            }
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
            Quiz::NoCourse | Quiz::SquareArea | Quiz::FirstOrderEquationQuestion => {
                "Not applicable".to_string()
            }
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
            Quiz::FirstOrderEquationQuestion => {
                QuestionBox::FirstOrderEquationQuestion(FirstOrderEquationQuestion::random())
            }
        };

        questions.push(question);
    }

    questions
}
