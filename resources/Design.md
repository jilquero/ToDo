<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#entities">Etities</a></li>
    <li>
        <a href="#api-endpoints">API Endpoints</a>
        <ol>
            <li><a href="#user-endpoints">User Endpoints</a></li>
            <li><a href="#task-endpoints">Task Endpoints</a></li>
        </ol>
    </li>
    <li><a href="#project-structure">Project Structure</a></li>
  </ol>
</details>

# Entities

### User

```postgresql
create table users (
  user_uuid uuid         default  primary key,
  username  varchar(255) not null,
  email     varchar(255) not null unique,
  password  varchar(255) not null
);
```

### Task

```postgresql
create table tasks (
  task_uuid   uuid          primary key,
  state       TaskState,
  title       varchar(255),
  description varchar(255),
  user_uuid   uuid references users (user_uuid) on delete cascade
);
```

### Task state

```postgresql
create type TaskState as enum('NotStarted','InProgress','Completed');
```

# API Endpoints

**Base path**: /api/v1

## User endpoints

user response:

```json
{
  "user_uuid": "4c28bc86-52d5-4a15-979b-76af54ec4862",
  "username": "username",
  "email": "example@email.com"
}
```

### POST /users/register

create user and log him in

body: create_user

```json
{
  "username": "username",
  "email": "example@email.com",
  "password": "Example123%",
  "password_confirm": "Example123%"
}
```

response: created user

### POST /users/login

login user

body:

```json
{
  "email": "example@email.com",
  "password": "Example123%"
}
```

response: llogged int user

### POST /users/logout

logout user

resposne:

```json
{
    "Logged out"
}
```

### GET /users/me

get logged in user details

response: users

### PATCH /users/me

update logged in user:

body: update_user (null fields wont be updated)

```json
{
  "username": "username",
  "email": "example@email.com",
  "password": null
}
```

reponse: updated user

## Task endpoints

### GET /tasks

get a list of tasks

response:

```json
[
  {
    "task_uuid": "14813c54-c898-4ba3-a417-4ef239ccf805",
    "user_uuid": "4c28bc86-52d5-4a15-979b-76af54ec4862",
    "state": "InProgress",
    "title": "title",
    "description": "description"
  }
]
```

### POST /tasks

create a task

body

```json
{
  "state": "NotStarted",
  "title": "title",
  "description": "description"
}
```

response: created task

### PATCH /tasks/{task_uuid}

update task

fields can be null

available statuses:

- NotStarted
- InProgres
- Completed

body:

```json
{
  "state": "InProgres",
  "title": "new title",
  "description": null
}
```

response: updated task

### DELETE /tasks/{task_uuid}

delete task

reposne:

```json
{
    "Deleted"
}
```

# Project Structure

## Common

library shared by backend and frontend with common models and validation

```text
- model/
    - error.rs
    - task.rs
    - user.rs
```

## Backend

- api: module with handlers
- domain: module with tasks and users domain
- infrastructure: module with config reader and utils for password saving

```text
- src
    - api/
        - task.rs
        - user.rs
    - domain/
        - tasks.rs
        - users.rs
        - error.rs
    - infrastructure/
        - utils.rs
        - config.rs
    - main.rs
- Cargo.toml
- Dockerfile
```

## Frontend

- src: source files
  - api: module for sending requests to backend
  - components: application components
  - routes: different pages of the application
  - store: application state management module
  - app
- static: base html file with links and scripts to redned application in and styles file

```text
- src/
    - api.rs
    - components/
        - app/
            - header.rs
            - nav.rs
        - task/
            - details.rs
            - filter.rs
            - form.rs
    - routes/
        - about.rs
        - home.rs
        - login.rs
        - profile.rs
        - register.rs
        - tasks.rs
    - store.rs
    - app.rs
- static/
    - index.html
    - styles.scss
- Cargo.toml
- Dockerfile
- package.json
```
