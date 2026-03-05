use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{common::error::ValidationError};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_public: bool
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub is_public: bool
}

#[derive(Serialize, Deserialize)]
pub struct UpdateVisibility {
    pub is_public: bool
}

pub struct SignUpCredentials {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginCredentials {
    pub email: String,
    pub password: String
}

#[derive(Clone, Copy, Debug)]
pub struct UserId(pub Uuid);


#[derive(Debug, Deserialize, Serialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpDto {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<SignUpDto> for SignUpCredentials {
    type Error = ValidationError;
    fn try_from(value: SignUpDto) -> Result<Self, Self::Error> {
        let email = value.email.trim();
        let name = value.name.trim();
        let password = value.password.trim();
        let username = value.username.trim();

        if !email.contains("@") || email.len() < 5 {
            return Err(ValidationError::InvalidEmail);
        };

        if name.len() < 3 {
            return Err(ValidationError::TooShortName);
        };

        if password.len() < 5 {
            return Err(ValidationError::InvalidPassword)
        };

        Ok(Self {
            email: email.to_string(),
            name: name.to_string(),
            password: password.to_string(),
            username: username.to_string()
        })
        
    }
}

impl TryFrom<LoginDto> for LoginCredentials {
    type Error = ValidationError;
    fn try_from(value: LoginDto) -> Result<Self, Self::Error> {
        let email = value.email.trim();
        let password = value.password.trim();

        if !email.contains("@") || email.len() < 5 {
            return Err(ValidationError::InvalidEmail);
        };
        if password.len() < 6 {
            return Err(ValidationError::InvalidPassword);
        };

        Ok(Self {
            email: email.to_string(),
            password: password.to_string()
        })
    }
}