use common::TopUserSchoolEntry;

use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::fetch_top_users_by_school;

#[derive(Clone, PartialEq)]
enum FetchTopUsersState {
    Idle,
    Loading,
    Success(Vec<TopUserSchoolEntry>),
    Error(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct TopUsersProps {
    pub school_name: String, // Added: Name of the school to fetch top users for
    pub school_id: Uuid,     // Added: ID of the school
    #[prop_or(3)]
    pub limit: usize, // How many top users to fetch for the podium calculation
}

#[function_component(TopUsers)]
pub fn top_users(props: &TopUsersProps) -> Html {
    let fetch_state = use_state(|| FetchTopUsersState::Idle);

    // Clone props for the effect's dependency and async block
    let school_name_for_effect = props.school_name.clone();
    let school_id_for_effect = props.school_id;
    let limit_for_effect = props.limit;
    let fetch_state_for_effect = fetch_state.clone();

    use_effect_with(
        (
            school_name_for_effect.clone(),
            school_id_for_effect,
            limit_for_effect,
        ), // Dependencies
        move |(current_school_name, current_school_id, current_limit)| {
            fetch_state_for_effect.set(FetchTopUsersState::Loading);

            // Create owned copies of all data needed in the async block
            let school_name_cloned = current_school_name.clone();
            let school_id_cloned = *current_school_id;
            let limit_cloned = *current_limit;
            let fetch_state_cloned = fetch_state_for_effect.clone();

            spawn_local(async move {
                match fetch_top_users_by_school(
                    &school_name_cloned, // Pass a reference to our owned String
                    &school_id_cloned,
                    limit_cloned as u32,
                )
                .await
                {
                    Ok(api_data) => {
                        // api_data is already Vec<TopUserSchoolEntry>
                        fetch_state_cloned.set(FetchTopUsersState::Success(api_data));
                    }
                    Err(e) => {
                        fetch_state_cloned.set(FetchTopUsersState::Error(format!(
                            "Failed to load top users: {}",
                            e
                        )));
                    }
                }
            });
            || () // Cleanup function
        },
    );
    // Helper to render each podium step
    let render_podium_step = |user_opt: Option<&TopUserSchoolEntry>, // Use TopUserSchoolEntry
                              rank: u32,
                              place_class: &'static str|
     -> Html {
        match user_opt {
            Some(user) => html! {
                <div class={classes!("podium-step", place_class)}>
                    <div class="podium-rank">{rank.to_string()}</div>
                    <div class="podium-name" title={format!("{} från {}", user.name, user.school)}>{&user.name}</div>
                    <div class="podium-count">{format!("{} listor", user.leaderboard_count)}</div>
                    // --- START: Added Medal Display ---
                    <div class="podium-medals">
                        { if user.gold_medals > 0 { html!{ <span class="medal gold" title={format!("{} Guld", user.gold_medals)}>{"🥇"}{ user.gold_medals }</span> } } else { html!{} } }
                        { if user.silver_medals > 0 { html!{ <span class="medal silver" title={format!("{} Silver", user.silver_medals)}>{"🥈"}{ user.silver_medals }</span> } } else { html!{} } }
                        { if user.bronze_medals > 0 { html!{ <span class="medal bronze" title={format!("{} Brons", user.bronze_medals)}>{"🥉"}{ user.bronze_medals }</span> } } else { html!{} } }
                        { if user.generic_medals > 0 { html!{ <span class="medal generic" title={format!("{} Övriga", user.generic_medals)}>{"🏅"}{ user.generic_medals }</span> } } else { html!{} } }
                    </div>
                    // --- END: Added Medal Display ---
                </div>
            },
            None => {
                // Render an empty step if no user for this rank
                html! { <div class={classes!("podium-step", "empty", place_class)}></div> }
            }
        }
    };

    html! {
    <div class="top-users-podium-container">
        <h4 class="podium-title">{format!("På flest olika topplistor")}</h4>
        {
            match &*fetch_state {
                FetchTopUsersState::Idle | FetchTopUsersState::Loading => html!{
                    <div class="podium-status">{"Laddar toppmatematiker..."}</div>
                },
                FetchTopUsersState::Error(err) => html!{
                    <div class="podium-status error">{format!("Kunde inte ladda: {}", err.chars().take(50).collect::<String>())}</div>
                },
                FetchTopUsersState::Success(users) => {
                    // Get top 3 users for display, even if props.limit fetched more
                    let first = users.first();
                    let second = users.get(1);
                    let third = users.get(2);

                    if users.is_empty() {
                         html! { <div class="podium-status">{"Inga användare ännu. Bli först på pallen!"}</div> }
                    } else {
                        html! {
                            <>
                            <div class="podium">
                                // Visual order: 2nd, 1st (center, higher), 3rd
                                { render_podium_step(second, 2, "second-place") }
                                { render_podium_step(first, 1, "first-place") }
                                { render_podium_step(third, 3, "third-place") }
                            </div>
                            </>
                        }
                    }
                }
            }
        }
        </div>
    }
}
