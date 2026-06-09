use sqlx::{PgPool, Postgres, QueryBuilder, Result};
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::task::model::{CreateLabelDto, Label, LabelResponse, TaskCred},
};

pub struct TaskRepo;

impl TaskRepo {
    pub async fn delete(pool: &PgPool, task_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delect task".into()));
        }

        Ok(())
    }

    pub async fn update(
        pool: &PgPool,
        task_id: Uuid,
        task: Option<&str>,
        description: Option<&str>,
    ) -> Result<TaskCred> {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("UPDATE tasks SET ");

        let mut separated = qb.separated(", ");

        if let Some(v) = task {
            separated.push("title = ").push_bind(v);
        }

        if let Some(v) = description {
            separated.push("description = ").push_bind(v);
        }

        if task.is_none() && description.is_none() {
            return Err(sqlx::Error::Protocol("No field to update".into()));
        }

        separated.push("updated_at = now()");

        qb.push(" WHERE id = ").push_bind(task_id);

        qb.push(" RETURNING id, title, description, category_id, created_at, updated_at");

        let updated_task: TaskCred = qb.build_query_as().fetch_one(pool).await?;

        Ok(updated_task)
    }

    pub async fn create_tag(pool: &PgPool, user_id: Uuid, tag: CreateLabelDto) -> Result<Label> {
        let tag = sqlx::query_as!(
            Label,
            r#"
            INSERT INTO tags (user_id, name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, slug 
            "#,
            user_id,
            tag.name,
            tag.slug
        )
        .fetch_one(pool)
        .await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<LabelResponse>> {
        let tags: Vec<LabelResponse> = sqlx::query_as!(
            LabelResponse,
            r#"
            SELECT id, name, slug
            FROM tags
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(tags)
    }

    pub async fn delete_tag(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            "DELETE FROM tags WHERE slug = $1 AND user_id = $2",
            slug,
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delete tag".into()));
        }

        Ok(())
    }

    pub async fn create_categories(
        pool: &PgPool,
        user_id: Uuid,
        category: CreateLabelDto,
    ) -> Result<Label> {
        let category = sqlx::query_as!(
            Label,
            r#"
            INSERT INTO categories (user_id, name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, slug
            "#,
            user_id,
            category.name,
            category.slug
        )
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn fetch_all_categories(pool: &PgPool, user_id: Uuid) -> Result<Vec<LabelResponse>> {
        let categories = sqlx::query_as!(
            LabelResponse,
            r#"
            SELECT id, name, slug
            FROM categories
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn delete_categories(
        pool: &PgPool,
        slug: &str,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            "DELETE FROM categories WHERE slug = $1 AND user_id = $2",
            slug,
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delete a category".into()));
        }

        Ok(())
    }
}
