use crate::{
    api,
    components::task::{details::TaskDetails, form::TaskForm},
    store::{add_task, edit_task, remove_task, set_filter, Filter, Store},
};
use common::model::task::{CreateTask, TaskState, UpdateTask};
use strum::IntoEnumIterator;
use uuid::Uuid;
use validator::Validate;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::task::filter::Filter as FilterItem;

#[function_component(TodoApp)]
pub fn todo_app() -> Html {
    // State
    let (store, dispatch) = use_store::<Store>();

    // Callbacks
    let onadd = {
        let dispatch = dispatch.clone();

        Callback::from(move |(title, description): (String, String)| {
            let dispatch = dispatch.clone();

            if title.is_empty() || description.is_empty() {
                return;
            }

            spawn_local(async move {
                let task = CreateTask {
                    state: TaskState::NotStarted,
                    title,
                    description,
                };

                match task.validate() {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Validation error: {:?}", e);
                        return;
                    }
                }

                let task = api::create_task(task).await.unwrap();
                add_task(dispatch, task);
            })
        })
    };

    let ondelete = {
        let dispatch = dispatch.clone();

        Callback::from(move |uuid| {
            let dispatch = dispatch.clone();

            spawn_local(async move {
                api::delete_task(uuid).await.unwrap();
                remove_task(dispatch, uuid);
            })
        })
    };

    let ontoggle = {
        let dispatch = dispatch.clone();

        Callback::from(move |(uuid, state): (Uuid, TaskState)| {
            let dispatch = dispatch.clone();

            spawn_local(async move {
                let task_state = match state {
                    TaskState::NotStarted => TaskState::InProgress,
                    TaskState::InProgress => TaskState::Completed,
                    TaskState::Completed => TaskState::NotStarted,
                };

                let task = UpdateTask {
                    state: Some(task_state),
                    title: None,
                    description: None,
                };

                match task.validate() {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Validation error: {:?}", e);
                        return;
                    }
                }

                let task = api::update_task(uuid, task).await.unwrap();
                edit_task(dispatch, task);
            })
        })
    };

    let onedit = {
        let dispatch = dispatch.clone();

        Callback::from(move |(uuid, title, description): (Uuid, String, String)| {
            let dispatch = dispatch.clone();

            if title.is_empty() || description.is_empty() {
                return;
            }

            spawn_local(async move {
                let task = UpdateTask {
                    state: None,
                    title: Some(title),
                    description: Some(description),
                };

                match task.validate() {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Validation error: {:?}", e);
                        return;
                    }
                }

                let task = api::update_task(uuid, task).await.unwrap();
                edit_task(dispatch, task);
            })
        })
    };

    let onset_filter = {
        let dispatch = dispatch;

        Callback::from(move |filter: Filter| {
            let dispatch = dispatch.clone();

            spawn_local(async move {
                set_filter(dispatch, filter);
            })
        })
    };

    html! {
        <div class="todo__app">
            <div class="tasks">
                <TaskForm onadd={onadd} />
                <ul class="filters">
                    { for Filter::iter().map(|filter| {
                        html! {
                            <FilterItem {filter} selected={store.filter==filter} onset_filter={onset_filter.clone()} />
                        }
                    })}
                </ul>
                <ul class="tasks__list">
                    { for store.tasks.iter().filter(|e| store.filter.fits(e)).cloned().enumerate().map(|(id, task)|
                        html! {
                            <li class="task-item">
                                <TaskDetails {task}
                                    id={id.to_string()}
                                    edit={false}
                                    ontoggle={ontoggle.clone()}
                                    ondelete={ondelete.clone()}
                                    onedit={onedit.clone()}
                                />
                            </li>
                    }) }
                </ul>
            </div>
        </div>
    }
}
