use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="content has-text-centered">
            <p class="block">
               <strong>{ "🦀️ Rust Fullstack Todos Application" }</strong>
            </p>
            <p class="block">
                {"By "}
                <a target="_black" href="https://lexcao.io">{ "Lex Cao" }</a>
            </p>
            <p class="block">
                {" Source code is available on "}
                <a target="_black" href="https://github.com/lexcao/rust_fullstack_todo">{ "GitHub" }</a>
            </p>
        </div>
    }
}
