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

    let on_name_change = {
        let player_name = player_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            player_name.set(input.value());
        })
    };

    let on_submit_score = {
        let player_name = player_name.clone();
        let submit_status = submit_status.clone();
        let course = props.course.clone();
        let user_time = props.user_time;
        let entries = entries.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Validate inputs
            let name = (*player_name).clone();
            if name.trim().is_empty() {
                submit_status.set(Some("Please enter your name".to_string()));
                return;
            }

            let time: f64 = match user_time {
                Some(t) => t,
                None => {
                    submit_status.set(Some("No valid time to submit".to_string()));
                    return;
                }
            };

            let course_clone = course.clone();
            let name_clone = name.clone();
            let submit_status_clone = submit_status.clone();
            let entries_clone = entries.clone();

            submit_status.set(Some("Submitting...".to_string()));

            // Submit score to the backend
            spawn_local(async move {
                let request = SubmitScoreRequest {
                    name: name_clone,
                    course: course_clone.clone(),
                    time_seconds: time,
                };

                match Request::post(&format!("{}/api/submit", API_BASE_URL))
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 201 {
                            submit_status_clone
                                .set(Some("Score submitted successfully!".to_string()));

                            // Refresh leaderboard
                            match Request::get(&format!(
                                "{}/api/leaderboard?course={}",
                                API_BASE_URL, course_clone
                            ))
                            .send()
                            .await
                            {
                                Ok(leaderboard_response) => {
                                    if let Ok(data) =
                                        leaderboard_response.json::<Vec<LeaderboardEntry>>().await
                                    {
                                        entries_clone.set(data);
                                    }
                                }
                                Err(_) => {}
                            }
                        } else {
                            submit_status_clone.set(Some(format!("Error: {}", response.status())));
                        }
                    }
                    Err(e) => {
                        submit_status_clone.set(Some(format!("Network error: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="leaderboard-container">
            <h2>{format!("{} Leaderboard", props.course)}</h2>

            // Submit score form
            if props.show_submit && props.user_time.is_some() {
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

            // Server-side rendered version link
            <div class="ssr-link">
                <p>
                    {"View "}
                    <a href={format!("{}/leaderboard?course={}", API_BASE_URL, props.course)} target="_blank">
                        {"server-rendered leaderboard"}
                    </a>
                </p>
            </div>
        </div>
    }
}
