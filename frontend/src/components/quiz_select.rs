use crate::quiz::{Quiz, Subject};
use std::{collections::BTreeMap, rc::Rc};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct EnumListSelectorProps {
    pub options: Rc<Vec<Quiz>>,
    pub selected: Quiz,
    pub on_change: Callback<Quiz>,
}

#[derive(Properties, PartialEq)]
pub struct QuizSelectionProps {
    pub options: Rc<Vec<Quiz>>,
    pub selected: Quiz,
    pub on_change: Callback<Quiz>,
}

#[function_component(QuizSelect)]
pub fn quiz_select(props: &QuizSelectionProps) -> Html {
    // State to track which subject sections are expanded (true = expanded, false = collapsed)
    // Initialize all sections as collapsed (false)
    let collapsed_states = use_state(|| {
        let map = BTreeMap::new();
        // Optional: Initialize based on existing difficulties, or leave empty
        // If left empty, sections will default to collapsed when first encountered.
        // Or, you can pre-populate if needed:
        // map.insert(Subject::YearOne, false);
        // map.insert(Subject::YearTwo, false);
        // map.insert(Subject::YearThree, false);
        map
    });

    // Group courses by subject
    let quiz_by_subject = use_memo(props.options.clone(), |options| {
        let mut grouped: BTreeMap<Subject, Vec<Quiz>> = BTreeMap::new();
        for course in options.iter() {
            if *course == Quiz::NoCourse {
                // Skip NoCourse from the list
                continue;
            }
            grouped
                .entry(course.subject())
                .or_insert_with(Vec::new)
                .push(course.clone());
        }
        grouped
    });

    html! {
        <div class="quiz-select-container"> // Changed class name for clarity
            <h3>{"Välj en quiz"}</h3>
            {
                quiz_by_subject.iter().map(|(subject, quiz)| {
                    let subject_css_label = match subject {
                        Subject::Addition => "addition",
                        Subject::Subtraction => "subtraction",
                        Subject::Multiplication => "multiplication",
                        Subject::Division => "division",
                        Subject::Number => "number",
                        Subject::Geometry => "geometry",
                        Subject::Statisitics => "statistics",
                        Subject::Algebra => "algebra",
                        Subject::Random => "random",
                    };
                    let subject_name = match subject {
                       Subject::Addition => "Addition",
                        Subject::Subtraction => "Subtraktion",
                        Subject::Multiplication => "Multiplikation",
                        Subject::Division => "Division",
                        Subject::Number => "Tal",
                        Subject::Geometry => "Geometri",
                        Subject::Statisitics => "Statistik",
                        Subject::Algebra => "Algebra",
                        Subject::Random => "Blandat",
                    };

                    let is_expanded = *collapsed_states.get(subject).unwrap_or(&false);

                    let on_toggle = {
                        let collapsed_states = collapsed_states.clone();
                        let subject = *subject;
                        Callback::from(move |_| {
                            let mut new_states = (*collapsed_states).clone();
                            let current_state = new_states.get(&subject).unwrap_or(&false);
                            new_states.insert(subject, !current_state);
                            collapsed_states.set(new_states);
                        })
                    };

                    let arrow = if is_expanded { "▼" } else { "▶" };

                    let section_classes = classes!(
                        "subject-section",
                        format!("subject-{}", subject_css_label),
                        is_expanded.then_some("expanded")
                    );

                    html! {
                        <div class={section_classes}>
                            <h4 class="collapsible-header" onclick={on_toggle}>
                                { arrow } { subject_name }
                            </h4>
                            { if is_expanded {
                                html! {
                                    <ul class="quiz-list">
                                        {
                                            quiz.iter().map(|course| {
                                                let course_value = course.clone();
                                                let on_click = {
                                                    let on_change = props.on_change.clone();
                                                    let course = course_value.clone();
                                                    Callback::from(move |_| {
                                                        on_change.emit(course.clone());
                                                    })
                                                };

                                                let is_selected = &props.selected == course;
                                                // Removed the individual subject class from li,
                                                // it's now on the parent section. Keep .selected.
                                                let item_class = if is_selected { "selected" } else { "" };

                                                html! {
                                                    <li
                                                        class={item_class} // Only apply 'selected' class here
                                                        onclick={on_click}
                                                    >
                                                        {course.to_string()}
                                                        // Keep checkmark logic if needed
                                                        // {if is_selected { " ✓" } else { "" }}
                                                    </li>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </ul>
                                }
                            } else {
                                html! {} // Render nothing if collapsed
                            }}
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
