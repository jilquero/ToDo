use yew::prelude::*;

use crate::{components::app::link::RouteLink, routes::AppRoute};

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
            <ul class="navbar-nav me-auto mb-2 mb-lg-0 mx-auto">
                <li class="nav-item">
                    <RouteLink to={AppRoute::Tasks}><i class="bi bi-list-check"></i>{ "Tasks" }</RouteLink>
                </li>
                <li class="nav-item">
                    <RouteLink to={AppRoute::Home}><i class="bi bi-house"></i>{ "Home" }</RouteLink>
                </li>
                <li class="nav-item">
                    <RouteLink to={AppRoute::About}><i class="bi bi-info-circle"></i>{ "About" }</RouteLink>
                </li>
            </ul>
    }
}
