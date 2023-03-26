use common::model::user::{CreateUser, UpdateUser};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::utils::hash_password;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(
        CreateUser {
            username,
            email,
            password,
            password_confirm: _,
        }: CreateUser,
    ) -> Self {
        let hash = hash_password(password.as_str()).unwrap();
        User {
            user_uuid: Uuid::new_v4(),
            username,
            email,
            password: hash,
        }
    }

    pub fn update(
        &mut self,
        UpdateUser {
            username,
            email,
            password,
        }: UpdateUser,
    ) {
        self.username = username.unwrap_or_else(|| self.username.clone());
        self.email = email.unwrap_or_else(|| self.email.clone());
        self.password = match password {
            Some(password) => hash_password(password.as_str()).unwrap(),
            None => self.password.clone(),
        };
    }
}
