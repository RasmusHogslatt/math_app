use crate::{quiz::Question, quizzes::FractionComparisonQuestion};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FractionComparisonComponentProps {
    pub question: FractionComparisonQuestion,
    pub on_answer: Callback<String>,
}

#[function_component(FractionComparisonComponent)]
pub fn fraction_comparison_component(props: &FractionComparisonComponentProps) -> Html {
    let on_first_selected = {
        let on_answer = props.on_answer.clone();
        Callback::from(move |_| {
            on_answer.emit("0".to_string()); // 0 for first value
        })
    };

    let on_second_selected = {
        let on_answer = props.on_answer.clone();
        Callback::from(move |_| {
            on_answer.emit("1".to_string()); // 1 for second value
        })
    };

    html! {
        <div class="number-comparison-container">
            <h2>{ props.question.display() }</h2>
            <div class="comparison-options">
                <button
                    class="comparison-option"
                    onclick={on_first_selected}
                >
                    { props.question.first_value().display() }
                </button>

                <div class="comparison-separator">{"or"}</div>

                <button
                    class="comparison-option"
                    onclick={on_second_selected}
                >
                    { props.question.second_value().display() }
                </button>
            </div>
        </div>
    }
}
