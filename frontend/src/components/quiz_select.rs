use crate::quiz::{Difficulty, Quiz};
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
    // State to track which difficulty sections are expanded (true = expanded, false = collapsed)
    // Initialize all sections as collapsed (false)
    let collapsed_states = use_state(|| {
        let mut map = BTreeMap::new();
        // Optional: Initialize based on existing difficulties, or leave empty
        // If left empty, sections will default to collapsed when first encountered.
        // Or, you can pre-populate if needed:
        map.insert(Difficulty::YearOne, false);
        map.insert(Difficulty::YearTwo, false);
        map.insert(Difficulty::YearThree, false);
        map
    });

    // Group courses by difficulty
    let courses_by_difficulty = use_memo(props.options.clone(), |options| {
        let mut grouped: BTreeMap<Difficulty, Vec<Quiz>> = BTreeMap::new();
        for course in options.iter() {
            if *course == Quiz::NoCourse {
                // Skip NoCourse from the list
                continue;
            }
            grouped
                .entry(course.difficulty())
                .or_insert_with(Vec::new)
                .push(course.clone());
        }
        grouped
    });

    html! {
        <div class="quiz-select-container"> // Changed class name for clarity
            <h3>{"Choose a course:"}</h3>
            {
                courses_by_difficulty.iter().map(|(difficulty, courses)| {
                    let difficulty_css_label = match difficulty {
                        Difficulty::YearOne => "YearOne",
                        Difficulty::YearTwo => "YearTwo",
                        Difficulty::YearThree => "YearThree",
                        Difficulty::YearFour => "YearFour",
                        Difficulty::YearFive => "YearFive",
                        Difficulty::YearSix => "YearSix",
                        Difficulty::YearSeven => "YearSeven",
                        Difficulty::YearEight => "YearEight",
                        Difficulty::YearNine => "YearNine",
                    };
                    let difficulty_name = match difficulty {
                        Difficulty::YearOne => "Year 1",
                        Difficulty::YearTwo => "Year 2",
                        Difficulty::YearThree => "Year 3",
                        Difficulty::YearFour => "Year 4",
                        Difficulty::YearFive => "Year 5",
                        Difficulty::YearSix => "Year 6",
                        Difficulty::YearSeven => "Year 7",
                        Difficulty::YearEight => "Year 8",
                        Difficulty::YearNine => "Year 9",
                    };

                    let is_expanded = *collapsed_states.get(difficulty).unwrap_or(&false);

                    let on_toggle = {
                        let collapsed_states = collapsed_states.clone();
                        let difficulty = *difficulty;
                        Callback::from(move |_| {
                            let mut new_states = (*collapsed_states).clone();
                            let current_state = new_states.get(&difficulty).unwrap_or(&false);
                            new_states.insert(difficulty, !current_state);
                            collapsed_states.set(new_states);
                        })
                    };

                    let arrow = if is_expanded { "▼" } else { "▶" };

                    let section_classes = classes!(
                        "difficulty-section",
                        format!("difficulty-{}", difficulty_css_label),
                        is_expanded.then_some("expanded")
                    );

                    html! {
                        <div class={section_classes}>
                            <h4 class="collapsible-header" onclick={on_toggle}>
                                { arrow } { difficulty_name }
                            </h4>
                            { if is_expanded {
                                html! {
                                    <ul class="course-list">
                                        {
                                            courses.iter().map(|course| {
                                                let course_value = course.clone();
                                                let on_click = {
                                                    let on_change = props.on_change.clone();
                                                    let course = course_value.clone();
                                                    Callback::from(move |_| {
                                                        on_change.emit(course.clone());
                                                    })
                                                };

                                                let is_selected = &props.selected == course;
                                                // Removed the individual difficulty class from li,
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
            // Keep the current selection display if needed
            <p class="current-selection-display">
                {"Current selection: "}
                { if props.selected != Quiz::NoCourse {
                    props.selected.to_string()
                 } else {
                    "None".to_string()
                 }}
            </p>
        </div>
    }
}
