use crate::quiz::MultipleChoiceQuestionProvider;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChoiceButtonQuizComponentProps<Q>
where
    Q: MultipleChoiceQuestionProvider + 'static,
{
    pub question: Q,
    pub on_answer: Callback<String>,
}

#[function_component]
pub fn ChoiceButtonQuizComponent<Q>(props: &ChoiceButtonQuizComponentProps<Q>) -> Html
where
    Q: MultipleChoiceQuestionProvider + 'static,
{
    let choices = props.question.get_choices();

    html! {
        <div class="choice-button-quiz-container"> // You can name this class as you like
            // The main question prompt (e.g., "Vilket värde är störst: ... eller ...?")
            // comes from the Question trait's display method.
            <h2>{ props.question.display() }</h2>

            <div class="choice-options"> // Container for the buttons
                {
                    choices.into_iter().map(|choice| {
                        let on_choice_selected = {
                            let on_answer = props.on_answer.clone();
                            let choice_value = choice.value.clone();
                            Callback::from(move |_| {
                                on_answer.emit(choice_value.clone());
                            })
                        };
                        html! {
                            <button
                                class="choice-option-button" // General class for styling
                                onclick={on_choice_selected}
                            >
                                { choice.display_text }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
