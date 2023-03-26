use sqlx::{Pool, Postgres};

use crate::domain::{error::Error, users::user::User};

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}

impl Repository {
    pub async fn find(&self, pool: Pool<Postgres>) -> Result<Vec<User>, Error> {
        let query = "select * from users";

        match sqlx::query_as::<_, User>(query).fetch_all(&pool).await {
            Ok(users) => Ok(users),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }

    pub async fn find_by_email(&self, pool: Pool<Postgres>, email: String) -> Result<User, Error> {
        let query = "select * from users where email = $1";

        match sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_one(&pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(Error::NotFound),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    pub async fn find_by_uuid(
        &self,
        pool: Pool<Postgres>,
        uuid: uuid::Uuid,
    ) -> Result<User, Error> {
        let query = "select * from users where user_uuid = $1";

        match sqlx::query_as::<_, User>(query)
            .bind(uuid)
            .fetch_one(&pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(Error::NotFound),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    pub async fn save(&self, pool: Pool<Postgres>, entity: User) -> Result<User, Error> {
        let query = "insert into users
        values ($1, $2, $3, $4)
        returning *";

        match sqlx::query_as::<_, User>(query)
            .bind(entity.user_uuid)
            .bind(entity.username)
            .bind(entity.email)
            .bind(entity.password)
            .fetch_one(&pool)
            .await
        {
            Ok(user) => Ok(user),
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

    pub async fn update(&self, pool: Pool<Postgres>, entity: User) -> Result<User, Error> {
        let query = "update users
        set username = $1, password = $2, email = $3
        where user_uuid = $4
        returning *";

        match sqlx::query_as::<_, User>(query)
            .bind(entity.username)
            .bind(entity.password)
            .bind(entity.email)
            .bind(entity.user_uuid)
            .fetch_one(&pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(e)) => match e.code().as_deref() {
                Some("23505") => Err(Error::NotUnique(e.to_string())),
                _ => Err(Error::Other(e.to_string())),
            },
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }

    pub async fn delete(&self, pool: Pool<Postgres>, uuid: uuid::Uuid) -> Result<(), Error> {
        let query = "delete from users where user_uuid = $1";

        match sqlx::query(query).bind(uuid).execute(&pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }
}
