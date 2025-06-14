mod api;
pub mod components;
mod quiz;
mod quizzes;
mod util;
use common::User;
use components::Leaderboard;
use components::QuizSelect;
use components::QuizSession;
use components::ResultSection;
use components::TopUsers;
use gloo_timers::callback::Interval;
use quiz::*;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::console;
use web_time::{Duration, Instant};
use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum AppState {
    Selection,
    Quiz,
    Result(bool, Duration), // (passed, time_taken)
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    const DEV_UUID: Uuid = uuid::uuid!("26493f93-6de3-4376-94b3-c8ec2dba1dc4");
    let mut dummy_user = User::new_dummy();
    dummy_user.school_id = DEV_UUID;
    let all_courses = Rc::new(vec![
        Quiz::NoCourse,
        Quiz::Addition1_10,
        Quiz::Addition100,
        Quiz::Subtraction1_10,
        Quiz::Multiplication1_10,
        Quiz::Multiplication1_20,
        Quiz::Division1_10,
        Quiz::Area,
        Quiz::Circumference,
        Quiz::FirstOrderEquation,
        Quiz::FirstDegreeDerivativeQuestion,
        Quiz::NumberComparison,
        Quiz::FractionComparison,
        Quiz::Rounding,
        Quiz::Average,
        Quiz::Median,
        Quiz::FractionToDegree,
        Quiz::PercentChange,
        Quiz::Expression,
        Quiz::NegativeValues,
        Quiz::ClockReading,
        Quiz::ClockReadingSimple,
        Quiz::RomanNumerals,
    ]);

    let course = use_state(|| Quiz::NoCourse);
    let app_state = use_state(|| AppState::Selection);
    let questions = use_state(Vec::new);
    let current_question = use_state(|| 0);
    let start_time = use_state(Instant::now);
    let elapsed_time = use_state(|| Duration::from_secs(0));
    let interval_ref = use_mut_ref(|| None::<Interval>);
    let failed_question_details = use_state(|| None::<(QuestionBox, String)>);

    // Course selection handler
    let on_course_change = {
        let course = course.clone();
        Callback::from(move |new_course: Quiz| {
            course.set(new_course);
        })
    };

    // Start quiz handler
    let on_start_quiz = {
        let course = course.clone();
        let app_state = app_state.clone();
        let questions = questions.clone();
        let current_question = current_question.clone();
        let start_time_state_handle = start_time.clone();
        let elapsed_time = elapsed_time.clone();
        let interval_ref = interval_ref.clone();

        Callback::from(move |_| {
            if *course != Quiz::NoCourse {
                web_sys::console::log_1(
                    &format!(
                        "on_start_quiz START: elapsed_time BEFORE reset: {:?}",
                        *elapsed_time
                    )
                    .into(),
                );

                let number_questions_for_quiz = course.number_of_questions();
                if number_questions_for_quiz == 0 {
                    web_sys::console::log_1(&"Cannot start quiz: 0 questions configured.".into());
                    return;
                }

                // Generate questions based on selected course
                let generated_questions =
                    generate_questions(*course.clone(), number_questions_for_quiz);
                questions.set(generated_questions);
                current_question.set(0);

                // Reset timer
                let quiz_start_instant = Instant::now();
                start_time_state_handle.set(quiz_start_instant);
                elapsed_time.set(Duration::from_secs(0));

                web_sys::console::log_2(
                    &format!(
                        "on_start_quiz MID: start_time state set to: {:?}",
                        quiz_start_instant
                    )
                    .into(),
                    &format!(
                        "on_start_quiz MID: elapsed_time reset to: {:?}",
                        *elapsed_time
                    )
                    .into(),
                );

                // Start timer interval
                let elapsed = elapsed_time.clone();
                let timer_interval = Interval::new(100, move || {
                    let now = Instant::now();
                    let duration = now.duration_since(quiz_start_instant);
                    elapsed.set(duration);
                });
                interval_ref.borrow_mut().replace(timer_interval);
                web_sys::console::log_1(&"on_start_quiz END: New timer created and stored.".into());
                app_state.set(AppState::Quiz);
            }
        })
    };

    let on_answer = {
        let questions = questions.clone();
        let current_question = current_question.clone();
        let app_state = app_state.clone();
        let elapsed_time = elapsed_time.clone();
        let interval_ref = interval_ref.clone();
        let failed_question_details = failed_question_details.clone();

        Callback::from(move |answer: String| {
            let current_q = *current_question;
            let total_questions_for_current_quiz = (*questions).len();
            if total_questions_for_current_quiz == 0 {
                // Should not happen if on_start_quiz has guards
                return;
            }
            // Ensure we don't panic if questions isn't populated somehow
            if let Some(q) = (*questions).get(current_q) {
                if q.check_answer(&answer) {
                    // Correct Answer Logic
                    if current_q + 1 >= total_questions_for_current_quiz {
                        if let Some(handle) = interval_ref.borrow_mut().take() {
                            handle.cancel();
                        }
                        failed_question_details.set(None);
                        app_state.set(AppState::Result(true, *elapsed_time));
                    } else {
                        current_question.set(current_q + 1);
                    }
                } else {
                    if let Some(handle) = interval_ref.borrow_mut().take() {
                        handle.cancel();
                    }
                    failed_question_details.set(Some((q.clone(), answer)));
                    app_state.set(AppState::Result(false, *elapsed_time));
                }
            } else {
                if let Some(handle) = interval_ref.borrow_mut().take() {
                    handle.cancel();
                }
                failed_question_details.set(None);
                questions.set(Vec::new());
                current_question.set(0);
                elapsed_time.set(Duration::from_secs(0));
                app_state.set(AppState::Selection);
            }
        })
    };

    let on_restart = {
        let app_state = app_state.clone();
        let interval_ref = interval_ref.clone();
        let questions = questions.clone();
        let current_question = current_question.clone();
        let elapsed_time = elapsed_time.clone();
        let failed_question_details = failed_question_details.clone();

        Callback::from(move |_| {
            if let Some(handle) = interval_ref.borrow_mut().take() {
                handle.cancel();
            }
            questions.set(Vec::new());
            current_question.set(0);
            elapsed_time.set(Duration::from_secs(0));
            failed_question_details.set(None);
            app_state.set(AppState::Selection);
        })
    };

    // Reset timer and states
    {
        let course = course.clone();
        let app_state = app_state.clone();
        let interval_ref = interval_ref.clone();
        let questions = questions.clone();
        let current_question = current_question.clone();
        let elapsed_time = elapsed_time.clone();
        let failed_question_details = failed_question_details.clone();

        use_effect_with(*course, move |_current_course| {
            if *app_state != AppState::Selection {
                web_sys::console::log_1(
                    &"Course changed, resetting state and stopping timer.".into(),
                );

                if let Some(handle) = interval_ref.borrow_mut().take() {
                    handle.cancel();
                    web_sys::console::log_1(&"Timer stopped due to course change.".into());
                }

                questions.set(Vec::new());
                current_question.set(0);
                elapsed_time.set(Duration::from_secs(0));
                failed_question_details.set(None);
                app_state.set(AppState::Selection);
            }
            || ()
        });
    }

    let allow_submission = matches!(*app_state, AppState::Result(true, _));

    let current_user_time = match *app_state {
        AppState::Result(true, time_taken) => Some(time_taken.as_secs_f64()),
        _ => None,
    };

    html! {
        <div class="app-container">
            <div class="sidebar">
                <QuizSelect
                    options={all_courses.clone()}
                    selected={*course}
                    on_change={on_course_change}
                />
            </div>
            <div class="main-content">
                <div class="title-section">
                    <h1>{"Mer Matte"}</h1>
                     <TopUsers
                        limit={3}
                        school_name={dummy_user.school.clone()}
                        school_id={dummy_user.school_id}
                    />

                </div>
                <div class="dynamic-content-wrapper">
                    {
                        match (*app_state).clone() {
                            AppState::Selection => html! {
                                <div class="start-section">
                                    <h2>{format!("{}", course.to_string())}</h2>
                                    <p>{"Välj en quiz i listan och klicka på Starta quiz när du är redo."}</p>
                                    <button
                                        onclick={on_start_quiz}
                                        disabled={*course == Quiz::NoCourse}
                                    >
                                        {"Starta quiz"}
                                    </button>
                                </div>
                            },
                            AppState::Quiz => {
                                let current_q = *current_question;
                                let total_questions_for_current_quiz = (*questions).len();
                                if current_q < (*questions).len() {
                                    let question = (*questions)[current_q].clone();
                                    html! {
                                        <QuizSession
                                            question={question}
                                            elapsed_time={*elapsed_time}
                                            current_question={current_q}
                                            total_questions={total_questions_for_current_quiz}
                                            on_answer={on_answer.clone()}
                                        />
                                    }
                                } else {
                                    html! { <p>{"Laddar frågor..."}</p> }
                                }
                            },
                            AppState::Result(passed, time_taken) => {
                                let failure_data = if !passed {
                                    (*failed_question_details).clone()
                                } else {
                                    None
                                };
                                html! {
                                    <ResultSection
                                    passed={passed}
                                    time_taken={time_taken}
                                    course={*course}
                                    on_restart={on_restart.clone()}
                                    failed_question_details={failure_data}
                                />
                            }
                            }
                        }
                    }
                </div>
            </div>
            <div class="leaderboard-panel">
            <Leaderboard
               course={(*course).to_string()}
               user={dummy_user.clone()}
               allow_submission={allow_submission}
               user_time={current_user_time}
           />
       </div>
        </div>
    }
}

fn main() {
    console::log_1(&"Quiz application starting...".into());
    yew::Renderer::<App>::new().render();
}
