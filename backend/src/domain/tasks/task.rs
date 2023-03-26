use common::model::task::{CreateTask, TaskState as State, UpdateTask};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Task {
    pub task_uuid: Uuid,
    pub user_uuid: Uuid,
    pub state: TaskState,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(
        user_uuid: Uuid,
        CreateTask {
            state,
            title,
            description,
        }: CreateTask,
    ) -> Self {
        Task {
            task_uuid: Uuid::new_v4(),
            user_uuid,
            state: to_task_state(state),
            title,
            description,
        }
    }

    pub fn update(
        &mut self,
        UpdateTask {
            state,
            title,
            description,
        }: UpdateTask,
    ) {
        self.state = to_task_state(state.unwrap_or_else(|| self.state.clone().into()));
        self.title = title.unwrap_or_else(|| self.title.clone());
        self.description = description.unwrap_or_else(|| self.description.clone());
    }

    pub fn start(&mut self) {
        self.state = TaskState::InProgress;
    }

    pub fn complete(&mut self) {
        self.state = TaskState::Completed;
    }
}

fn to_task_state(state: State) -> TaskState {
    match state {
        State::NotStarted => TaskState::NotStarted,
        State::InProgress => TaskState::InProgress,
        State::Completed => TaskState::Completed,
    }
}
