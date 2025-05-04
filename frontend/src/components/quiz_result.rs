use crate::quiz::{Question, QuestionBox, Quiz};
use web_time::Duration;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultSectionProps {
    pub passed: bool,
    pub time_taken: Duration,
    pub course: Quiz,
    pub on_restart: Callback<()>,
    pub failed_question_details: Option<(QuestionBox, String)>,
}

#[function_component(ResultSection)]
pub fn result(props: &ResultSectionProps) -> Html {
    let message = if props.passed {
        format!(
            "Snyggt jobbat, det tog {:.1} sekunder!",
            props.time_taken.as_secs_f32()
        )
    } else {
        format!("Fel! Försök igen.")
    };

    let on_restart = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_| {
            on_restart.emit(());
        })
    };

    html! {
        <div class="result-section">
            <h2 class={if props.passed { "Alla rätt" } else { "Fel svar" }}>{message}</h2>

            { if !props.passed {
                if let Some((failed_question, user_answer)) = &props.failed_question_details {
                    html! {
                        <div class="failure-details">
                            <p><strong>{"Fråga:"}</strong><br/>{ failed_question.prompt() }</p>
                            <p><strong>{"Ditt svar:"}</strong><br/><span style="color: red;">{ user_answer }</span></p>
                            <p><strong>{"Rätt svar:"}</strong><br/><span style="color: green;">{ failed_question.answer() }</span></p>
                        </div>
                    }
                } else {
                    // Optional: Message if details are missing for some reason
                    html! { <p>{"Could not load failure details."}</p>}
                }
            } else {
                html! {} // Render nothing if passed
            }}

            <div class="result-actions">
                <button onclick={on_restart}>{"Försök igen"}</button>
            </div>
        </div>
    }
}
