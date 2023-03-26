use std::ops::Deref;

use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlFormElement, HtmlInputElement, MouseEvent};
use yew::{function_component, html, use_state, Callback, Html, InputEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
    pub onadd: Callback<(String, String)>,
}

#[function_component(TaskForm)]
pub fn task_item(props: &TodoFormProps) -> Html {
    let title = use_state(|| "".to_string());
    let description = use_state(|| "".to_string());

    let on_title_change = {
        let title = title.clone();

        move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            title.set(value);
        }
    };

    let on_description_change = {
        let description = description.clone();

        move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            description.set(value);
        }
    };

    let onadd = {
        let onadd = props.onadd.clone();
        let title = title.clone();
        let description = description.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            let form = document()
                .get_element_by_id("task__form")
                .unwrap()
                .unchecked_into::<HtmlFormElement>();

            if !form.report_validity() {
                return;
            };

            onadd.emit((title.deref().clone(), description.deref().clone()));
            title.set("".to_string());
            description.set("".to_string());
        }
    };

    html! {
        <form class="task__form" id="task__form" action="/">
            <div class="task__form__input">
                <input
                    id="task__form__input__title"
                    class="task__form__input__title"
                    placeholder="Title"
                    value={title.deref().clone()}
                    oninput={on_title_change}
                    minlength="5"
                    maxlength="20"
                    required=true
                />
                <input
                    id="task__form__input__description"
                    class="task__form__input__description"
                    placeholder="Description"
                    value={description.deref().clone()}
                    oninput={on_description_change}
                    minlength="5"
                    maxlength="40"
                    required=true
                />
            </div>
            <label class="task__form__label">
                <button type="submit" class="task__form__submit hidden-btn" onclick={onadd} />
                <i class="bi bi-plus-circle"></i>
            </label>
        </form>
    }
}

fn get_value(event: Event) -> String {
    event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value()
}

// let form = document()
//     .get_element_by_id("task__details_form")
//     .unwrap()
//     .unchecked_into::<HtmlFormElement>();

// form.report_validity();

// let input1 = title
//     .cast::<HtmlInputElement>()
//     .expect("div_ref not attached to div element");

// let input2 = description
//     .cast::<HtmlInputElement>()
//     .expect("div_ref not attached to div element");

// let title = input1.value();
// let description = input2.value();

// let title = title.trim();
// if title.len() > 5 || title.len() < 20 {
//     log!("Title must be between 5 and 20 characters");
//     return;
// }

// let description = description.trim();
// if description.len() > 5 || description.len() < 40 {
//     log!("Description must be between 5 and 40 characters");
//     return;
// }
