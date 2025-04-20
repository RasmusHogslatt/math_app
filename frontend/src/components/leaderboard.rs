// frontend/src/components/leaderboard.rs
use crate::api::{self, ApiError}; // Import the api module and error type
use common::{LeaderboardEntry, SubmitScoreRequest};
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

#[derive(Properties, PartialEq)]
pub struct LeaderboardProps {
    pub course: String,
    // If true, allow showing the submit form IF user_time is Some
    pub allow_submission: bool,
    pub user_time: Option<f64>,
    // No longer need on_reset_timer from parent
}

#[function_component(Leaderboard)]
pub fn leaderboard(props: &LeaderboardProps) -> Html {
    let fetch_state = use_state(|| FetchState::Idle);
    let player_name = use_state(String::new);
    let submit_state = use_state(|| SubmitState::Idle);

    // Effect for fetching leaderboard data
    {
        let fetch_state = fetch_state.clone();
        let course = props.course.clone();

        use_effect_with(props.course.clone(), move |_| {
            if course == "No Course" {
                fetch_state.set(FetchState::Success(Vec::new())); // Don't fetch for "No Course"
            } else {
                fetch_state.set(FetchState::Loading);
                spawn_local(async move {
                    match api::fetch_leaderboard(&course).await {
                        Ok(data) => fetch_state.set(FetchState::Success(data)),
                        Err(e) => {
                            fetch_state.set(FetchState::Error(format!("Failed to load: {}", e)))
                        }
                    }
                });
            }

            || () // Using the same closure type for both paths
        });
    }

    // Effect for resetting submission state when course or user_time changes
    {
        let player_name = player_name.clone();
        let submit_state = submit_state.clone();
        let user_time = props.user_time; // Capture user_time for comparison

        use_effect_with((props.course.clone(), user_time), move |_| {
            // Reset submission state if course changes OR if a new time is available
            player_name.set(String::new());
            submit_state.set(SubmitState::Idle);
            || ()
        });
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
        Callback::from(move |_| {
            if course != "No Course" {
                let fetch_state = fetch_state.clone();
                let course = course.clone();
                fetch_state.set(FetchState::Loading); // Optionally show loading during refresh
                spawn_local(async move {
                    match api::fetch_leaderboard(&course).await {
                        Ok(data) => fetch_state.set(FetchState::Success(data)),
                        Err(e) => {
                            fetch_state.set(FetchState::Error(format!("Failed to refresh: {}", e)))
                        } // Or just keep old data on refresh error
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

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Prevent submission if already submitting or successful/already submitted
            if !matches!(*submit_state, SubmitState::Idle) {
                return;
            }

            let name = (*player_name).trim().to_string();
            if name.is_empty() {
                submit_state.set(SubmitState::Error("Please enter your name".into()));
                return;
            }
            let time = match user_time {
                Some(t) => t,
                None => {
                    // This case should ideally not happen if the form isn't shown
                    submit_state.set(SubmitState::Error("No valid time to submit".into()));
                    return;
                }
            };

            submit_state.set(SubmitState::Submitting);

            let req = SubmitScoreRequest {
                name: name.clone(),
                course: course.clone(),
                time_seconds: time,
            };

            let submit_state = submit_state.clone(); // Clone for async block
            let refresh_leaderboard = refresh_leaderboard.clone(); // Clone for async block

            spawn_local(async move {
                match api::submit_score(&req).await {
                    Ok(()) => {
                        submit_state.set(SubmitState::Success("Score submitted!".into()));
                        refresh_leaderboard.emit(()); // Refresh leaderboard on success
                    }
                    Err(ApiError::Conflict(_)) => {
                        submit_state.set(SubmitState::AlreadySubmitted);
                        refresh_leaderboard.emit(()); // Refresh anyway, maybe data changed
                    }
                    Err(e) => {
                        submit_state.set(SubmitState::Error(format!("Submission failed: {}", e)));
                        // Don't reset to Idle here, keep error shown until user interaction
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
            <h2>{format!("{} Leaderboard", props.course)}</h2>

            // Submit score form
            if show_submit_form {
                <div class="submit-score">
                    <h3>{"Submit Your Score"}</h3>
                    <p>{format!("Your time: {:.2} seconds", props.user_time.unwrap_or(0.0))}</p>
                    <form onsubmit={handle_submit}>
                        <input
                            type="text"
                            placeholder="Enter your name"
                            value={(*player_name).clone()}
                            oninput={handle_name_change}
                            disabled={*submit_state == SubmitState::Submitting}
                        />
                        <button type="submit" disabled={*submit_state == SubmitState::Submitting}>
                            { if *submit_state == SubmitState::Submitting { "Submitting..." } else { "Submit Score" } }
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
                <p class="status-message success">{"Score already submitted."}</p>
            }

            // Leaderboard table
            {
                match &*fetch_state {
                    FetchState::Idle | FetchState::Loading => html!{ <p>{"Loading leaderboard..."}</p> },
                    FetchState::Error(err) => html!{ <p class="error">{format!("Error loading leaderboard: {}", err)}</p> },
                    FetchState::Success(entries) if entries.is_empty() => html!{ <p>{"No scores yet. Be the first to submit!"}</p> },
                    FetchState::Success(entries) => html! {
                        <table class="leaderboard-table">
                            <thead>
                                <tr>
                                    <th>{"Rank"}</th>
                                    <th>{"Name"}</th>
                                    <th>{"Time (seconds)"}</th>
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
