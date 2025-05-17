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
pub enum Subject {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Number,
    Geometry,
    Statisitics,
    Algebra,
    Random,
}

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Quiz {
    NoCourse,
    Addition1_10,
    Addition100,
    Subtraction1_10,
    Multiplication1_10,
    Multiplication1_20,
    Division1_10,
    SquareArea,
    FirstOrderEquation,
    FirstDegreeDerivativeQuestion,
    NumberComparison,
    FractionComparison,
    Rounding,
    Average,
    Median,
    FractionToDegree,
    PercentChange,
    Expression,
}

impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "Välj en quiz"),
            Quiz::Addition1_10 => write!(f, "Addition 1-10"),
            Quiz::Addition100 => write!(f, "Addition 100-999"),
            Quiz::Subtraction1_10 => write!(f, "Subtraktion 1-10"),
            Quiz::Multiplication1_10 => write!(f, "Multiplikation 1-10"),
            Quiz::Multiplication1_20 => write!(f, "Multiplikation 1-20"),
            Quiz::Division1_10 => write!(f, "Division 1-10"),
            Quiz::SquareArea => write!(f, "Area"),
            Quiz::FirstOrderEquation => {
                write!(f, "Första ordningens ekvation")
            }
            Quiz::FirstDegreeDerivativeQuestion => {
                write!(f, "Första gradens derivata")
            }
            Quiz::NumberComparison => write!(f, "Störst värde"),
            Quiz::FractionComparison => write!(f, "Störst bråk"),
            Quiz::Rounding => write!(f, "Avrundning"),
            Quiz::Average => write!(f, "Medelvärde"),
            Quiz::Median => write!(f, "Median"),
            Quiz::FractionToDegree => write!(f, "Bråk till grader"),
            Quiz::PercentChange => write!(f, "Procentuell förändring"),
            Quiz::Expression => write!(f, "Matematiska uttryck"),
        }
    }
}

impl Quiz {
    pub fn subject(&self) -> Subject {
        match self {
            Quiz::NoCourse => Subject::Addition,
            Quiz::Addition1_10 => Subject::Addition,
            Quiz::Addition100 => Subject::Addition,
            Quiz::Subtraction1_10 => Subject::Subtraction,
            Quiz::Multiplication1_10 => Subject::Multiplication,
            Quiz::Multiplication1_20 => Subject::Multiplication,
            Quiz::Division1_10 => Subject::Division,
            Quiz::SquareArea => Subject::Geometry,
            Quiz::FirstOrderEquation => Subject::Algebra,
            Quiz::FirstDegreeDerivativeQuestion => Subject::Algebra,
            Quiz::NumberComparison => Subject::Number,
            Quiz::FractionComparison => Subject::Number,
            Quiz::Rounding => Subject::Number,
            Quiz::Average => Subject::Statisitics,
            Quiz::Median => Subject::Statisitics,
            Quiz::FractionToDegree => Subject::Geometry,
            Quiz::PercentChange => Subject::Statisitics,
            Quiz::Expression => Subject::Algebra,
        }
    }

