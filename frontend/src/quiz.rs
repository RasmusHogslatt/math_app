use crate::{quizzes::*, util::validate_input};
use std::fmt::{self, Display};

// --- ADDING A NEW QUIZ CHECKLIST ---
// 1. Create `frontend/src/quizzes/your_quiz.rs` with the struct implementing `Question`.
// 2. Add a variant to the `Quiz` enum below. (e.g., `YourQuiz`)
// 3. Add a variant to the `QuestionBox` enum below. (e.g., `YourQuiz(YourQuizQuestion)`)
// 4. Update `generate_question` match statement (inside generate_questions).
// 5. Add the `Quiz::YourQuiz` variant to the `ALL_COURSES` list below.
// 6. (Optional) If custom UI needed, update `frontend/src/components/quiz_session.rs`.
// ------------------------------------

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
    SevenFirstOrderEquation,
    FirstDegreeDerivativeQuestion,
    NumberComparison,
    SixRounding,
    SixAverage,
    SixMedian,
    SevenPercentChange,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
            Quiz::SquareArea => write!(f, "Area"),
            Quiz::SevenFirstOrderEquation => {
                write!(f, "First Order Equation")
            }
            Quiz::FirstDegreeDerivativeQuestion => {
                write!(f, "First Degree Derivative")
            }
            Quiz::NumberComparison => write!(f, "Number Comparison"),
            Quiz::SixRounding => write!(f, "Rounding"),
            Quiz::SixAverage => write!(f, "Average"),
            Quiz::SixMedian => write!(f, "Median"),
            Quiz::SevenPercentChange => write!(f, "Percent Change"),
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
            "First Order Equation" => Ok(Quiz::SevenFirstOrderEquation),
            "First Degree Derivative" => Ok(Quiz::FirstDegreeDerivativeQuestion),
            "Number Comparison" => Ok(Quiz::NumberComparison),
            "Rounding" => Ok(Quiz::SixRounding),
            "Average" => Ok(Quiz::SixAverage),
            "Median" => Ok(Quiz::SixMedian),
            "Percent Change" => Ok(Quiz::SevenPercentChange),
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
            Quiz::SevenFirstOrderEquation => Difficulty::YearSeven,
            Quiz::FirstDegreeDerivativeQuestion => Difficulty::YearThree,
            Quiz::NumberComparison => Difficulty::YearTwo,
            Quiz::SixRounding => Difficulty::YearSix,
            Quiz::SixAverage => Difficulty::YearSix,
            Quiz::SixMedian => Difficulty::YearSix,
            Quiz::SevenPercentChange => Difficulty::YearSeven,
        }
    }
}

pub trait Question {
    fn prompt(&self) -> String;
    fn answer(&self) -> &str;
    fn check_answer(&self, answer: &str) -> bool {
        validate_input(self.answer(), answer)
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
    FirstOrderEquationQuestion(SevenFirstOrderEquationQuestion),
    FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion),
    NumberComparison(NumberComparisonQuestion),
    SixRounding(SixRoundingQuestion),
    SixAverage(SixAverageQuestion),
    SixMedian(SixMedianQuestion),
    SevenPercentChange(SevenPercentChangeQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Math(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.prompt(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.prompt(),
            QuestionBox::NumberComparison(q) => q.prompt(),
            QuestionBox::SixRounding(q) => q.prompt(),
            QuestionBox::SixAverage(q) => q.prompt(),
            QuestionBox::SixMedian(q) => q.prompt(),
            QuestionBox::SevenPercentChange(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Math(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.answer(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.answer(),
            QuestionBox::NumberComparison(q) => q.answer(),
            QuestionBox::SixRounding(q) => q.answer(),
            QuestionBox::SixAverage(q) => q.answer(),
            QuestionBox::SixMedian(q) => q.answer(),
            QuestionBox::SevenPercentChange(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Math(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
            QuestionBox::FirstOrderEquationQuestion(q) => q.check_answer(answer),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.check_answer(answer),
            QuestionBox::NumberComparison(q) => q.check_answer(answer),
            QuestionBox::SixRounding(q) => q.check_answer(answer),
            QuestionBox::SixAverage(q) => q.check_answer(answer),
            QuestionBox::SixMedian(q) => q.check_answer(answer),
            QuestionBox::SevenPercentChange(q) => q.check_answer(answer),
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
            Quiz::SevenFirstOrderEquation => {
                QuestionBox::FirstOrderEquationQuestion(SevenFirstOrderEquationQuestion::random())
            }
            Quiz::FirstDegreeDerivativeQuestion => {
                QuestionBox::FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion::random())
            }
            Quiz::NumberComparison => {
                QuestionBox::NumberComparison(NumberComparisonQuestion::random())
            }
            Quiz::SixRounding => QuestionBox::SixRounding(SixRoundingQuestion::random()),
            Quiz::SixAverage => QuestionBox::SixAverage(SixAverageQuestion::random()),
            Quiz::SixMedian => QuestionBox::SixMedian(SixMedianQuestion::random()),
            Quiz::SevenPercentChange => {
                QuestionBox::SevenPercentChange(SevenPercentChangeQuestion::random())
            }
        };

        questions.push(question);
    }

    questions
}
