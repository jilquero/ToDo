use std::ops::Deref;

use common::model::user::CreateUser;
use gloo_utils::document;
use validator::Validate;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{Redirect, Link};
use yewdux::prelude::*;

use crate::{
    api,
    routes::AppRoute,
    store::{register_user, Store},
};

#[function_component(Register)]
pub fn register() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let state = use_state(|| CreateUser::default());

    let onchange_email = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(CreateUser {
                email: value,
                ..state.deref().clone()
            });
        }
    };

    let onchange_username = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(CreateUser {
                username: value,
                ..state.deref().clone()
            });
        }
    };

    let onchange_password = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(CreateUser {
                password: value,
                ..state.deref().clone()
            });
        }
    };

    let onchange_password_confirm = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(CreateUser {
                password_confirm: value,
                ..state.deref().clone()
            });
        }
    };

    let onsubmit = {
        let dispatch = dispatch.clone();
        let state = state.clone();

        move |e: MouseEvent| {
            e.prevent_default();
            let dispatch = dispatch.clone();
            let state = state.clone();

            spawn_local(async move {
                match validate_inputs(state.deref().clone()) {
                    Some(user) => {
                        let user = api::register(user).await;

                        match user {
                            Ok(user) => {
                                register_user(dispatch.clone(), user);
                            }
                            Err(e) => {
                                let input = document()
                                    .get_element_by_id(&format!("register__{}", "email"))
                                    .unwrap();
                                let error = document()
                                    .get_element_by_id(&format!("register__{}__error", "email"))
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

    html! {
        <>
        {match store.user {
            Some(_) => html! {<Redirect<AppRoute> to={AppRoute::Profile} />},
            None => html! {}
        }}
        <form action="/" class="register">
            <h1 class="register__title">{"Register"}</h1>
            <div class="register__input input-control form-floating mb-4">
                <input id="register__username" name="username" type="text" class="form-control" placeholder=""
                    value={state.username.clone()} onchange={onchange_username} />
                <label for="register__username">{"Username"}</label>
                <div id="register__username__error" class="error"></div>
            </div>
            <div class="register__input input-control form-outline form-floating mb-4">
                <input id="register__email" name="email" type="text" class="form-control" placeholder=""
                    value={state.email.clone()} onchange={onchange_email} />
                <label for="register__email">{"Email"}</label>
                <div id="register__email__error" class="error"></div>
            </div>
            <div class="register__input input-control form-outline form-floating mb-4">
                <input id="register__password" name="password" type="password" class="form-control" placeholder=""
                    value={state.password.clone()} onchange={onchange_password} />
                <label for="register__password">{"Password"}</label>
                <div id="register__password__error" class="error"></div>
            </div>
            <div class="register__input input-control form-outline form-floating mb-4">
                <input id="register__password_confirm" name="password_confirm" type="password" class="form-control"
                    placeholder="" value={state.password_confirm.clone()} onchange={onchange_password_confirm} />
                <label for="register__password_confirm">{"Password confirm"}</label>
                <div id="register__password_confirm__error" class="error"></div>
            </div>
            <div class="register__login">{"Already have an account? Login "}
                <Link<AppRoute> classes="register__login" to={AppRoute::Login}>{"here"}</Link<AppRoute>>
            </div>
            <button class="register__submit" type="submit" onclick={onsubmit}>{"Sign Up"}</button>
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

fn validate_inputs(create_user: CreateUser) -> Option<CreateUser> {
    let fields = vec!["email", "username", "password", "password_confirm"];

    for field in fields.iter() {
        let input = document()
            .get_element_by_id(&format!("register__{}", field))
            .unwrap();
        let error = document()
            .get_element_by_id(&format!("register__{}__error", field))
            .unwrap();

        error.set_inner_html("");
        input.set_class_name("form-control is-valid");
    }

    match create_user.validate() {
        Ok(_) => Some(create_user),
        Err(e) => {
            for field in fields.iter() {
                let input = document()
                    .get_element_by_id(&format!("register__{}", field))
                    .unwrap();
                let error = document()
                    .get_element_by_id(&format!("register__{}__error", field))
                    .unwrap();

                match e.field_errors().get(field) {
                    Some(value) => {
                        let error_messages = value
                            .iter()
                            .map(|e| e.message.as_ref().unwrap().as_ref())
                            .collect::<Vec<&str>>()
                            .join(" ");

                        error.set_inner_html(&error_messages);
                        input.set_class_name("form-control is-invalid");
                    }
                    None => {}
                }
            }
            None
        }
    }
}
