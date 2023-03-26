use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

lazy_static::lazy_static! {
    // static ref PASSWORD_REGEX: Regex = Regex::new(r"^(.{0,7}|[^0-9]*|[^A-Z]*|[^a-z]*|[a-zA-Z0-9]*)$").unwrap();
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
    static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9]{6,}$").unwrap();
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq, Validate)]
pub struct User {
    pub user_uuid: Uuid,
    #[validate(regex(
        path = "RE_USERNAME",
        message = "Username must number and alphabets only and must be 6 characters long"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Validate)]
pub struct CreateUser {
    #[validate(regex(
        path = "RE_USERNAME",
        message = "Username must number and alphabets only and must be 6 characters long"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces. Must be 8 characters long."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords must match"))]
    pub password_confirm: String,
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Validate)]
pub struct UpdateUser {
    #[validate(regex(
        path = "RE_USERNAME",
        message = "Username must number and alphabets only and must be 6 characters long"
    ))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces. Must be 8 characters long."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Validate)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_ascii_digit();
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        Err(ValidationError::new("Password Validation Failed"))
    }
}
