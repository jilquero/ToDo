use std::ops::Deref;

use common::model::task::{Task as Item, TaskState};
use gloo_console::log;
use gloo_utils::document;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement, MouseEvent};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub task: Item,
    pub id: String,
    pub edit: bool,
    pub ontoggle: Callback<(Uuid, TaskState)>,
    pub ondelete: Callback<Uuid>,
    pub onedit: Callback<(Uuid, String, String)>,
}

#[function_component(TaskDetails)]
pub fn task_details(props: &TaskProps) -> Html {
    let uuid = props.task.task_uuid;
    let id = props.id.clone();
    let edit = use_state(|| props.edit);
    let title = use_node_ref();
    let description = use_node_ref();

    let ontoggle = {
        let ontoggle = props.ontoggle.clone();
        let task_state = props.task.state.clone();
        move |_| ontoggle.emit((uuid, task_state.clone()))
    };

    let ondelete = {
        let ondelete = props.ondelete.clone();
        move |_| ondelete.emit(uuid)
    };

    let onedit = {
        let edit = edit.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            edit.set(true);
        }
    };

    let onsave = {
        let id = props.id.clone();
        let onedit = props.onedit.clone();
        let edit = edit.clone();
        let title = title.clone();
        let description = description.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            let form = document()
                .get_element_by_id(format!("task__details__form__{}", &id).as_str())
                .unwrap()
                .unchecked_into::<HtmlFormElement>();

            if !form.report_validity() {
                return;
            };

            let input1 = title
                .cast::<HtmlInputElement>()
                .expect("div_ref not attached to div element");

            let input2 = description
                .cast::<HtmlInputElement>()
                .expect("div_ref not attached to div element");

            log!("input1: {}", &input1.value());
            log!("input2: {}", &input2.value());

            let title = input1.value();
            let description = input2.value();

            onedit.emit((uuid, title.clone(), description.clone()));
            edit.set(false);
        }
    };

    let oncancel = {
        let onedit = props.onedit.clone();
        let edit = edit.clone();

        move |_| {
            onedit.emit((uuid, "".to_string(), "".to_string()));
            edit.set(false);
        }
    };

    html! {
        //<li class="task">
        <form class="task" action="/" id={format!("task__details__form__{}", &id)}>
            <div class="task__details">
                <div class="task__details__state">
                    { to_icon(&props.task.state) }
                </div>
                <div class="task__details__input">
                    <input
                        ref={title}
                        class="task__details__title form-control"
                        readonly={!*edit.deref()}
                        value={props.task.title.clone()}
                        minlength="5"
                        maxlength="20"
                        required=true
                        />
                    <input
                        ref={description}
                        class="task__details__description form-control"
                        readonly={!*edit.deref()}
                        value={props.task.description.clone()}
                        minlength="5"
                        maxlength="20"
                        required=true
                        />
                </div>
            </div>
            <div class="task__actions">
                    {if *edit.deref() {
                        html! {
                            <>
                            <label class="task__actions__label">
                                <button class="task__actions__save" onclick={onsave} type="submit" />
                                <i class="bi bi-check-circle"></i>
                            </label>
                            <label class="task__actions__label">
                                <button class="task__actions__cancel" onclick={oncancel} type="button" />
                                <i class="bi bi-x-circle"></i>
                            </label>
                            </>
                        }
                    } else {
                        html! {
                            <>
                            <label class="task__actions__label">
                                <button class="task__actions__complete" onclick={ontoggle} type="button" />
                                <i class="bi bi-arrow-left-right"></i>
                            </label>
                            <label class="task__actions__label">
                                <button class="task__actions__edit" onclick={onedit} type="button" />
                                <i class="bi bi-pencil-square"></i>
                            </label>
                            <label class="task__actions__label">
                                <button class="task__actions__delete" onclick={ondelete} type="button" />
                                <i class="bi bi-x-circle"></i>
                            </label>
                            </>
                        }
                    }}
            </div>
        </form>
        //</li>
    }
}

fn to_icon(state: &TaskState) -> Html {
    match state {
        TaskState::NotStarted => html! {<i class="bi bi-pause-circle"></i>},
        TaskState::InProgress => html! {<i class="bi bi-arrow-clockwise"></i>},
        TaskState::Completed => html! {<i class="bi bi-check2-circle"></i>},
    }
}
