use sqlx::{PgPool, Postgres, QueryBuilder, Result};
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::task::model::{Category, CreateCategoryDto, CreateTagDto, Tags, Tagtask, taskCred},
};

pub struct taskRepo;

impl taskRepo {
    // pub async fn insert(pool: &PgPool, user_id: Uuid, new: &Newtask) -> Result<task> {
    //     let task = sqlx::query_as!(
    //         task,
    //         r#"
    //     INSERT INTO tasks (user_id, title, description, category_id)
    //     VALUES ($1, $2, $3, $4)
    //     RETURNING id, user_id, title, description, created_at, category_id, updated_at
    //     "#,
    //         user_id,
    //         new.task,
    //         new.description,
    //         new.category_id
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(task)
    // }

    // pub async fn fetch(pool: &PgPool, task_id: Uuid) -> Result<Option<taskCred>> {
    //     let task = sqlx::query_as!(
    //         taskCred,
    //         r#"
    //     SELECT id, title, description, created_at, category_id, updated_at
    //     FROM tasks
    //     WHERE id = $1
    //     "#,
    //         task_id
    //     )
    //     .fetch_optional(pool)
    //     .await?;

    //     Ok(task)
    // }

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
    ) -> Result<taskCred> {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("UPDATE tasks SET");

        let mut separated = qb.separated(", ");

        if let Some(v) = task {
            separated.push("task = ").push_bind(v);
        }

        if let Some(v) = description {
            separated.push("description = ").push_bind(v);
        }

        if task.is_none() && description.is_none() {
            return Err(sqlx::Error::Protocol("No field to update".into()));
        }

        qb.push(" WHERE id = ").push_bind(task_id);

        qb.push("RETURNING id, task, description, is_done, created_at");

        let updated_task: taskCred = qb.build_query_as().fetch_one(pool).await?;

        Ok(updated_task)
    }

    //tags
    pub async fn create_tag(pool: &PgPool, user_id: Uuid, tag: CreateTagDto) -> Result<Tags> {
        let tag = sqlx::query_as!(
            Tags,
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

    pub async fn fetch_all_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<CreateTagDto>> {
        let tags: Vec<CreateTagDto> = sqlx::query_as!(
            CreateTagDto,
            r#"
            SELECT name, slug 
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
        category: CreateCategoryDto,
    ) -> Result<Category> {
        let category = sqlx::query_as!(
            Category,
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

    pub async fn fetch_all_categories(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<CreateCategoryDto>> {
        let categories = sqlx::query_as!(
            CreateCategoryDto,
            r#"
            SELECT name, slug
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

    pub async fn create_tag_task(pool: &PgPool, task_id: &Uuid, tag_id: &Uuid) -> Result<()> {
        sqlx::query_as!(
            Tagtask,
            r#"
            INSERT INTO tag_task (task_id, tag_id) 
            VALUES ($1, $2)
            "#,
            task_id,
            tag_id
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_all_tag_task(
        pool: &PgPool,
        task_id: Uuid,
    ) -> Result<Vec<Tagtask>, AppError> {
        let all_tag_task = sqlx::query_as!(
            Tagtask,
            r#"
            SELECT tag_id, t.name, t.slug
            FROM tag_task
            JOIN tags t ON t.id = tag_id
            WHERE task_id = $1
            "#,
            task_id
        )
        .fetch_all(pool)
        .await?;

        Ok(all_tag_task)
    }

    pub async fn fetch_tag(pool: &PgPool, slug: &str, user_id: Uuid) -> Result<Option<Tags>> {
        let tag = sqlx::query_as!(
            Tags,
            r#"
            SELECT id, user_id, name, slug 
            From tags
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(tag)
    }

    pub async fn fetch_tag_id(
        pool: &PgPool,
        tag_id: Uuid,
    ) -> Result<Option<CreateTagDto>, AppError> {
        let tag = sqlx::query_as!(
            CreateTagDto,
            r#"
            SELECT name, slug
            FROM tags
            WHERE id = $1
            "#,
            tag_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(tag)
    }

    pub async fn fetch_category_id(
        pool: &PgPool,
        category_id: &Uuid,
    ) -> Result<Option<CreateCategoryDto>, AppError> {
        let category: Option<CreateCategoryDto> = sqlx::query_as!(
            CreateCategoryDto,
            r#"
            SELECT name, slug
            FROM categories
            WHERE id = $1
            "#,
            category_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(category)
    }

    pub async fn fetch_category(
        pool: &PgPool,
        slug: &str,
        user_id: Uuid,
    ) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT id, user_id, name, slug 
            From categories
            WHERE slug = $1 AND user_id = $2
            "#,
            slug,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(category)
    }
}
