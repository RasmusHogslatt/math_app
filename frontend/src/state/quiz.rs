/* use common::Course;
use common::Question;
use common::generate_questions;
use yew::prelude::*;

/// Holds current quiz state
pub struct QuizState {
    pub course: Course,
    pub questions: Vec<Question>,
    pub current_index: usize,
    pub score: usize,
}

// check this later
impl Default for QuizState {
    fn default() -> Self {
        QuizState {
            course: Course::Addition(common::AdditionSettings { max: 10 }),
            questions: vec![],
            current_index: 0,
            score: 0,
        }
    }
}

/// Actions user can trigger
pub enum Msg {
    SetCourse(Course),
    NextQuestion,
    SubmitAnswer(i32),
    Reset,
}

/// Reducer to update QuizState
pub fn reducer(state: QuizState, msg: Msg) -> QuizState {
    match msg {
        Msg::SetCourse(course) => {
            let questions = generate_questions(&course);
            QuizState {
                course,
                questions,
                current_index: 0,
                score: 0,
            }
        }
        Msg::SubmitAnswer(ans) => {
            let mut state = state;
            if ans == state.questions[state.current_index].answer {
                state.score += 1;
            }
            state
        }
        Msg::NextQuestion => {
            let mut state = state;
            state.current_index = (state.current_index + 1).min(state.questions.len() - 1);
            state
        }
        Msg::Reset => QuizState::default(),
    }
}
 */
