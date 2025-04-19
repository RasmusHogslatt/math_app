// frontend/src/components/leaderboard.rs
use common::{LeaderboardEntry, SubmitScoreRequest};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Backend API base URL
const API_BASE_URL: &str = "http://127.0.0.1:8080";

#[derive(Properties, PartialEq)]
pub struct LeaderboardProps {
    pub course: String,
    pub show_submit: bool,
    pub user_time: Option<f64>,
}

#[function_component(Leaderboard)]
pub fn leaderboard(props: &LeaderboardProps) -> Html {
    let entries = use_state(|| Vec::<LeaderboardEntry>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let player_name = use_state(|| String::new());
    let submit_status = use_state(|| None::<String>);
    let submitted = use_state(|| false);
    let submitting = use_state(|| false);

    // Fetch leaderboard data when component mounts or course changes
    {
        let entries = entries.clone();
        let loading = loading.clone();
        let error = error.clone();
        let course = props.course.clone();

        use_effect_with(props.course.clone(), move |_| {
            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match Request::get(&format!(
                    "{}/api/leaderboard?course={}",
                    API_BASE_URL, course
                ))
                .send()
                .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<Vec<LeaderboardEntry>>().await {
                                Ok(data) => {
                                    entries.set(data);
                                    loading.set(false);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to parse response: {}", e)));
                                    loading.set(false);
                                }
                            }
                        } else {
                            error.set(Some(format!("Server error: {}", response.status())));
                            loading.set(false);
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Network error: {}", e)));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    // Inside the Leaderboard component in leaderboard.rs

    // Add this useEffect to reset form states when the course changes
    {
        let player_name = player_name.clone();
        let submit_status = submit_status.clone();
        let submitted = submitted.clone();
        let submitting = submitting.clone();

        use_effect_with(props.course.clone(), move |_| {
            player_name.set(String::new());
            submit_status.set(None);
            submitted.set(false);
            submitting.set(false);
            || ()
        });
    }

    // In leaderboard.rs, inside the Leaderboard component
    // Add this effect to reset submission states when course changes
    {
        let submitted = submitted.clone();
        let player_name = player_name.clone();
        let submit_status = submit_status.clone();
        let submitting = submitting.clone();

        use_effect_with(props.course.clone(), move |_| {
            // Reset all submission-related states
            submitted.set(false);
            player_name.set(String::new());
            submit_status.set(None);
            submitting.set(false);

            || ()
        });
    }

    // In leaderboard.rs - add this effect
    {
        let submitted = submitted.clone();
        let submitting = submitting.clone();
        let player_name = player_name.clone();
        let submit_status = submit_status.clone();
        let user_time = props.user_time;

        use_effect_with(user_time, move |_| {
            // Reset submission states when new user_time arrives
            if user_time.is_some() {
                submitted.set(false);
                submitting.set(false);
                player_name.set(String::new());
                submit_status.set(None);
            }
            || ()
        });
    }

    let on_name_change = {
        let player_name = player_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            player_name.set(input.value());
        })
    };

    let on_submit_score = {
        // outer clones (make the closure Fn, not FnOnce)
        let player_name = player_name.clone();
        let submit_status = submit_status.clone();
        let entries = entries.clone();
        let course = props.course.clone();
        let user_time = props.user_time;
        let submitting = submitting.clone();
        let submitted = submitted.clone();
        let current_course = props.course.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // Add course validation check
            if course != current_course {
                submit_status.set(Some("Course changed - submission cancelled".into()));
                return;
            }
            // prevent double‐click
            if *submitting || *submitted {
                return;
            }
            submitting.set(true);

            // validate
            let name = (*player_name).trim().to_string();
            if name.is_empty() {
                submit_status.set(Some("Please enter your name".into()));
                submitting.set(false);
                return;
            }
            let time = if let Some(t) = user_time {
                t
            } else {
                submit_status.set(Some("No valid time to submit".into()));
                submitting.set(false);
                return;
            };

            submit_status.set(Some("Submitting…".into()));

            // clones for async block
            let submit_status = submit_status.clone();
            let entries = entries.clone();
            let course = course.clone();
            let name = name.clone();
            let submitting = submitting.clone();
            let submitted = submitted.clone();

            spawn_local(async move {
                let req = SubmitScoreRequest {
                    name,
                    course: course.clone(),
                    time_seconds: time,
                };

                let resp = Request::post(&format!("{}/api/submit", API_BASE_URL))
                    .json(&req)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status() == 201 => {
                        submit_status.set(Some("Score submitted!".into()));
                        submitted.set(true);
                    }
                    Ok(r) if r.status() == 409 => {
                        submit_status.set(Some("You already submitted that score.".into()));
                        submitted.set(true);
                    }
                    Ok(r) => {
                        submit_status.set(Some(format!("Error: {}", r.status())));
                    }
                    Err(err) => {
                        submit_status.set(Some(format!("Network error: {}", err)));
                    }
                }

                // regardless of 201 or 409, we can refresh the leaderboard
                if let Ok(lb) = Request::get(&format!(
                    "{}/api/leaderboard?course={}",
                    API_BASE_URL, course
                ))
                .send()
                .await
                {
                    if let Ok(data) = lb.json::<Vec<LeaderboardEntry>>().await {
                        entries.set(data);
                    }
                }

                submitting.set(false);
            });
        })
    };

    html! {
        <div class="leaderboard-container">
            <h2>{format!("{} Leaderboard", props.course)}</h2>

            // Submit score form
            if props.show_submit && props.user_time.is_some() && !*submitted {
                <div class="submit-score">
                    <h3>{"Submit Your Score"}</h3>
                    <p>{format!("Your time: {:.2} seconds", props.user_time.unwrap())}</p>
                    <form onsubmit={on_submit_score}>
                        <input
                            type="text"
                            placeholder="Enter your name"
                            value={(*player_name).clone()}
                            oninput={on_name_change}
                        />
                        <button type="submit">{"Submit Score"}</button>
                    </form>
                    if let Some(status) = (*submit_status).clone() {
                        <p class="status-message">{status}</p>
                    }
                </div>
            }

            // Leaderboard table
            if *loading {
                <p>{"Loading leaderboard..."}</p>
            } else if let Some(err) = (*error).clone() {
                <p class="error">{format!("Error loading leaderboard: {}", err)}</p>
            } else if (*entries).is_empty() {
                <p>{"No scores yet. Be the first to submit!"}</p>
            } else {
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
                            (*entries).iter().enumerate().map(|(index, entry)| {
                                html! {
                                    <tr>
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

        </div>
    }
}
