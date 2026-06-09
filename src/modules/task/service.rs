use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::task::{
        model::{CreateLabelDto, Label, LabelResponse, UpdateTaskCredentials},
        repository::TaskRepo,
    },
};

#[derive(Debug, Clone)]
pub struct TaskService {
    pub pool: PgPool,
}

impl TaskService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn update(
        &self,
        update: UpdateTaskCredentials,
        task_id: Uuid,
    ) -> Result<(), AppError> {
        TaskRepo::update(
            &self.pool,
            task_id,
            update.task.as_deref(),
            update.description.as_deref(),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(&self, task_id: Uuid) -> Result<(), AppError> {
        TaskRepo::delete(&self.pool, task_id).await?;

        Ok(())
    }

    pub async fn create_tag(&self, user_id: Uuid, dto: CreateLabelDto) -> Result<Label, AppError> {
        let tag = TaskRepo::create_tag(&self.pool, user_id, dto).await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(&self, user_id: Uuid) -> Result<Vec<LabelResponse>, AppError> {
        let tags = TaskRepo::fetch_all_tags(&self.pool, user_id).await?;

        Ok(tags)
    }

    pub async fn delete_tag(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TaskRepo::delete_tag(&self.pool, &slug, user_id).await?;

        Ok(())
    }

    pub async fn create_category(
        &self,
        user_id: Uuid,
        dto: CreateLabelDto,
    ) -> Result<Label, AppError> {
        let category = CreateLabelDto::validation(dto)?;

        let tag = TaskRepo::create_categories(&self.pool, user_id, category).await?;

        Ok(tag)
    }

    pub async fn fetch_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<LabelResponse>, AppError> {
        let categories = TaskRepo::fetch_all_categories(&self.pool, user_id).await?;

        Ok(categories)
    }

    pub async fn delete_category(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        TaskRepo::delete_categories(&self.pool, &slug, user_id).await?;
        Ok(())
    }
}
