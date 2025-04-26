use crate::quiz::{Difficulty, Quiz};
use std::rc::Rc;
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

                        // Get difficulty for styling
                        let difficulty_class = match course.difficulty() {
                            Difficulty::Easy => "difficulty-easy",
                            Difficulty::Medium => "difficulty-medium",
                            Difficulty::Hard => "difficulty-hard",
                        };

                        // Get difficulty label
                        let difficulty_label = match course.difficulty() {
                            Difficulty::Easy => " (Easy)",
                            Difficulty::Medium => " (Medium)",
                            Difficulty::Hard => " (Hard)",
                        };

                        html! {
                            <li
                                class={classes!(class, difficulty_class)}
                                onclick={on_click}
                            >
                                {course.to_string()}{difficulty_label}
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
