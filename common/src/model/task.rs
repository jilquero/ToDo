use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct Task {
    pub task_uuid: Uuid,
    pub user_uuid: Uuid,
    pub state: TaskState,
    #[validate(length(
        min = 5,
        max = 20,
        message = "Title must be between 5 and 20 characters long"
    ))]
    pub title: String,
    #[validate(length(
        min = 5,
        max = 40,
        message = "Description must be between 5 and 40 characters long"
    ))]
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Validate)]
pub struct CreateTask {
    pub state: TaskState,
    #[validate(length(
        min = 5,
        max = 20,
        message = "Title must be between 5 and 20 characters long"
    ))]
    pub title: String,
    #[validate(length(
        min = 5,
        max = 40,
        message = "Description must be between 5 and 40 characters long"
    ))]
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Validate)]
pub struct UpdateTask {
    pub state: Option<TaskState>,
    #[validate(length(
        min = 5,
        max = 20,
        message = "Title must be between 5 and 20 characters long"
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 5,
        max = 40,
        message = "Description must be between 5 and 40 characters long"
    ))]
    pub description: Option<String>,
}
