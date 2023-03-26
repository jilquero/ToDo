use yew::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let parsed = Html::from_html_unchecked(AttrValue::from(
        r#"
    
        <h2>About The Project</h2>
        <p>This is a simple todo application. You can create and manage account and tasks. Passwords are hashed and
            stored in postgres database. You can <strong>C</strong>reate <strong>R</strong>ead
            <strong>U</strong>pdate
            <strong>D</strong>elete tasks and manage account information. It is built using rust and technologies
            mentioned below.</p>
        <h1>Tech stack</h1>
        <ul>
            <li>database: postgres</li>
            <li>migrations: dbmate</li>
            <li>backend: actix web</li>
            <li>session storage: redis</li>
            <li>frontend: yew wasm</li>
            <li>styles: bootstrap sass</li>
            <li>docker</li>
        </ul>
        <h1>Crates</h1>
        <ul>
            <li>uuid: Uuid v4 generation</li>
            <li>serde: json serialization and deserialization</li>
            <li>argon2: password hashing and verification</li>
            <li>actix-session and identity: user session and authentication</li>
            <li>sqlx: database driver and connection pool</li>
            <li>validator: entities validation sent between frontend and backend</li>
            <li>regex: regex matching</li>
            <li>reqwest: client for sending requests to backend</li>
            <li>yew-router: application routing</li>
            <li>wasm-bindgen: library to to interact with teh browser through a number of crates</li>
            <li>gloo: toolkit for writing web applications with rust and wasm</li>
            <li>yewdux: redux like state management crate for yew</li>
            <li>comrak: markdown parser</li>
        </ul>
        <h1>Prerequisite</h1>
        <ul>
            <li>rust
                <ul>
                    <li>cargo install wasm-pack</li>
                    <li>cargo install cargo-watch
                        // might be needed
                        // sudo apt-get install build-essentials
                        // sudo apt-get install libssl-dev
                        // sudo apt-get install libnet-ssleay-perl
                        // sudo apt-get install pkg-config</li>
                </ul>
            </li>
            <li>npm</li>
            <li>docker</li>
        </ul>
        <p>You can also build and run this application entirely in docker so no need to install anything else</p>
        <h1>Run</h1>
        <ul>
            <li>
                <p>Preffered method is to run in docker with <code>make up</code></p>
            </li>
            <li>
                <p>Different makefile targets:</p>
                <ul>
                    <li><code>test</code>: to test application (in the future)</li>
                    <li><code>dev</code>: runs startup.sh development mode</li>
                    <li><code>up</code>: to build images and run with docker compose</li>
                    <li><code>down</code>: to shutdown application</li>
                    <li><code>build</code>: to build application</li>
                    <li><code>connect_to_db</code>: to run database and connect to it via psql</li>
                </ul>
            </li>
            <li>
                <p>On windows you can try:</p>
                <ul>
                    <li>
                        <p>running posgres dbmate and redis with docker:</p>
                        <ul>
                            <li><code>docker compose -f docker-compose.acyaml up -d postgres</code></li>
                            <li><code>docker compose -f docker-compose.yaml up dbmate</code></li>
                            <li><code>docker compose -f docker-compose.yaml up -d redis</code></li>
                        </ul>
                    </li>
                    <li>
                        <p>backend with</p>
                        <ul>
                            <li><code>cargo watch run -x</code></li>
                        </ul>
                    </li>
                    <li>
                        <p>frontent with</p>
                        <ul>
                            <li><code>npm start</code></li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
        <h1><a href="/">Design</a></h1>
        <h1><a href="/">Usage</a></h1>
    "#,
    ));
    html! {
        <div class="markdown">
            {parsed}
        </div>
    }
}
