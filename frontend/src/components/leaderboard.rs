// frontend/src/components/leaderboard.rs
use crate::api::{self, ApiError}; // Import the api module and error type
use common::{LeaderboardEntry, SubmitScoreRequest, User};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Define state enums for clarity
#[derive(Clone, PartialEq)]
enum FetchState {
    Idle,
    Loading,
    Success(Vec<LeaderboardEntry>),
    Error(String),
}

#[derive(Clone, PartialEq, Debug)]
enum SubmitState {
    Idle, // Can submit
    Submitting,
    Success(String),  // Success message
    Error(String),    // Error message
    AlreadySubmitted, // Special case for 409 or if submitted flag is true
}

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    pub course: String,
    pub user: User,
    pub allow_submission: bool,
    pub user_time: Option<f64>,
}

#[function_component(Leaderboard)]
pub fn leaderboard(props: &LeaderboardProps) -> Html {
    let fetch_state = use_state(|| FetchState::Idle);
    let player_name = use_state(String::new);
    let submit_state = use_state(|| SubmitState::Idle);

    // Ensure this is the version you have:
    {
        let fetch_state = fetch_state.clone();
        let course = props.course.clone();
        let user = props.user.clone(); // User prop contains school/school_id

        // *** Effect depends on course AND user ***
        use_effect_with((course.clone(), user.clone()), move |_| {
            if course == "No Course" {
                fetch_state.set(FetchState::Success(Vec::new()));
            } else {
                fetch_state.set(FetchState::Loading);
                // *** Get school/school_id from the user prop ***
                let school = user.school.clone();
                let school_id = user.school_id;

                spawn_local(async move {
                    // *** Call fetch_leaderboard with all 3 arguments ***
                    match api::fetch_leaderboard(&course, &school, &school_id).await {
                        Ok(data) => fetch_state.set(FetchState::Success(data)),
                        Err(e) => {
                            fetch_state.set(FetchState::Error(format!("Kunde inte ladda: {}", e)))
                        }
                    }
                });
            }
            || ()
        });
    }

    // Effect for resetting submission state when course or user_time changes
    {
        let submit_state = submit_state.clone();
        let user_time = props.user_time;

        use_effect_with(
            (props.course.clone(), user_time, props.user.name.clone()),
            move |_| {
                // Consider pre-filling name again if user changes, or keep empty
                // player_name.set(initial_name); // Or player_name.set(String::new());
                submit_state.set(SubmitState::Idle);
                || ()
            },
        );
    }

    let handle_name_change = {
        let player_name = player_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            player_name.set(input.value());
        })
    };

    // Extracted function to refresh leaderboard after submission
    let refresh_leaderboard = {
        let fetch_state = fetch_state.clone();
        let course = props.course.clone();
        let user = props.user.clone(); // Need user info for refresh as well
        Callback::from(move |_| {
            if course != "No Course" {
                let fetch_state = fetch_state.clone();
                let course = course.clone();
                let school = user.school.clone(); // Get user details for refresh
                let school_id = user.school_id;
                fetch_state.set(FetchState::Loading);
                spawn_local(async move {
                    // Refresh using the updated fetch_leaderboard call
                    match api::fetch_leaderboard(&course, &school, &school_id).await {
                        Ok(data) => fetch_state.set(FetchState::Success(data)),
                        Err(e) => fetch_state
                            .set(FetchState::Error(format!("Kunde inte ladda om: {}", e))),
                    }
                });
            }
        })
    };

    let handle_submit = {
        let player_name = player_name.clone();
        let submit_state = submit_state.clone();
        let course = props.course.clone();
        let user_time = props.user_time;
        let refresh_leaderboard = refresh_leaderboard.clone();
        // Get user details directly from props inside the callback closure
        let user = props.user.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if !matches!(*submit_state, SubmitState::Idle) {
                // Consider allowing retry from Error state?
                // if !matches!(*submit_state, SubmitState::Idle | SubmitState::Error(_)) { return; }
                return;
            }

            let name_to_submit = (*player_name).trim();
            let name = if name_to_submit.is_empty() {
                // Use name from User prop if input is empty? Or force input?
                // user.name.clone()
                "Anonym" // Or keep forcing input like before
            } else {
                name_to_submit
            };

            if name_to_submit.is_empty() {
                // Keep validation if input required
                submit_state.set(SubmitState::Error("Ange ett namn".into()));
                return;
            }

            let time = match user_time {
                Some(t) => t,
                None => {
                    submit_state.set(SubmitState::Error("Ingen tid finns".into()));
                    return;
                }
            };

            submit_state.set(SubmitState::Submitting);

            let req = SubmitScoreRequest {
                name: name.to_string(), // Use the validated/derived name
                course: course.clone(),
                school: user.school.clone(), // Use school from user prop
                school_id: user.school_id, // Use school_id from user prop
                time_seconds: time,
            };

            let submit_state = submit_state.clone();
            let refresh_leaderboard = refresh_leaderboard.clone();

            spawn_local(async move {
                match api::submit_score(&req).await {
                    Ok(()) => {
                        submit_state
                            .set(SubmitState::Success("Ditt resultat skickades in!".into()));
                        refresh_leaderboard.emit(());
                    }
                    Err(ApiError::Conflict(_)) => {
                        // Use a more specific message potentially, or the one from server
                        submit_state.set(SubmitState::AlreadySubmitted);
                        refresh_leaderboard.emit(());
                    }
                    Err(e) => {
                        submit_state.set(SubmitState::Error(format!(
                            "Resultatet kunde inte skickas in: {}",
                            e
                        )));
                        // Consider resetting to Idle after a delay?
                    }
                }
            });
        })
    };

    // Determine if the submit form should be visible
    let show_submit_form = props.allow_submission
        && props.user_time.is_some()
        && !matches!(
            *submit_state,
            SubmitState::Success(_) | SubmitState::AlreadySubmitted
        );
    html! {
        <div class="leaderboard-container">
        <h2>{format!("Topplista: {}", props.course)}</h2>

            // Submit score form
            if show_submit_form {
                <div class="submit-score">
                    <h3>{"Skicka in din tid"}</h3>
                    <p>{format!("Din tid: {:.2} sekunder", props.user_time.unwrap_or(0.0))}</p>
                    <form onsubmit={handle_submit}>
                        <input
                            type="text"
                            placeholder="Ange ditt namn"
                            value={(*player_name).clone()}
                            oninput={handle_name_change}
                            disabled={*submit_state == SubmitState::Submitting}
                        />
                        <button type="submit" disabled={*submit_state == SubmitState::Submitting}>
                            { if *submit_state == SubmitState::Submitting { "Skickar inte..." } else { "Skicka in" } }
                        </button>
                    </form>
                    {
                        match &*submit_state {
                            SubmitState::Error(msg) => html!{<p class="status-message error">{msg}</p>},
                            // Don't show idle/submitting messages, handled by button text/state
                            _ => html!{}
                        }
                    }
                </div>
            } else if let SubmitState::Success(msg) = &*submit_state {
                // Show success message if form is hidden after success
                <p class="status-message success">{msg}</p>
            } else if let SubmitState::AlreadySubmitted = *submit_state {
                // Show already submitted message
                <p class="status-message success">{"Du har redan skickat in din tid."}</p>
            }

            // Leaderboard table
            {
                match &*fetch_state {
                    FetchState::Idle | FetchState::Loading => html!{ <p>{"Laddar topplistan..."}</p> },
                    FetchState::Error(err) => html!{ <p class="error">{format!("Kunde inte ladda topplista: {}", err)}</p> },
                    FetchState::Success(entries) if entries.is_empty() => html!{ <p>{"Ingen har skickat in ännu. Du kan bli nummer ett!"}</p> },
                    FetchState::Success(entries) => html! {
                        <table class="leaderboard-table">
                            <thead>
                                <tr>
                                    <th>{"Rank"}</th>
                                    <th>{"Namn"}</th>
                                    <th>{"Tid (sekunder)"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    entries.iter().enumerate().map(|(index, entry)| {
                                        html! {
                                            <tr key={entry.id}> // Add a key for performance
                                                <td>{index + 1}</td>
                                                <td>{&entry.name}</td>
                                                <td>{format!("{:.2}", entry.time_seconds)}</td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            </tbody>
                        </table>
                    }
                }
            }
        </div>
    }
}
