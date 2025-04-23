use std::fmt::{self, Display};

use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
    SquareArea, // Modify this to do all square, rectangle, triangle and circle
    // Add percentages
    FirstOrderEquationQuestion,
    FirstDegreeDerivativeQuestion,
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
            Quiz::FirstDegreeDerivativeQuestion => {
                write!(f, "First Degree Derivative")
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
            "First Degree Derivative" => Ok(Quiz::FirstDegreeDerivativeQuestion),
            _ => Ok(Quiz::NoCourse),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuestionBox {
    Math(MathQuestion),
    Area(AreaQuestion),
    FirstOrderEquationQuestion(FirstOrderEquationQuestion),
    FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Math(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.prompt(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Math(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.answer(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Math(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
            QuestionBox::FirstOrderEquationQuestion(q) => q.check_answer(answer),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.check_answer(answer),
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
            Quiz::NoCourse
            | Quiz::SquareArea
            | Quiz::FirstOrderEquationQuestion
            | Quiz::FirstDegreeDerivativeQuestion => 0,
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
            Quiz::NoCourse
            | Quiz::SquareArea
            | Quiz::FirstOrderEquationQuestion
            | Quiz::FirstDegreeDerivativeQuestion => Self::new(0, 0, operation),
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
            Quiz::NoCourse
            | Quiz::SquareArea
            | Quiz::FirstOrderEquationQuestion
            | Quiz::FirstDegreeDerivativeQuestion => "Not applicable".to_string(),
        }
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FirstDegreeDerivativeQuestion {
    // Represent a polynomial function like ax^n + bx^m + c
    terms: Vec<(i32, u32)>, // Each term is (coefficient, exponent)
    constant: i32,          // Constant term
    answer_text: String,    // The derivative expression
}

impl FirstDegreeDerivativeQuestion {
    // Create a new derivative question with specific terms
    pub fn new(terms: Vec<(i32, u32)>, constant: i32) -> Self {
        // Calculate the derivative
        let derivative_terms: Vec<(i32, u32)> = terms
            .iter()
            .filter(|(_, exponent)| *exponent > 0) // Filter out constants (x^0)
            .map(|(coef, exp)| (coef * (*exp as i32), exp - 1)) // Apply power rule: d/dx(ax^n) = a*n*x^(n-1)
            .collect();

        // Format the answer as a string
        let answer = FirstDegreeDerivativeQuestion::format_polynomial(&derivative_terms, 0);

        Self {
            terms,
            constant,
            answer_text: answer,
        }
    }

    // Generate a random first-degree derivative question
    pub fn random() -> Self {
        let mut rng = rand::rng();

        // Generate between 1 and 3 terms
        let num_terms = rng.random_range(1..4);
        let mut terms = Vec::with_capacity(num_terms);

        // Generate terms with coefficients and exponents
        for _ in 0..num_terms {
            let coefficient = rng.random_range(-9..10);
            if coefficient == 0 {
                continue; // Skip zero terms
            }

            let exponent = rng.random_range(1..4); // Power from 1 to 3
            terms.push((coefficient, exponent));
        }

        // Add a constant term sometimes
        let constant = if rng.random_bool(0.7) {
            rng.random_range(-10..11)
        } else {
            0
        };

        Self::new(terms, constant)
    }

    // Helper method to format a polynomial expression
    // Updated format_polynomial method to handle zero constants correctly
    fn format_polynomial(terms: &[(i32, u32)], constant: i32) -> String {
        // If we have no terms and constant is zero, return "0"
        if terms.is_empty() && constant == 0 {
            return "0".to_string();
        }

        // If we have no terms but constant is non-zero, just return the constant
        if terms.is_empty() {
            return constant.to_string();
        }

        let mut result = String::new();
        let mut first_term = true;

        // Add each term to the result
        for (coef, exp) in terms {
            if *coef == 0 {
                continue; // Skip zero terms
            }

            // Handle the sign
            if first_term {
                if *coef < 0 {
                    result.push('-');
                }
            } else {
                if *coef < 0 {
                    result.push_str(" - ");
                } else {
                    result.push_str(" + ");
                }
            }

            // Add coefficient (unless it's 1 or -1 and not a constant)
            let abs_coef = coef.abs();
            if abs_coef != 1 || *exp == 0 {
                result.push_str(&abs_coef.to_string());
            }

            // Add variable and exponent
            match exp {
                0 => {}                                      // No variable for constant terms
                1 => result.push('x'),                       // Just 'x' for first degree
                _ => result.push_str(&format!("x^{}", exp)), // 'x^n' for higher degrees
            }

            first_term = false;
        }

        // Add constant if non-zero
        if constant != 0 {
            if !first_term {
                if constant < 0 {
                    result.push_str(" - ");
                    result.push_str(&constant.abs().to_string());
                } else {
                    result.push_str(" + ");
                    result.push_str(&constant.to_string());
                }
            } else {
                result.push_str(&constant.to_string());
            }
        }

        result
    }

    // Format the original polynomial
    fn format_original(&self) -> String {
        Self::format_polynomial(&self.terms, self.constant)
    }
}

impl Question for FirstDegreeDerivativeQuestion {
    fn prompt(&self) -> String {
        format!("Find the derivative of f(x) = {}", self.format_original())
    }

    fn answer(&self) -> &str {
        &self.answer_text
    }

    // Override to handle different equivalent forms of the answer
    fn check_answer(&self, answer: &str) -> bool {
        // This is a simple implementation that checks exact match
        // For a more robust solution, you'd want to parse and compare expressions algebraically

        // Normalize the answer by removing spaces and converting to lowercase
        let normalized_answer = answer.trim().to_lowercase().replace(" ", "");
        let normalized_correct = self.answer_text.to_lowercase().replace(" ", "");

        normalized_answer == normalized_correct
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
            Quiz::FirstDegreeDerivativeQuestion => {
                QuestionBox::FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion::random())
            }
        };

        questions.push(question);
    }

    questions
}
