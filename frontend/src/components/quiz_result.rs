use crate::quiz::Quiz;
use web_time::Duration;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultSectionProps {
    pub passed: bool,
    pub time_taken: Duration,
    pub course: Quiz,
    pub on_restart: Callback<()>,
}

#[function_component(ResultSection)]
pub fn result(props: &ResultSectionProps) -> Html {
    let message = if props.passed {
        format!(
            "Good job, you finished in {:.1} seconds!",
            props.time_taken.as_secs_f32()
        )
    } else {
        format!("Failed! Try again.")
    };

    let on_restart = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_| {
            on_restart.emit(());
        })
    };

    html! {
        <div class="result-section">
            <h2 class={if props.passed { "success" } else { "failure" }}>{message}</h2>

            <div class="result-actions">
                <button onclick={on_restart}>{"Try Again"}</button>
            </div>
        </div>
    }
}
