use common::model::task::{CreateTask, UpdateTask};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::error::Error;

use super::{repository::Repository, task::Task};

#[derive(Debug, Clone)]
pub struct Service {
    repo: Repository,
    pool: Pool<Postgres>,
}

impl Service {
    pub fn new(pool: Pool<Postgres>) -> Service {
        Service {
            repo: Repository::new(),
            pool,
        }
    }
}

impl Service {
    pub async fn find_tasks_by_user(&self, uuid: Uuid) -> Result<Vec<Task>, Error> {
        self.repo.find_by_user_uuid(self.pool.clone(), uuid).await
    }

    pub async fn find(&self) -> Result<Vec<Task>, Error> {
        self.repo.find(self.pool.clone()).await
    }

    pub async fn find_task_by_user(&self, user_uuid: Uuid, task_uuid: Uuid) -> Result<Task, Error> {
        let task = self.find_by_uuid(task_uuid).await?;

        if task.user_uuid == user_uuid {
            Ok(task)
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn find_by_uuid(&self, task_uuid: uuid::Uuid) -> Result<Task, Error> {
        self.repo.find_by_uuid(self.pool.clone(), task_uuid).await
    }

    pub async fn save(&self, user_uuid: Uuid, create_task: CreateTask) -> Result<Task, Error> {
        let task = Task::new(user_uuid, create_task);

        self.repo.save(self.pool.clone(), task).await
    }

    pub async fn update(
        &self,
        user_uuid: Uuid,
        task_uuid: Uuid,
        update_task: UpdateTask,
    ) -> Result<Task, Error> {
        let mut task = self.find_task_by_user(user_uuid, task_uuid).await?;
        task.update(update_task);

        self.repo.update(self.pool.clone(), task).await
    }

    pub async fn delete(&self, user_uuid: Uuid, task_uuid: Uuid) -> Result<(), Error> {
        self.find_task_by_user(user_uuid, task_uuid).await?;

        self.repo.delete(self.pool.clone(), task_uuid).await
    }
}
