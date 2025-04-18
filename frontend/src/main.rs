use gloo_timers::callback::Interval;

use rand::{Rng, rng};
use std::fmt::{self, Display};
use std::rc::Rc;
use web_sys::console;
use web_time::{Duration, Instant};
use yew::functional::*;
use yew::prelude::*;

// Quiz enum definition
#[derive(Clone, PartialEq, Debug, Copy)]
enum Quiz {
    NoCourse,
    Addition,
    Subtraction,
    Multiplication,
}

// Implement Display for converting enum variants to strings
impl Display for Quiz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quiz::NoCourse => write!(f, "No Course"),
            Quiz::Addition => write!(f, "Addition"),
            Quiz::Subtraction => write!(f, "Subtraction"),
            Quiz::Multiplication => write!(f, "Multiplication"),
        }
    }
}

// Structure to represent a quiz question
#[derive(Clone, PartialEq)]
struct Question {
    first_operand: i32,
    second_operand: i32,
    operator: String,
    answer: i32,
}

impl Question {
    fn new(first: i32, second: i32, op: &str, ans: i32) -> Self {
        Question {
            first_operand: first,
            second_operand: second,
            operator: op.to_string(),
            answer: ans,
        }
    }

    fn display(&self) -> String {
        format!(
            "{} {} {} = ?",
            self.first_operand, self.operator, self.second_operand
        )
    }
}

// Enum for application state
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
            <button onclick={on_restart}>{"Try Again"}</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ResultSectionProps {
    pub passed: bool,
    pub time_taken: Duration,
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

    // Total number of questions
    let total_questions = 10;

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
        let elapsed_time = elapsed_time.clone();
        let interval_ref = interval_ref.clone();

        Callback::from(move |_| {
            if *course != Quiz::NoCourse {
                // Generate questions based on selected course
                let generated_questions = generate_questions(*course.clone(), total_questions);
                questions.set(generated_questions);
                current_question.set(0);

                // Reset timer
                let now = Instant::now();
                start_time.set(now);
                elapsed_time.set(Duration::from_secs(0));

                // Start timer interval
                let elapsed = elapsed_time.clone();
                let start = start_time.clone();
                let timer_interval = Interval::new(100, move || {
                    let now = Instant::now();
                    let duration = now.duration_since(*start);
                    elapsed.set(duration);
                });
                interval_ref.borrow_mut().replace(timer_interval);

                // Change state to Quiz
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

        Callback::from(move |_| {
            // Ensure any lingering interval is cancelled
            if let Some(handle) = interval_ref.borrow_mut().take() {
                handle.cancel();
            }
            app_state.set(AppState::Selection);
        })
    };

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
                                on_restart={on_restart.clone()}
                            />
                        }
                    }
                }
            </div>
            <style>
                {r#"
                    .app-container {
                        display: flex;
                        height: 100vh;
                    }
                    .sidebar {
                        width: 250px;
                        padding: 20px;
                        background-color: #f5f5f5;
                        border-right: 1px solid #ddd;
                    }
                    .main-content {
                        flex: 1;
                        padding: 20px;
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                    }
                    .enum-list-selector ul {
                        list-style-type: none;
                        padding: 0;
                    }
                    .enum-list-selector li {
                        padding: 8px 12px;
                        cursor: pointer;
                        border: 1px solid #ddd;
                        margin-bottom: 4px;
                        border-radius: 4px;
                    }
                    .enum-list-selector li:hover {
                        background-color: #f5f5f5;
                    }
                    .enum-list-selector li.selected {
                        background-color: #e6f7ff;
                        border-color: #1890ff;
                    }
                    .start-section {
                        text-align: center;
                    }
                    .start-section button {
                        padding: 10px 20px;
                        font-size: 16px;
                        background-color: #1890ff;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    }
                    .start-section button:disabled {
                        background-color: #d9d9d9;
                        cursor: not-allowed;
                    }
                    .quiz-section {
                        width: 100%;
                        max-width: 500px;
                    }
                    .quiz-header {
                        display: flex;
                        justify-content: space-between;
                        margin-bottom: 20px;
                    }
                    .question {
                        text-align: center;
                    }
                    .question h2 {
                        font-size: 24px;
                        margin-bottom: 20px;
                    }
                    .question form {
                        display: flex;
                        gap: 10px;
                        justify-content: center;
                    }
                    .question input {
                        padding: 10px;
                        font-size: 16px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        width: 100px;
                    }
                    .question button {
                        padding: 10px 20px;
                        background-color: #1890ff;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    }
                    .result-section {
                        text-align: center;
                    }
                    .result-section .success {
                        color: #52c41a;
                    }
                    .result-section .failure {
                        color: #f5222d;
                    }
                    .result-section button {
                        padding: 10px 20px;
                        background-color: #1890ff;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                        margin-top: 20px;
                    }
                "#}
            </style>
        </div>
    }
}

// Function to generate quiz questions based on the selected course
fn generate_questions(course: Quiz, count: usize) -> Vec<Question> {
    let mut rng = rng();
    let mut questions = Vec::with_capacity(count);

    match course {
        Quiz::Addition => {
            for _ in 0..count {
                let a = rng.random_range(1..=5);
                let b = rng.random_range(1..=5);
                questions.push(Question::new(a, b, "+", a + b));
            }
        }
        Quiz::Subtraction => {
            for _ in 0..count {
                let a = rng.random_range(10..=100);
                let b = rng.random_range(1..=a); // Ensure b is not greater than a
                questions.push(Question::new(a, b, "-", a - b));
            }
        }
        Quiz::Multiplication => {
            for _ in 0..count {
                let a = rng.random_range(1..=12);
                let b = rng.random_range(1..=12);
                questions.push(Question::new(a, b, "*", a * b));
            }
        }
        Quiz::NoCourse => {
            // Return empty questions if no course is selected
        }
    }

    questions
}

fn main() {
    console::log_1(&"Quiz application starting...".into());
    yew::Renderer::<App>::new().render();
}