    pub fn number_of_questions(&self) -> usize {
        match self {
            Quiz::NoCourse => 1,
            Quiz::Addition1_10 => 10,
            Quiz::Addition100 => 10,
            Quiz::Subtraction1_10 => 10,
            Quiz::Multiplication1_10 => 10,
            Quiz::Multiplication1_20 => 10,
            Quiz::Division1_10 => 10,
            Quiz::SquareArea => 10,
            Quiz::FirstOrderEquation => 10,
            Quiz::FirstDegreeDerivativeQuestion => 10,
            Quiz::NumberComparison => 10,
            Quiz::FractionComparison => 10,
            Quiz::Rounding => 10,
            Quiz::Average => 10,
            Quiz::Median => 10,
            Quiz::FractionToDegree => 10,
            Quiz::PercentChange => 10,
            Quiz::Expression => 10,
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
    Addition1_10(AdditionQuestion1_10),
    Addition100(AdditionQuestion100),
    Subtraction1_10(SubtractionQuestion1_10),
    Multiplication1_10(MultiplicationQuestion1_10),
    Multiplication1_20(MultiplicationQuestion1_20),
    Division1_10(DivisionQuestion1_10),
    Area(AreaQuestion),
    FirstOrderEquationQuestion(FirstOrderEquationQuestion),
    FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion),
    NumberComparison(NumberComparisonQuestion),
    FractionComparison(FractionComparisonQuestion),
    SixRounding(RoundingQuestion),
    SixAverage(AverageQuestion),
    SixMedian(MedianQuestion),
    SixFractionToDegree(FractionToDegree),
    SevenPercentChange(PercentChangeQuestion),
    EightExpression(ExpressionQuestion),
}

impl Question for QuestionBox {
    fn prompt(&self) -> String {
        match self {
            QuestionBox::Addition1_10(q) => q.prompt(),
            QuestionBox::Addition100(q) => q.prompt(),
            QuestionBox::Subtraction1_10(q) => q.prompt(),
            QuestionBox::Multiplication1_10(q) => q.prompt(),
            QuestionBox::Multiplication1_20(q) => q.prompt(),
            QuestionBox::Division1_10(q) => q.prompt(),
            QuestionBox::Area(q) => q.prompt(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.prompt(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.prompt(),
            QuestionBox::NumberComparison(q) => q.prompt(),
            QuestionBox::FractionComparison(q) => q.prompt(),
            QuestionBox::SixRounding(q) => q.prompt(),
            QuestionBox::SixAverage(q) => q.prompt(),
            QuestionBox::SixMedian(q) => q.prompt(),
            QuestionBox::SixFractionToDegree(q) => q.prompt(),
            QuestionBox::SevenPercentChange(q) => q.prompt(),
            QuestionBox::EightExpression(q) => q.prompt(),
        }
    }

    fn answer(&self) -> &str {
        match self {
            QuestionBox::Addition1_10(q) => q.answer(),
            QuestionBox::Addition100(q) => q.answer(),
            QuestionBox::Subtraction1_10(q) => q.answer(),
            QuestionBox::Multiplication1_10(q) => q.answer(),
            QuestionBox::Multiplication1_20(q) => q.answer(),
            QuestionBox::Division1_10(q) => q.answer(),
            QuestionBox::Area(q) => q.answer(),
            QuestionBox::FirstOrderEquationQuestion(q) => q.answer(),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.answer(),
            QuestionBox::NumberComparison(q) => q.answer(),
            QuestionBox::FractionComparison(q) => q.answer(),
            QuestionBox::SixRounding(q) => q.answer(),
            QuestionBox::SixAverage(q) => q.answer(),
            QuestionBox::SixMedian(q) => q.answer(),
            QuestionBox::SixFractionToDegree(q) => q.answer(),
            QuestionBox::SevenPercentChange(q) => q.answer(),
            QuestionBox::EightExpression(q) => q.answer(),
        }
    }

    fn check_answer(&self, answer: &str) -> bool {
        match self {
            QuestionBox::Addition1_10(q) => q.check_answer(answer),
            QuestionBox::Addition100(q) => q.check_answer(answer),
            QuestionBox::Subtraction1_10(q) => q.check_answer(answer),
            QuestionBox::Multiplication1_10(q) => q.check_answer(answer),
            QuestionBox::Multiplication1_20(q) => q.check_answer(answer),
            QuestionBox::Division1_10(q) => q.check_answer(answer),
            QuestionBox::Area(q) => q.check_answer(answer),
            QuestionBox::FirstOrderEquationQuestion(q) => q.check_answer(answer),
            QuestionBox::FirstDegreeDerivativeQuestion(q) => q.check_answer(answer),
            QuestionBox::NumberComparison(q) => q.check_answer(answer),
            QuestionBox::FractionComparison(q) => q.check_answer(answer),
            QuestionBox::SixRounding(q) => q.check_answer(answer),
            QuestionBox::SixAverage(q) => q.check_answer(answer),
            QuestionBox::SixMedian(q) => q.check_answer(answer),
            QuestionBox::SixFractionToDegree(q) => q.check_answer(answer),
            QuestionBox::SevenPercentChange(q) => q.check_answer(answer),
            QuestionBox::EightExpression(q) => q.check_answer(answer),
        }
    }
}

// Function to generate questions based on quiz type
pub fn generate_questions(quiz_type: Quiz, count: usize) -> Vec<QuestionBox> {
    let mut questions = Vec::with_capacity(count);

    for _ in 0..count {
        let question = match quiz_type {
            Quiz::Addition1_10 => QuestionBox::Addition1_10(AdditionQuestion1_10::random()),
            Quiz::Addition100 => QuestionBox::Addition100(AdditionQuestion100::random()),
            Quiz::Subtraction1_10 => {
                QuestionBox::Subtraction1_10(SubtractionQuestion1_10::random())
            }
            Quiz::Multiplication1_10 => {
                QuestionBox::Multiplication1_10(MultiplicationQuestion1_10::random())
            }
            Quiz::Multiplication1_20 => {
                QuestionBox::Multiplication1_20(MultiplicationQuestion1_20::random())
            }
            Quiz::Division1_10 => QuestionBox::Division1_10(DivisionQuestion1_10::random()),
            Quiz::SquareArea => QuestionBox::Area(AreaQuestion::random()),
            Quiz::NoCourse => QuestionBox::Addition1_10(AdditionQuestion1_10::random()),
            Quiz::FirstOrderEquation => {
                QuestionBox::FirstOrderEquationQuestion(FirstOrderEquationQuestion::random())
            }
            Quiz::FirstDegreeDerivativeQuestion => {
                QuestionBox::FirstDegreeDerivativeQuestion(FirstDegreeDerivativeQuestion::random())
            }
            Quiz::NumberComparison => {
                QuestionBox::NumberComparison(NumberComparisonQuestion::random())
            }
            Quiz::FractionComparison => {
                QuestionBox::FractionComparison(FractionComparisonQuestion::random())
            }
            Quiz::Rounding => QuestionBox::SixRounding(RoundingQuestion::random()),
            Quiz::Average => QuestionBox::SixAverage(AverageQuestion::random()),
            Quiz::Median => QuestionBox::SixMedian(MedianQuestion::random()),
            Quiz::FractionToDegree => QuestionBox::SixFractionToDegree(FractionToDegree::random()),
            Quiz::PercentChange => QuestionBox::SevenPercentChange(PercentChangeQuestion::random()),
            Quiz::Expression => QuestionBox::EightExpression(ExpressionQuestion::random()),
        };

        questions.push(question);
    }

    questions
}

#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    pub display_text: String,
    pub value: String, // The value to emit when this choice is selected, and to be checked by `check_answer`
}
pub trait MultipleChoiceQuestionProvider: Question + Clone + PartialEq + std::fmt::Debug {
    fn get_choices(&self) -> Vec<Choice>;
}

// quizzes/simple_addition_choice_quiz.rs (new file)
// use crate::quiz::{Choice, Question, MultipleChoiceQuestionProvider};
// use rand::Rng;

// #[derive(Clone, Debug, PartialEq)]
// pub struct SimpleAdditionChoiceQuestion {
//     term1: i32,
//     term2: i32,
//     choices_values: Vec<i32>, // Values for the buttons
//     correct_answer_value: i32,
// }

// impl SimpleAdditionChoiceQuestion {
//     pub fn random() -> Self {
//         let mut rng = rand::thread_rng();
//         let term1 = 2; // Fixed for "What is 2 + X?"
//         let term2 = rng.gen_range(1..=10);
//         let correct_result = term1 + term2;

//         let mut choices_values = vec![correct_result];
//         // Generate 2 or 3 wrong answers
//         let num_wrong_answers = rng.gen_range(2..=3); // For 3 or 4 total choices
//         while choices_values.len() < (1 + num_wrong_answers) {
//             let wrong_offset = rng.gen_range(-3..=3);
//             if wrong_offset == 0 { continue; } // Avoid same as correct or other wrong answers
//             let wrong_answer = correct_result + wrong_offset;
//             if !choices_values.contains(&wrong_answer) {
//                 choices_values.push(wrong_answer);
//             }
//         }
//         use rand::seq::SliceRandom;
//         choices_values.shuffle(&mut rng); // Shuffle the choices

//         Self {
//             term1,
//             term2,
//             choices_values,
//             correct_answer_value: correct_result,
//         }
//     }
// }

// impl Question for SimpleAdditionChoiceQuestion {
//     fn prompt(&self) -> String {
//         format!("Vad blir {} + {}?", self.term1, self.term2)
//     }

//     // This answer() is for showing the correct answer if the user gets it wrong.
//     // The `Choice.value` is what's used for button clicks.
//     fn answer(&self) -> &str {
//         // This might be tricky if you only have the value. You might need to store
//         // the correct answer string explicitly or reconstruct it.
//         // For now, let's assume check_answer handles the logic.
//         // If you need to display the correct choice's text, you might need to find it in get_choices().
//         // For simplicity here, we are matching against the numeric value.
//         // A better approach might be to have `correct_choice_value: String` in the struct.
//         // This example focuses on `check_answer` using the stringified numeric value.
//         Box::leak(self.correct_answer_value.to_string().into_boxed_str())
//     }

//     fn check_answer(&self, answer: &str) -> bool {
//         // `answer` will be the stringified value from the Choice struct
//         if let Ok(ans_val) = answer.parse::<i32>() {
//             ans_val == self.correct_answer_value
//         } else {
//             false
//         }
//     }

//     fn display(&self) -> String {
//         self.prompt()
//     }
// }

// impl MultipleChoiceQuestionProvider for SimpleAdditionChoiceQuestion {
//     fn get_choices(&self) -> Vec<Choice> {
//         self.choices_values
//             .iter()
//             .map(|val| Choice {
//                 display_text: val.to_string(),
//                 value: val.to_string(), // Send the number as a string
//             })
//             .collect()
//     }
// }
