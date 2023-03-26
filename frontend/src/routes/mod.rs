use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
mod login;
mod profile;
mod register;
mod tasks;

use about::About;
use home::Home;
use login::Login;
use profile::Profile;
use register::Register;
use tasks::TodoApp;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile")]
    Profile,
    #[at("/tasks")]
    Tasks,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Login => html! { <Login /> },
        AppRoute::Register => html! { <Register /> },
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::Tasks => html! { <TodoApp /> },
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
