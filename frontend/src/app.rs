/* use common::{AdditionSettings, Course, MultiplicationSettings};

use yew::prelude::*;

use crate::components::CourseSelector;

#[function_component(App)]
pub fn app() -> Html {
    // initial course with default settings
    let course = use_state(|| Course::Addition(AdditionSettings { max: 10 }));

    // callback to update course
    let on_change = {
        let course = course.clone();
        Callback::from(move |new_course: Course| {
            course.set(new_course);
        })
    };

    html! {
      <div class="container">
        <CourseSelector
            course={(*course).clone()}
            on_change={on_change} />
        <p>{ format!("Selected: {:?}", *course) }</p>
      </div>
    }
}
 */
