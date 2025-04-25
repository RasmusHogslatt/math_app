use crate::quizzes::*;
use std::fmt::{self, Display};

// Can I add subtypes to the enum? For example Addition and Subtraction should be "easy"
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

impl Quiz {
    pub fn difficulty(&self) -> Difficulty {
        match self {
            Quiz::NoCourse => Difficulty::Easy,
            Quiz::Addition => Difficulty::Easy,
            Quiz::Subtraction => Difficulty::Easy,
            Quiz::Multiplication => Difficulty::Easy,
            Quiz::SquareArea => Difficulty::Medium,
            Quiz::FirstOrderEquationQuestion => Difficulty::Hard,
            Quiz::FirstDegreeDerivativeQuestion => Difficulty::Hard,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
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
