use std::ops::Deref;

use common::model::user::UpdateUser;
use gloo_utils::document;
use validator::Validate;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::prelude::*;

use crate::{
    api::{self},
    routes::AppRoute,
    store::{logout_user, update_user, Store},
};

#[derive(PartialEq, Properties)]
pub struct ProfileProps {}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let edit = use_state(|| false);
    let state = use_state(|| {
        if let Some(user) = &store.user {
            UpdateUser {
                username: Some(store.user.as_ref().unwrap().username.clone()),
                email: Some(store.user.as_ref().unwrap().email.clone()),
                password: None,
            }
        } else {
            UpdateUser::default()
        }
    });

    let onedit = {
        let edit = edit.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            edit.set(true)
        }
    };

    let onchange_username = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(UpdateUser {
                username: Some(value),
                ..state.deref().clone()
            });
        }
    };

    let onchange_email = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(UpdateUser {
                email: Some(value),
                ..state.deref().clone()
            });
        }
    };

    let onchange_password = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(UpdateUser {
                password: if value.is_empty() { None } else { Some(value) },
                ..state.deref().clone()
            });
        }
    };

    let onsave = {
        let edit = edit.clone();
        let dispatch = dispatch.clone();
        let state = state.clone();

        move |e: MouseEvent| {
            e.prevent_default();
            let edit = edit.clone();
            let dispatch = dispatch.clone();
            let state = state.clone();

            spawn_local(async move {
                match validate_inputs(state.deref().clone()) {
                    Some(user) => {
                        let user = api::update_user(user).await;

                        match user {
                            Ok(user) => {
                                update_user(dispatch, user);
                                edit.set(false);
                                clear_inputs();
                            }
                            Err(e) => {
                                let input = document()
                                    .get_element_by_id(&format!("profile__{}", "email"))
                                    .unwrap();
                                let error = document()
                                    .get_element_by_id(&format!("profile__{}__error", "email"))
                                    .unwrap();

                                error.set_inner_html("Email already exists");
                                input.set_class_name("form-control is-invalid");
                            }
                        }
                    }
                    None => {}
                }
            });
        }
    };

    let oncancel = {
        let edit = edit.clone();
        let dispatch = dispatch.clone();
        let store = store.clone();
        let state = state.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            state.set(UpdateUser {
                username: Some(store.user.as_ref().to_owned().unwrap().username.clone()),
                email: Some(store.user.as_ref().to_owned().unwrap().email.clone()),
                password: None,
            });

            edit.set(false);
            clear_inputs();
        }
    };

    let onlogout = {
        let dispatch = dispatch.clone();

        move |e: MouseEvent| {
            e.prevent_default();
            let dispatch = dispatch.clone();

            spawn_local(async move {
                api::logout().await;
                logout_user(dispatch);
            })
        }
    };

    html! {
        <>
        {match store.user {
            Some(_) => html! {},
            None => html! {<Redirect<AppRoute> to={AppRoute::Login} />}
        }}
        <form action="/" class="profile">
            <h1 class="profile__title">{"Profile"}</h1>
            <div class="profile__details">
                <div class="profile__details__row row mb-3">
                    <label for="profile__username" class="col col-form-label">{"Username"}</label>
                    <div class="col-auto">
                        <input id="profile__username" type="text" class="form-control" aria-label="First name" value={state.username.clone()} readonly={!*edit.deref()} onchange={onchange_username} />
                        <div id="profile__username__error" class="error"></div>
                    </div>
                </div>
                <div class="profile__details__row row mb-3">
                    <label for="profile__email" class="col col-form-label">{"Email"}</label>
                    <div class="col-auto">
                        <input id="profile__email" type="text" class="form-control" aria-label="First name" value={state.email.clone()} readonly={!*edit.deref()} onchange={onchange_email} />
                        <div id="profile__email__error" class="error"></div>
                    </div>
                </div>
                {if *edit.deref() {
                    html! {
                        <>
                        <div class="profile__details__row row mb-3">
                            <label for="profile__password" class="col col-form-label">{"Password"}</label>
                            <div class="col-auto">
                                <input id="profile__password" type="password" class="form-control" aria-label="First name" value={state.password.clone()} onchange={onchange_password} />
                               <div id="profile__password__error" class="error"></div>
                            </div>
                        </div>
                        </>
                    }
                } else {
                    html! {}
                }}
                <div class="profile__actions row mb-3">
                    {if *edit.deref() {
                        html! {
                            <>
                            <div class="col-auto">
                                <button class="profile__actions__button" type="button" onclick={onsave}>{"Save"}</button>
                            </div>
                            <div class="col-auto">
                                <button class="profile__actions__button" type="button" onclick={oncancel}>{"Cancel"}</button>
                            </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="col-auto">
                                <button class="profile__actions__button" type="submit" onclick={onedit}>{"Edit"}</button>
                            </div>
                        }
                    }}
                    <div class="col-auto">
                        <button class="profile__actions__button" type="submit" onclick={onlogout}>{"Logout"}</button>
                    </div>
                </div>
            </div>
        </form>
        </>
    }
}

