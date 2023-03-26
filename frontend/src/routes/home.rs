use yew::prelude::*;
use yew_hooks::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let parsed = Html::from_html_unchecked(AttrValue::from(
        r#"

        <h1>Entities</h1>
        <h3>User</h3>
        <pre lang="postgresql" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">create table users (
        </span><span style="color:#c0c5ce;">  user_uuid uuid         default  primary key,
        </span><span style="color:#c0c5ce;">  username  varchar(255) not null,
        </span><span style="color:#c0c5ce;">  email     varchar(255) not null unique,
        </span><span style="color:#c0c5ce;">  password  varchar(255) not null
        </span><span style="color:#c0c5ce;">);
        </span>
        </code></pre>
        <h3>Task</h3>
        <pre style="background-color:#2b303b;" lang="postgresql"><code>
        <span style="color:#c0c5ce;">create table tasks (
        </span><span style="color:#c0c5ce;">  task_uuid   uuid          primary key,
        </span><span style="color:#c0c5ce;">  state       TaskState,
        </span><span style="color:#c0c5ce;">  title       varchar(255),
        </span><span style="color:#c0c5ce;">  description varchar(255),
        </span><span style="color:#c0c5ce;">  user_uuid   uuid references users (user_uuid) on delete cascade
        </span><span style="color:#c0c5ce;">);
        </span>
        </code></pre>
        <h3>Task state</h3>
        <pre style="background-color:#2b303b;" lang="postgresql"><code>
        <span style="color:#c0c5ce;">create type TaskState as enum(&#39;NotStarted&#39;,&#39;InProgress&#39;,&#39;Completed&#39;);
        </span>
        </code></pre>
        <h1>API Endpoints</h1>
        <p><strong>Base path</strong>: /api/v1</p>
        <h2>User endpoints</h2>
        <p>user response:</p>
        <pre lang="json" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">user_uuid</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">4c28bc86-52d5-4a15-979b-76af54ec4862</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">email</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">example@email.com</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <h3>POST /users/register</h3>
        <p>create user and log him in</p>
        <p>body: create_user</p>
        <pre style="background-color:#2b303b;" lang="json"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">email</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">example@email.com</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">password</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">Example123%</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">password_confirm</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">Example123%</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <p>response: created user</p>
        <h3>POST /users/login</h3>
        <p>login user</p>
        <p>body:</p>
        <pre lang="json" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">email</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">example@email.com</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">password</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">Example123%</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <p>response: llogged int user</p>
        <h3>POST /users/logout</h3>
        <p>logout user</p>
        <p>resposne:</p>
        <pre style="background-color:#2b303b;" lang="json"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">Logged out</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <h3>GET /users/me</h3>
        <p>get logged in user details</p>
        <p>response: users</p>
        <h3>PATCH /users/me</h3>
        <p>update logged in user:</p>
        <p>body: update_user (null fields wont be updated)</p>
        <pre style="background-color:#2b303b;" lang="json"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">username</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">email</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">example@email.com</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">password</span><span style="color:#c0c5ce;">&quot;: </span><span style="color:#d08770;">null
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <p>reponse: updated user</p>
        <h2>Task endpoints</h2>
        <h3>GET /tasks</h3>
        <p>get a list of tasks</p>
        <p>response:</p>
        <pre lang="json" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">[
        </span><span style="color:#c0c5ce;">  {
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">task_uuid</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">14813c54-c898-4ba3-a417-4ef239ccf805</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">user_uuid</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">4c28bc86-52d5-4a15-979b-76af54ec4862</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">state</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">InProgress</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">title</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">title</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">description</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">description</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">  }
        </span><span style="color:#c0c5ce;">]
        </span>
        </code></pre>
        <h3>POST /tasks</h3>
        <p>create a task</p>
        <p>body</p>
        <pre lang="json" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">state</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">NotStarted</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">title</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">title</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">description</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">description</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <p>response: created task</p>
        <h3>PATCH /tasks/{task_uuid}</h3>
        <p>update task</p>
        <p>fields can be null</p>
        <p>available statuses:</p>
        <ul>
            <li>NotStarted</li>
            <li>InProgres</li>
            <li>Completed</li>
        </ul>
        <p>body:</p>
        <pre lang="json" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">state</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">InProgres</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">title</span><span style="color:#c0c5ce;">&quot;: &quot;</span><span style="color:#a3be8c;">new title</span><span style="color:#c0c5ce;">&quot;,
        </span><span style="color:#c0c5ce;">  &quot;</span><span style="color:#a3be8c;">description</span><span style="color:#c0c5ce;">&quot;: </span><span style="color:#d08770;">null
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <p>response: updated task</p>
        <h3>DELETE /tasks/{task_uuid}</h3>
        <p>delete task</p>
        <p>reposne:</p>
        <pre style="background-color:#2b303b;" lang="json"><code>
        <span style="color:#c0c5ce;">{
        </span><span style="color:#c0c5ce;">    &quot;</span><span style="color:#a3be8c;">Deleted</span><span style="color:#c0c5ce;">&quot;
        </span><span style="color:#c0c5ce;">}
        </span>
        </code></pre>
        <h1>Project Structure</h1>
        <h2>Common</h2>
        <p>library shared by backend and frontend with common models and validation</p>
        <pre lang="text" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">- model/
        </span><span style="color:#c0c5ce;">    - error.rs
        </span><span style="color:#c0c5ce;">    - task.rs
        </span><span style="color:#c0c5ce;">    - user.rs
        </span>
        </code></pre>
        <h2>Backend</h2>
        <ul>
            <li>api: module with handlers</li>
            <li>domain: module with tasks and users domain</li>
            <li>infrastructure: module with config reader and utils for password saving</li>
        </ul>
        <pre style="background-color:#2b303b;" lang="text"><code>
        <span style="color:#c0c5ce;">- src
        </span><span style="color:#c0c5ce;">    - api/
        </span><span style="color:#c0c5ce;">        - task.rs
        </span><span style="color:#c0c5ce;">        - user.rs
        </span><span style="color:#c0c5ce;">    - domain/
        </span><span style="color:#c0c5ce;">        - tasks.rs
        </span><span style="color:#c0c5ce;">        - users.rs
        </span><span style="color:#c0c5ce;">        - error.rs
        </span><span style="color:#c0c5ce;">    - infrastructure/
        </span><span style="color:#c0c5ce;">        - utils.rs
        </span><span style="color:#c0c5ce;">        - config.rs
        </span><span style="color:#c0c5ce;">    - main.rs
        </span><span style="color:#c0c5ce;">- Cargo.toml
        </span><span style="color:#c0c5ce;">- Dockerfile
        </span>
        </code></pre>
        <h2>Frontend</h2>
        <ul>
            <li>src: source files
                <ul>
                    <li>api: module for sending requests to backend</li>
                    <li>components: application components</li>
                    <li>routes: different pages of the application</li>
                    <li>store: application state management module</li>
                    <li>app</li>
                </ul>
            </li>
            <li>static: base html file with links and scripts to redned application in and styles file</li>
        </ul>
        <pre lang="text" style="background-color:#2b303b;"><code>
        <span style="color:#c0c5ce;">- src/
        </span><span style="color:#c0c5ce;">    - api.rs
        </span><span style="color:#c0c5ce;">    - components/
        </span><span style="color:#c0c5ce;">        - app/
        </span><span style="color:#c0c5ce;">            - header.rs
        </span><span style="color:#c0c5ce;">            - nav.rs
        </span><span style="color:#c0c5ce;">        - task/
        </span><span style="color:#c0c5ce;">            - details.rs
        </span><span style="color:#c0c5ce;">            - filter.rs
        </span><span style="color:#c0c5ce;">            - form.rs
        </span><span style="color:#c0c5ce;">    - routes/
        </span><span style="color:#c0c5ce;">        - about.rs
        </span><span style="color:#c0c5ce;">        - home.rs
        </span><span style="color:#c0c5ce;">        - login.rs
        </span><span style="color:#c0c5ce;">        - profile.rs
        </span><span style="color:#c0c5ce;">        - register.rs
        </span><span style="color:#c0c5ce;">        - tasks.rs
        </span><span style="color:#c0c5ce;">    - store.rs
        </span><span style="color:#c0c5ce;">    - app.rs
        </span><span style="color:#c0c5ce;">- static/
        </span><span style="color:#c0c5ce;">    - index.html
        </span><span style="color:#c0c5ce;">    - styles.scss
        </span><span style="color:#c0c5ce;">- Cargo.toml
        </span><span style="color:#c0c5ce;">- Dockerfile
        </span><span style="color:#c0c5ce;">- package.json
        </span>
        </code></pre>       

        "#,
    ));

    html! {
        <div class="markdown">
            {parsed}
        </div>
    }
}
