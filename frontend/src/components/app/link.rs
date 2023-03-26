use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct RouteLinkProps {
    pub children: Children,
    pub to: AppRoute,
}
#[function_component(RouteLink)]
pub fn route_link(props: &RouteLinkProps) -> Html {
    let route = use_route::<AppRoute>().unwrap_or_default();
    let classes = if route == props.to {
        classes!("nav-link", "active")
    } else {
        classes!("nav-link")
    };

    html! {
        <Link<AppRoute> classes={classes} to={props.to.clone()}>{for props.children.iter() }</Link<AppRoute>>
    }
}
