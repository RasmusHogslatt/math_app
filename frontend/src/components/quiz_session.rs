use crate::{
    components::ChoiceButtonQuizComponent,
    quiz::{Question, QuestionBox},
    quizzes::{FractionComparisonQuestion, NumberComparisonQuestion},
};
use web_time::Duration;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuizSectionProps {
    pub question: QuestionBox,
    pub elapsed_time: Duration,
    pub current_question: usize,
    pub total_questions: usize,
    pub on_answer: Callback<String>,
}

#[function_component(QuizSession)]
pub fn quiz_session(props: &QuizSectionProps) -> Html {
    let input_ref = use_node_ref();
    let answer = use_state(String::new);

    let on_submit = {
        let input_ref = input_ref.clone();
        let answer = answer.clone();
        let on_answer = props.on_answer.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let answer_value = (*answer).clone();
            on_answer.emit(answer_value);
            answer.set(String::new());
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input.focus().unwrap_or_default();
            }
        })
    };

    let on_input = {
        let answer = answer.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            answer.set(input.value());
        })
    };

    let timer_display = format!("Time: {:.1} seconds", props.elapsed_time.as_secs_f32());
    let progress = format!(
        "Question {}/{}",
        props.current_question + 1,
        props.total_questions
    );

    html! {
        <div class="quiz-section">
            <div class="quiz-header">
                <div class="timer">{timer_display}</div>
                <div class="progress">{progress}</div>
            </div>

            {
                // Choose which component to render based on the question type
                match &props.question {
                    QuestionBox::NumberComparison(question) => {
                        html! {
                            <ChoiceButtonQuizComponent<NumberComparisonQuestion>
                                question={question.clone()}
                                on_answer={props.on_answer.clone()}
                            />
                        }
                    },
                    QuestionBox::FractionComparison(question) => {
                        html! {
                            <ChoiceButtonQuizComponent<FractionComparisonQuestion>
                                question={question.clone()}
                                on_answer={props.on_answer.clone()}
                            />
                        }
                    },
                    // Example: If you add a new question type that uses this component
                    // QuestionBox::YourNewMultiChoiceQuestion(question) => {
                    // html! {
                    // <ChoiceButtonQuizComponent<YourNewMultiChoiceQuestionType>
                    // question={question.clone()}
                    // on_answer={props.on_answer.clone()}
                    // />
                    // }
                    // },
                    _ => {
                        html! {
                            <div class="question">
                                <h2>{props.question.display()}</h2>
                                <form onsubmit={on_submit}>
                                    <input
                                        type="text"
                                        ref={input_ref}
                                        value={(*answer).clone()}
                                        oninput={on_input}
                                        placeholder="Ange ditt svar"
                                    />
                                    <button type="submit">{"Submit"}</button>
                                </form>
                            </div>
                        }
                    }
                }
            }
        </div>
    }
}
