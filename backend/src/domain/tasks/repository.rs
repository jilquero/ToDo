use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{error::Error, tasks::task::Task};

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}

impl Repository {
    pub async fn find_by_user_uuid(
        &self,
        pool: Pool<Postgres>,
        uuid: Uuid,
    ) -> Result<Vec<Task>, Error> {
        let query = "select * from tasks where user_uuid = $1";

        match sqlx::query_as::<_, Task>(query)
            .bind(uuid)
            .fetch_all(&pool)
            .await
        {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }

    pub async fn find(&self, pool: Pool<Postgres>) -> Result<Vec<Task>, Error> {
        let query = "select * from tasks";

        match sqlx::query_as::<_, Task>(query).fetch_all(&pool).await {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }

    pub async fn find_by_uuid(
        &self,
        pool: Pool<Postgres>,
        uuid: uuid::Uuid,
    ) -> Result<Task, Error> {
        let query = "select * from tasks where task_uuid = $1";

        match sqlx::query_as::<_, Task>(query)
            .bind(uuid)
            .fetch_one(&pool)
            .await
        {
            Ok(task) => Ok(task),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(Error::NotFound),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    pub async fn save(&self, pool: Pool<Postgres>, entity: Task) -> Result<Task, Error> {
        let query = "insert into tasks
        values ($1, $2, $3, $4, $5)
        returning *";

        match sqlx::query_as::<_, Task>(query)
            .bind(entity.task_uuid)
            .bind(entity.state)
            .bind(entity.title)
            .bind(entity.description)
            .bind(entity.user_uuid)
            .fetch_one(&pool)
            .await
        {
            Ok(task) => Ok(task),
            Err(sqlx::Error::Database(e)) => match e.code().as_deref() {
                Some("23505") => Err(Error::NotUnique(e.to_string())),
                _ => Err(Error::Other(e.to_string())),
            },
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(Error::NotFound),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    pub async fn update(&self, pool: Pool<Postgres>, entity: Task) -> Result<Task, Error> {
        let query = "update tasks
        set state = $1, title = $2, description = $3, user_uuid = $4
        where task_uuid = $5
        returning *";

        match sqlx::query_as::<_, Task>(query)
            .bind(entity.state)
            .bind(entity.title)
            .bind(entity.description)
            .bind(entity.user_uuid)
            .bind(entity.task_uuid)
            .fetch_one(&pool)
            .await
        {
            Ok(task) => Ok(task),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }

    pub async fn delete(&self, pool: Pool<Postgres>, uuid: uuid::Uuid) -> Result<(), Error> {
        let query = "delete from tasks where task_uuid = $1";

        match sqlx::query(query).bind(uuid).execute(&pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }
}
