use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError, ValidationError},
    modules::user::{
        model::{LoginCredentials, SignUpCredentials, User, UserResponseDto},
        repository::UserRepo,
    },
};

#[derive(Debug, Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: SignUpCredentials) -> Result<UserResponseDto, AppError> {
        let option_user = UserRepo::fetch_by_email(&self.pool, &user.email).await?;

        if option_user.is_some() {
            return Err(AppError::Validation(ValidationError::UserAlreadyExits));
        }

        let created_user = UserRepo::create(
            &self.pool,
            &user.name,
            &user.email,
            &user.password,
            &user.username,
        )
        .await?;

        Ok(created_user)
    }

    pub async fn login(&self, user: LoginCredentials) -> Result<User, AppError> {
        let db_user = UserRepo::fetch_by_email(&self.pool, &user.email)
            .await?
            .ok_or_else(|| AppError::NotFound(NotFoundError::UserNotFound))?;

        if user.password != db_user.password {
            return Err(AppError::Validation(ValidationError::InvalidPassword));
        }

        Ok(db_user)
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<(), AppError> {
        UserRepo::delete(&self.pool, user_id).await?;

        Ok(())
    }

    pub async fn get(&self, user_id: Uuid) -> Result<UserResponseDto, AppError> {
        let user = UserRepo::fetch_by_id(&self.pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(NotFoundError::UserNotFound))?;

        Ok(user)
    }

    pub async fn change_visibility(&self, user_id: Uuid, is_public: bool) -> Result<(), AppError> {
        UserRepo::change_visibility(&self.pool, &user_id, is_public).await?;
        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, AppError> {
        let user = UserRepo::fetch_by_username(&self.pool, username)
            .await?
            .ok_or_else(|| AppError::NotFound(NotFoundError::UserNotFound))?;

        Ok(user)
    }
}
