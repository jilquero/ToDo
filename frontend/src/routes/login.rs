use std::{ops::Deref, vec};

use common::model::user::LoginUser;
use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{Link, Redirect};
use yewdux::prelude::*;

use crate::{
    api,
    routes::AppRoute,
    store::{login_user, set_tasks, Store},
};

#[function_component(Login)]
pub fn login() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let state = use_state(|| LoginUser::default());

    let onchange_email = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(LoginUser {
                email: value,
                ..state.deref().clone()
            });
        }
    };

    let onchange_password = {
        let state = state.clone();

        move |event: Event| {
            let value = get_value(event);
            state.set(LoginUser {
                password: value,
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
                let user = LoginUser {
                    email: state.email.deref().to_string(),
                    password: state.password.deref().to_string(),
                };

                let user = api::login(user).await;

                match user {
                    Ok(user) => {
                        login_user(dispatch.clone(), user);

                        let tasks = api::tasks().await.unwrap();
                        set_tasks(dispatch.clone(), tasks);
                    }
                    Err(_) => validate_inputs(),
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
        <form action="/" class="login">
            <h1 class="login__title">{"Login"}</h1>
            <div class="login__input input-control form-floating mb-4">
                <input id="login__email" name="email"  type="text" class="form-control" placeholder=""
                    value={state.email.clone()} onchange={onchange_email} />
                <label for="login__email">{"Email"}</label>
                <div id="login__email__error" class="error"></div>
            </div>
            <div class="login__input input-control form-floating mb-4">
                <input id="login__password" name="username" type="password" class="form-control" placeholder=""
                    value={state.password.clone()} onchange={onchange_password} />
                <label for="login__password">{"Password"}</label>
                <div id="login__password__error" class="error"></div>
            </div>
            <div class="login__register">{"Dont have an account? Register "}
                <Link<AppRoute> classes="login__register__link" to={AppRoute::Register}>{"here"}</Link<AppRoute>>
            </div>
            <button class="login__submit" type="submit" onclick={onsubmit}>{"Login"}</button>
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

fn validate_inputs() {
    let fields = vec!["email", "password"];

    for field in fields.iter() {
        let input = document()
            .get_element_by_id(&format!("login__{}", field))
            .unwrap()
            .unchecked_into::<HtmlInputElement>();

        let error = document()
            .get_element_by_id(&format!("login__{}__error", field))
            .unwrap();

        input.set_class_name("form-control is-invalid");
        error.set_inner_html("Invalid email or password");
    }
}
