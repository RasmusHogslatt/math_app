use crate::quizzes::*;
use std::fmt::{self, Display};

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash, Ord, PartialOrd)]
pub enum Difficulty {
    YearOne,
    YearTwo,
    YearThree,
    YearFour,
    YearFive,
    YearSix,
    YearSeven,
    YearEight,
    YearNine,
}

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
    SquareArea,
    FirstOrderEquationQuestion,
    FirstDegreeDerivativeQuestion,
    NumberComparison,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
            Quiz::SquareArea => write!(f, "Area"),
            Quiz::FirstOrderEquationQuestion => {
                write!(f, "First Order Equation")
            }
            Quiz::FirstDegreeDerivativeQuestion => {
                write!(f, "First Degree Derivative")
            }
            Quiz::NumberComparison => write!(f, "Number Comparison"),
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
            Quiz::NoCourse => Difficulty::YearOne,
            Quiz::Addition => Difficulty::YearOne,
            Quiz::Subtraction => Difficulty::YearOne,
            Quiz::Multiplication => Difficulty::YearOne,
            Quiz::SquareArea => Difficulty::YearTwo,
            Quiz::FirstOrderEquationQuestion => Difficulty::YearThree,
            Quiz::FirstDegreeDerivativeQuestion => Difficulty::YearThree,
            Quiz::NumberComparison => Difficulty::YearTwo,
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

#[derive(Clone, Debug, PartialEq)]
pub enum QuestionBox {
    Math(MathQuestion),
    Area(AreaQuestion),
    FirstOrderEquationQuestion(FirstOrderEquationQuestion),
    FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion),
    NumberComparison(NumberComparisonQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Math(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.prompt(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.prompt(),
            QuestionBox::NumberComparison(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Math(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.answer(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.answer(),
            QuestionBox::NumberComparison(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Math(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
            QuestionBox::FirstOrderEquationQuestion(q) => q.check_answer(answer),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.check_answer(answer),
            QuestionBox::NumberComparison(q) => q.check_answer(answer),
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
            Quiz::NumberComparison => {
                QuestionBox::NumberComparison(NumberComparisonQuestion::random())
            }
        };

        questions.push(question);
    }

    questions
}
