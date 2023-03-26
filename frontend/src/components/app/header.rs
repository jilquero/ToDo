use yew::prelude::*;

use crate::{
    components::app::{link::RouteLink, nav::Nav},
    routes::AppRoute,
};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="header navbar navbar-expand-lg navbar-dark">
            <div class="container-fluid justify-content-between">
                <a class="header__title navbar-brand" href="">{"{Todos}"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
                    <Nav />
                    <ul class="navbar-nav mb-2 mb-lg-0">
                        <li class="nav-item header__user">
                            <RouteLink to={AppRoute::Profile}><i class="bi bi-person-circle"></i></RouteLink>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
