use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::user::model::{SignUpCredentials, User, UserResponseDto},
};
pub struct UserRepo;

impl UserRepo {
    pub async fn create(
        pool: &PgPool,
        user: SignUpCredentials
    ) -> Result<UserResponseDto> {
        let user = sqlx::query_as!(
            UserResponseDto,
            r#"
        INSERT INTO users (name, username, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, username, email, profile_image
        "#,
            &user.name,
            &user.username,
            &user.email,
            &user.password,
            // &user.profile_image
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<UserResponseDto>> {
        let user = sqlx::query_as!(
            UserResponseDto,
            r#"
        SELECT id, name, username, email, profile_image
        FROM users
        WHERE id = $1
        "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            "
        DELETE FROM users
        WHERE id = $1
        ",
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delete user".into()));
        }

        Ok(())
    }

    pub async fn fetch_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, name, username, email, password, profile_image
        FROM users
        WHERE email = $1
        "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn fetch_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
        SELECT id, name, username, email, password, profile_image
        FROM users
        WHERE username = $1
        "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
