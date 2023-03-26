use common::model::{
    task::{CreateTask, Task, UpdateTask},
    user::{CreateUser, LoginUser, UpdateUser, User},
};
use uuid::Uuid;

#[derive(Debug)]
pub enum ApiError {
    NotAuthenticated,
    BadRequest,
    Unknown,
}

const BASE_URL: &str = "http://localhost:3000/api/v1";

pub async fn register(request: CreateUser) -> Result<User, ApiError> {
    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/users/register", BASE_URL))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .expect("register user");

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<User>().await.unwrap()),
        reqwest::StatusCode::BAD_REQUEST => Err(ApiError::BadRequest),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

pub async fn login(request: LoginUser) -> Result<User, ApiError> {
    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/users/login", BASE_URL))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .expect("login user");

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<User>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

pub async fn logout() {
    let client = reqwest::Client::new();
    client
        .post(&format!("{}/users/logout", BASE_URL))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await
        .unwrap();
}

pub async fn me() -> Result<User, ApiError> {
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("{}/users/me", BASE_URL))
        .fetch_credentials_include()
        .send()
        .await
        .expect("user details");

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<User>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

pub async fn update_user(request: UpdateUser) -> Result<User, ApiError> {
    let client = reqwest::Client::new();
    let res = client
        .patch(&format!("{}/users/me", BASE_URL))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .expect("updated user");

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<User>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

pub async fn task(uuid: Uuid) -> Result<Task, ApiError> {
    let client = reqwest::Client::new();
    let result = client
        .get(&format!("{}/tasks/{}", BASE_URL, uuid))
        .fetch_credentials_include()
        .send()
        .await
        .expect("task was here");

    match result.status() {
        reqwest::StatusCode::OK => Ok(result.json::<Task>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

pub async fn tasks() -> Result<Vec<Task>, ApiError> {
    let client = reqwest::Client::new();
    let result = client
        .get(&format!("{}/tasks", BASE_URL))
        .fetch_credentials_include()
        .send()
        .await
        .expect("tasks was here");

    match result.status() {
        reqwest::StatusCode::OK => Ok(result.json::<Vec<Task>>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Ok(result.json::<Vec<Task>>().await.unwrap()),
    }
}

pub async fn create_task(request: CreateTask) -> Result<Task, ApiError> {
    let client = reqwest::Client::new();
    let result = client
        .post(&format!("{}/tasks", BASE_URL))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .expect("create task was here");

    match result.status() {
        reqwest::StatusCode::CREATED => Ok(result.json::<Task>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Ok(result.json::<Task>().await.unwrap()),
    }
}

pub async fn update_task(uuid: Uuid, request: UpdateTask) -> Result<Task, ApiError> {
    let client = reqwest::Client::new();
    let result = client
        .patch(&format!("{}/tasks/{}", BASE_URL, uuid))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .expect("update task was here");

    match result.status() {
        reqwest::StatusCode::OK => Ok(result.json::<Task>().await.unwrap()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Ok(result.json::<Task>().await.unwrap()),
    }
}

pub async fn delete_task(uuid: Uuid) -> Result<(), ApiError> {
    let client = reqwest::Client::new();
    let result = client
        .delete(&format!("{}/tasks/{}", BASE_URL, uuid))
        .header("Content-Type", "application/json")
        .fetch_credentials_include()
        .send()
        .await
        .expect("delete task was here");

    match result.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::UNAUTHORIZED => Err(ApiError::NotAuthenticated),
        _ => Err(ApiError::Unknown),
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}
