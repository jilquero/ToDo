use common::model::user::{CreateUser, LoginUser, UpdateUser};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{domain::error::Error, infrastructure::utils::verify};

use super::{repository::Repository, user::User};

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
    pub async fn auth_user(&self, LoginUser { email, password }: LoginUser) -> Result<User, Error> {
        let user = self
            .repo
            .find_by_email(self.pool.clone(), email.clone())
            .await?;

        if verify(&user.password, password.as_str()).unwrap() {
            Ok(user)
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn find(&self) -> Result<Vec<User>, Error> {
        self.repo.find(self.pool.clone()).await
    }

    pub async fn find_by_email(&self, email: String) -> Result<User, Error> {
        self.repo
            .find_by_email(self.pool.clone(), email.clone())
            .await
    }

    pub async fn find_by_uuid(&self, uuid: uuid::Uuid) -> Result<User, Error> {
        self.repo.find_by_uuid(self.pool.clone(), uuid).await
    }

    pub async fn save(&self, create_user: CreateUser) -> Result<User, Error> {
        let entity = User::new(create_user);
        self.repo.save(self.pool.clone(), entity).await
    }

    pub async fn update(&self, uuid: Uuid, update_user: UpdateUser) -> Result<User, Error> {
        let mut user = self.find_by_uuid(uuid).await?;
        user.update(update_user);

        self.repo.update(self.pool.clone(), user).await
    }

    pub async fn delete(&self, uuid: uuid::Uuid) -> Result<(), Error> {
        self.repo.delete(self.pool.clone(), uuid).await
    }
}
