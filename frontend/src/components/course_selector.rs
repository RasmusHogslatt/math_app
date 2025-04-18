/* use common::{AdditionSettings, Course, MultiplicationSettings};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course: Course,
    pub on_change: Callback<Course>,
}

#[function_component(CourseSelector)]
pub fn course_selector(props: &Props) -> Html {
    let Props { course, on_change } = props.clone();
    let local = use_state(|| course.clone());

    // emit selection changes upstream
    {
        let local = local.clone();
        let on_change = on_change.clone();
        // use new `use_effect_with`, providing the selected value as the dependency
        use_effect_with((*local).clone(), move |selected| {
            on_change.emit(selected.clone());
            || ()
        });
    }

    let on_course_change = {
        let local = local.clone();
        Callback::from(move |e: Event| {
            let choice = e.target_unchecked_into::<HtmlSelectElement>().value();
            let new_course = match choice.as_str() {
                "Addition" => Course::Addition(AdditionSettings { max: 10 }),
                "Multiplication" => Course::Multiplication(MultiplicationSettings { table: 2 }),
                _ => local.clone(),
            };
            local.set(new_course);
        })
    };

    html! {
        <div class="course-selector">
            <label for="course">{ "Course:" }</label>
            <select id="course" onchange={on_course_change}>
                <option value="Addition" selected={matches!(*local, Course::Addition(_))}>{ "Addition" }</option>
                <option value="Multiplication" selected={matches!(*local, Course::Multiplication(_))}>{ "Multiplication" }</option>
            </select>

            { if let Course::Multiplication(cfg) = &*local {
                let on_table_input = {
                    let local = local.clone();
                    Callback::from(move |e: InputEvent| {
                        let v = e.target_unchecked_into::<HtmlInputElement>().value_as_number() as u8;
                        local.set(Course::Multiplication(MultiplicationSettings { table: v }));
                    })
                };
                html! {
                    <div>
                        <label for="table">{ "Table:" }</label>
                        <input id="table" type="number" min="1" max="12" value={cfg.table.to_string()} oninput={on_table_input} />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
 */
