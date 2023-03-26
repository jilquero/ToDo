<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#tech-stack">Tech stack</a></li>
    <li><a href="#crates">Crates</a></li>
    <li><a href="#prerequisite">Prerequisite</a></li>
    <li><a href="#run">Run</a></li>
    <li><a href="#designresourcesdesignmd">Design</a></li>
    <li><a href="#usageresourcesusagemd">Usage</a></li>
  </ol>
</details>

## About The Project

This is a simple todo application. You can create and manage account and tasks. Passwords are hashed and stored in postgres database. You can **C**reate **R**ead **U**pdate **D**elete tasks and manage account information. It is built using rust and technologies mentioned below. The error in api module in frontend is imposed by cargo limitation to check different build targets in workspace, functions used there are only available in the wasm32-unknown-unknown target.

# Tech stack

- database: postgres
- migrations: dbmate
- backend: actix web
- session storage: redis
- frontend: yew wasm
- styles: bootstrap sass
- docker

# Crates

- uuid: Uuid v4 generation
- serde: json serialization and deserialization
- argon2: password hashing and verification
- actix-session and identity: user session and authentication
- sqlx: database driver and connection pool
- validator: entities validation sent between frontend and backend
- regex: regex matching
- reqwest: client for sending requests to backend
- yew-router: application routing
- wasm-bindgen: library to to interact with teh browser through a number of crates
- gloo: toolkit for writing web applications with rust and wasm
- yewdux: redux like state management crate for yew
- comrak: markdown parser

# Prerequisite

- rust
  - cargo install wasm-pack
  - cargo install cargo-watch
    // might be needed
    // sudo apt-get install build-essentials
    // sudo apt-get install libssl-dev
    // sudo apt-get install libnet-ssleay-perl
    // sudo apt-get install pkg-config
- npm
- docker

You can also build and run this application entirely in docker so no need to install anything else

# Run

- Preffered method is to run in docker with `make up`

- Different makefile targets:

  - `test`: to test application (in the future)
  - `dev`: runs startup.sh development mode
  - `up`: to build images and run with docker compose
  - `down`: to shutdown application
  - `build`: to build application
  - `connect_to_db`: to run database and connect to it via psql

- On windows you can try:

  - running posgres dbmate and redis with docker:

    - `docker compose -f docker-compose.acyaml up -d postgres`
    - `docker compose -f docker-compose.yaml up dbmate`
    - `docker compose -f docker-compose.yaml up -d redis`

  - backend with
    - `cargo watch run -x`
  - frontent with
    - `npm start`

# [Design](resources/Design.md)

# [Usage](resources/Usage.md)
