use crate::store::Filter as FilterEnum;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct FilterProps {
    pub filter: FilterEnum,
    pub selected: bool,
    pub onset_filter: Callback<FilterEnum>,
}

#[function_component(Filter)]
pub fn filter(props: &FilterProps) -> Html {
    let filter = props.filter;

    let cls = if props.selected {
        "selected"
    } else {
        "not-selected"
    };

    let onset_filter = {
        let onset_filter = props.onset_filter.clone();
        move |_| onset_filter.emit(filter)
    };

    html! {
        <li class="filters__item">
            <a class={format!("hidden-anchor filters__item__link {}", cls)}
               href={props.filter.as_href()}
               onclick={onset_filter}
            >
                { props.filter }
            </a>
        </li>
    }
}
