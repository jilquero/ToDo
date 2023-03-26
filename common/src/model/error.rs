use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    WrongCredentials(String),
    Unauthorized(String),
    NotFound(String),
    Forbidden(String),
    InternalServerError(String),
    UnprocessableEntity(String),
}
