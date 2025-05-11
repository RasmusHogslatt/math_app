use yew::prelude::*;

use crate::quiz::TwoChoiceQuestionProvider;

#[derive(Properties, PartialEq)]
pub struct TwoChoiceButtonComponentProps<Q>
where
    Q: TwoChoiceQuestionProvider + 'static, // Q must implement our trait and be 'static
{
    pub question: Q,
    pub on_answer: Callback<String>,
}

#[function_component]
pub fn TwoChoiceButtonComponent<Q>(props: &TwoChoiceButtonComponentProps<Q>) -> Html
where
    Q: TwoChoiceQuestionProvider + 'static,
{
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
                    { props.question.first_choice_display() }
                </button>

                <div class="comparison-separator">{"or"}</div>

                <button
                    class="comparison-option"
                    onclick={on_second_selected}
                >
                    { props.question.second_choice_display() }
                </button>
            </div>
        </div>
    }
}