fn get_value(event: Event) -> String {
    event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value()
}

fn set_by_id(element: &str, class: Option<&str>, message: Option<&str>) {
    let elem = document().get_element_by_id(element).unwrap();
    elem.set_class_name(class.unwrap_or_else(|| ""));
    elem.set_inner_html(message.unwrap_or_else(|| ""));
}

fn validate_inputs(update_user: UpdateUser) -> Option<UpdateUser> {
    let fields = vec!["email", "username", "password"];

    for field in fields.iter() {
        set_by_id(
            &format!("profile__{}", field),
            Some("form-control is-valid"),
            None,
        );
        set_by_id(&format!("profile__{}__error", field), Some("error"), None);
    }

    match update_user.validate() {
        Ok(_) => Some(update_user),
        Err(e) => {
            for field in fields.iter() {
                match e.field_errors().get(field) {
                    Some(value) => {
                        let error_messages = value
                            .iter()
                            .map(|e| e.message.as_ref().unwrap().as_ref())
                            .collect::<Vec<&str>>()
                            .join(" ");

                        set_by_id(
                            &format!("profile__{}", field),
                            Some("form-control is-invalid"),
                            None,
                        );
                        set_by_id(
                            &format!("profile__{}__error", field),
                            Some("error"),
                            Some(&error_messages),
                        );
                    }
                    None => {}
                }
            }
            None
        }
    }
}

fn clear_inputs() {
    let fields = vec!["email", "username", "password"];

    for field in fields.iter() {
        set_by_id(&format!("profile__{}", field), Some("form-control"), None);
        set_by_id(&format!("profile__{}__error", field), Some("error"), None);
    }
}

// <div class="profile">
//     <div class="profile__info">
//         <h1>{"Profile"}</h1>
//         <label for="name">{"Name"}</label>
//         <input
//             class="profile__info__username"
//             type="text"
//             ame="name"
//             id="name"/>
//         <label for="email">{"Email"}</label>
//         <input
//             class="profile__info__email"
//             type="email"
//             name="email"
//             id="email"/>
//         {if *edit.deref() {
//             html! {
//                 <>
//                 <label for="password">{"Password"}</label>
//                 <input
//                     class="profile__password__password"
//                     type={if show.deref().clone() {"text"} else {"password"}}
//                     name="password"
//                     id="password"/>
//                 <label for="confirm_password">{"Confirm Password"}</label>
//                 <input
//                     class="profile__password__confirm_password"
//                     type={if show.deref().clone() {"text"} else {"password"}}
//                     name="confirm_password"
//                     id="confirm_password"/>
//                 </>
//             }
//         } else {
//             html! {}
//         }}
//     </div>
//     <div class="profile__actions">
//         <label class="profile__actions__label" for="edit">
//             <button
//                 class="profile__actions__button"
//                 name="edit"
//                 id="edit"
//                 onclick={onedit}
//             />
//             <i class="bi bi-pencil-square"></i>
//             {"Edit"}
//         </label>
//         <label class="profile__actions__label" for="save">
//             <button
//                 class="profile__actions__button"
//                 name="save"
//                 id="save"
//                 onclick={onsave}
//             />
//             <i class="bi bi-check-circle"></i>
//             {"Save"}
//         </label>
//         <label class="profile__actions__label" for="logout">
//             <button
//                 class="profile__actions__button"
//                 name="save"
//                 id="logout"
//                 onclick={onlogout}
//             />
//             {"Logout"}
//             <i class="bi bi-box-arrow-right"></i>
//         </label>
//         {if *edit.deref() {
//             html! {
//                 <>
//                 <label class="profile__actions__label">
//                     <button class="profile__actions__button" onclick={onshow} />
//                     {"Show"}
//                     <i class="bi bi-eye-slash"></i>
//                 </label>
//                 </>
//             }
//         } else {
//             html! {}
//         }}
//     </div>
// </div>
