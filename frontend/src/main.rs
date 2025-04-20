mod api;
mod components;
mod quiz;

use components::Leaderboard;
use gloo_timers::callback::Interval;
use quiz::*;

use rand::{Rng, rng};
use std::fmt::{self, Display};
use std::rc::Rc;
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

#[derive(Properties, PartialEq)]
struct EnumListSelectorProps {
    pub options: Rc<Vec<Quiz>>,
    pub selected: Quiz,
    pub on_change: Callback<Quiz>,
}

#[function_component(EnumListSelector)]
fn enum_list_selector(props: &EnumListSelectorProps) -> Html {
    html! {
        <div class="enum-list-selector">
            <h3>{"Choose a course:"}</h3>
            <ul>
                {
                    props.options.iter().map(|course| {
                        let course_value = course.clone();
                        let on_click = {
                            let on_change = props.on_change.clone();
                            let course = course_value.clone();
                            Callback::from(move |_| {
                                on_change.emit(course.clone());
                            })
                        };

                        let is_selected = &props.selected == course;
                        let class = if is_selected { "selected" } else { "" };

                        html! {
                            <li
                                class={class}
                                onclick={on_click}
                            >
                                {course.to_string()}
                                {if is_selected { " ✓" } else { "" }}
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
            <p>{"Current selection: "}{props.selected.to_string()}</p>
        </div>
    }
}

#[function_component(QuizSection)]
fn quiz_section(props: &QuizSectionProps) -> Html {
    let input_ref = use_node_ref();
    let answer = use_state(|| String::new());

    let on_submit = {
        let input_ref = input_ref.clone();
        let answer = answer.clone();
        let on_answer = props.on_answer.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let answer_value = (*answer).parse::<i32>().unwrap_or(0);
            on_answer.emit(answer_value);
            answer.set(String::new());
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input.focus().unwrap_or_default();
            }
        })
    };

    let on_input = {
        let answer = answer.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            answer.set(input.value());
        })
    };

    let timer_display = format!("Time: {:.1} seconds", props.elapsed_time.as_secs_f32());
    let progress = format!(
        "Question {}/{}",
        props.current_question + 1,
        props.total_questions
    );

    html! {
        <div class="quiz-section">
            <div class="quiz-header">
                <div class="timer">{timer_display}</div>
                <div class="progress">{progress}</div>
            </div>
            <div class="question">
                <h2>{props.question.display()}</h2>
                <form onsubmit={on_submit}>
                    <input
                        type="number"
                        ref={input_ref}
                        value={(*answer).clone()}
                        oninput={on_input}
                        placeholder="Enter your answer"
                       // autoFocus=true
                    />
                    <button type="submit">{"Submit"}</button>
                </form>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct QuizSectionProps {
    pub question: Question,
    pub elapsed_time: Duration,
    pub current_question: usize,
    pub total_questions: usize,
    pub on_answer: Callback<i32>,
}

#[function_component(ResultSection)]
fn result_section(props: &ResultSectionProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct ResultSectionProps {
    pub passed: bool,
    pub time_taken: Duration,
    pub course: Quiz,
    pub on_restart: Callback<()>,
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    // Create a vector that holds all possible enum variants
    let all_courses = Rc::new(vec![
        Quiz::NoCourse,
        Quiz::Addition,
        Quiz::Subtraction,
        Quiz::Multiplication,
    ]);

    let course = use_state(|| Quiz::NoCourse);
    let app_state = use_state(|| AppState::Selection);
    let questions = use_state(Vec::new);
    let current_question = use_state(|| 0);
    let start_time = use_state(|| Instant::now());
    let elapsed_time = use_state(|| Duration::from_secs(0));
    let interval_ref = use_mut_ref(|| None::<Interval>);
    /*  console::log_3(
        &format!("start_time: {:#?}", *start_time).into(),
        &format!("elapsed_time: {:#?}", *elapsed_time).into(),
        &format!("interval_ref: {:#?}", interval_ref.borrow()).into(),
    ); */

    // Total number of questions
    let total_questions = 3;

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
        let start_time = start_time.clone();
        let start_time_handle = start_time.clone();
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
                // Generate questions based on selected course
                let generated_questions = generate_questions(*course.clone(), total_questions);
                questions.set(generated_questions);
                current_question.set(0);

                // Reset timer
                let quiz_start_instant = Instant::now(); // <<< Get the specific Instant value
                start_time_handle.set(quiz_start_instant); // <<< Update the state using the handle
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

                // Start timer interval using the NEW start_time VALUE
                let elapsed = elapsed_time.clone();
                //let start = start_time.clone(); // Use the updated start_time
                // *** CAPTURE THE VALUE ***
                let timer_interval = Interval::new(100, move || {
                    let now = Instant::now();
                    // Use the captured 'quiz_start_instant' value directly
                    let duration = now.duration_since(quiz_start_instant);
                    elapsed.set(duration);
                });
                interval_ref.borrow_mut().replace(timer_interval); // Store the new interval handle
                web_sys::console::log_1(&"on_start_quiz END: New timer created and stored.".into());
                app_state.set(AppState::Quiz);
            }
        })
    };

    // Answer submission handler
    let on_answer = {
        let questions = questions.clone();
        let current_question = current_question.clone();
        let app_state = app_state.clone();
        let elapsed_time = elapsed_time.clone();
        let interval_ref = interval_ref.clone();

        Callback::from(move |answer: i32| {
            let current_q = *current_question;
            let q = &(*questions)[current_q];

            // Check if answer is correct
            if answer == q.answer {
                // If this is the last question
                if current_q + 1 >= total_questions {
                    // Stop timer
                    if let Some(handle) = interval_ref.borrow_mut().take() {
                        handle.cancel();
                    }
                    // Show success result
                    app_state.set(AppState::Result(true, *elapsed_time));
                    // Show success result
                    app_state.set(AppState::Result(true, *elapsed_time));
                } else {
                    // Move to next question
                    current_question.set(current_q + 1);
                }
            } else {
                // Wrong answer - stop timer and show failure
                if let Some(handle) = interval_ref.borrow_mut().take() {
                    handle.cancel();
                }
                app_state.set(AppState::Result(false, *elapsed_time));
            }
        })
    };

    // Restart handler
    let on_restart = {
        let app_state = app_state.clone();
        let interval_ref = interval_ref.clone();
        let questions = questions.clone();
        let current_question = current_question.clone();
        let elapsed_time = elapsed_time.clone();

        Callback::from(move |_| {
            // Cancel any ongoing interval
            if let Some(handle) = interval_ref.borrow_mut().take() {
                handle.cancel();
            }
            // Reset quiz-related states
            questions.set(Vec::new());
            current_question.set(0);
            elapsed_time.set(Duration::from_secs(0));
            web_sys::console::log_1(
                &format!(
                    "on_restart END: elapsed_time reset. Value is now: {:?}",
                    *elapsed_time
                )
                .into(),
            );
            app_state.set(AppState::Selection);
        })
    };

    {
        let course = course.clone();
        let app_state = app_state.clone();

        use_effect_with((*course).clone(), move |current_course| {
            // Reset to selection state if course changes
            if *current_course != Quiz::NoCourse && *app_state != AppState::Selection {
                app_state.set(AppState::Selection);
            }
            || ()
        });
    }

    // Effect to reset state and STOP TIMER if course changes during quiz/result
    {
        let course = course.clone();
        let app_state = app_state.clone();
        let interval_ref = interval_ref.clone(); // <<< Add interval_ref here
        let questions = questions.clone(); // <<< Add other states to reset if needed
        let current_question = current_question.clone(); // <<< Add
        let elapsed_time = elapsed_time.clone(); // <<< Add

        use_effect_with((*course).clone(), move |current_course| {
            // Check if the app state is not Selection AND the selected course is not NoCourse
            // This prevents resetting unnecessarily when first selecting a course from NoCourse
            if *app_state != AppState::Selection && *current_course != Quiz::NoCourse {
                web_sys::console::log_1(
                    &"Course changed, resetting state and stopping timer.".into(),
                ); // Debug log

                // Stop the timer if it's running
                if let Some(handle) = interval_ref.borrow_mut().take() {
                    handle.cancel();
                    web_sys::console::log_1(&"Timer stopped due to course change.".into());
                }

                // Reset other relevant states
                questions.set(Vec::new());
                current_question.set(0);
                elapsed_time.set(Duration::from_secs(0)); // Reset elapsed time state

                // Reset to selection state
                app_state.set(AppState::Selection);
            }
            || () // Cleanup function (can be empty)
        });
    }

    let allow_submission = matches!(*app_state, AppState::Result(true, _));

    let current_user_time = match *app_state {
        AppState::Result(true, time_taken) => Some(time_taken.as_secs_f64()),
        _ => None,
    };
    // Determine if a score should be submitted (only when in Result state with passing score)
    /*   let show_submit = match *app_state {
        AppState::Result(passed, _) => passed,
        _ => false,
    }; */

    // Get the current time for submission (only when in Result state)
    /*   let user_time = match *app_state {
           AppState::Result(passed, time_taken) if passed => Some(time_taken.as_secs_f64()),
           _ => None,
       };
    */
    html! {
        <div class="app-container">
            <div class="sidebar">
                <EnumListSelector
                    options={all_courses.clone()}
                    selected={(*course).clone()}
                    on_change={on_course_change}
                />
            </div>
            <div class="main-content">
                {
                    match (*app_state).clone() {
                        AppState::Selection => html! {
                            <div class="start-section">
                                <h2>{"Get Ready for the Quiz"}</h2>
                                <p>{"Select a course from the left and click Start when you're ready."}</p>
                                <button
                                    onclick={on_start_quiz}
                                    disabled={*course == Quiz::NoCourse}
                                >
                                    {"Start Course"}
                                </button>
                            </div>
                        },
                        AppState::Quiz => {
                            let current_q = *current_question;
                            if current_q < (*questions).len() {
                                let question = (*questions)[current_q].clone();
                                html! {
                                    <QuizSection
                                        question={question}
                                        elapsed_time={*elapsed_time}
                                        current_question={current_q}
                                        total_questions={total_questions}
                                        on_answer={on_answer.clone()}
                                    />
                                }
                            } else {
                                html! { <p>{"Loading questions..."}</p> }
                            }
                        },
                        AppState::Result(passed, time_taken) => html! {
                            <ResultSection
                                passed={passed}
                                time_taken={time_taken}
                                course={(*course).clone()}
                                on_restart={on_restart.clone()}
                            />
                        }
                    }
                }
            </div>
            <div class="leaderboard-panel">
            <Leaderboard
               course={(*course).to_string()} // Convert Quiz enum to String for the prop
               allow_submission={allow_submission}
               user_time={current_user_time}
               // on_reset_timer is removed
           />
       </div>
        </div>
    }
}

fn main() {
    console::log_1(&"Quiz application starting...".into());
    yew::Renderer::<App>::new().render();
}
