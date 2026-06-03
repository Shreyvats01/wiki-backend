use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::task::{
        model::{
            Category, CreateCategoryDto, CreateTagDto, Newtask, TagDtoWithId, Tagtask, Tags, task, taskResponse, UpdatetaskCredentials
        },
        repository::taskRepo,
    },
};

#[derive(Debug, Clone)]
pub struct taskService {
    pub pool: PgPool,
}

impl taskService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn update(
        &self,
        update: UpdatetaskCredentials,
        task_id: Uuid,
    ) -> Result<(), AppError> {
        taskRepo::update(
            &self.pool,
            task_id,
            update.task.as_deref(),
            update.description.as_deref(),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(&self, task_id: Uuid) -> Result<(), AppError> {
        taskRepo::delete(&self.pool, task_id).await?;

        Ok(())
    }

    pub async fn create_tag(&self, user_id: Uuid, dto: CreateTagDto) -> Result<Tags, AppError> {
        let tag = taskRepo::create_tag(&self.pool, user_id, dto).await?;

        Ok(tag)
    }

    pub async fn fetch_all_tags(&self, user_id: Uuid) -> Result<Vec<CreateTagDto>, AppError> {
        let tags = taskRepo::fetch_all_tags(&self.pool, user_id).await?;

        Ok(tags)
    }

    pub async fn delete_tag(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        taskRepo::delete_tag(&self.pool, &slug, user_id).await?;

        Ok(())
    }

    pub async fn create_category(
        &self,
        user_id: Uuid,
        dto: CreateCategoryDto,
    ) -> Result<Category, AppError> {
        let category = CreateCategoryDto::validation(dto)?;

        let tag = taskRepo::create_categories(&self.pool, user_id, category).await?;

        Ok(tag)
    }

    pub async fn fetch_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CreateCategoryDto>, AppError> {
        let categories = taskRepo::fetch_all_categories(&self.pool, user_id).await?;

        Ok(categories)
    }

    pub async fn delete_category(&self, slug: String, user_id: Uuid) -> Result<(), AppError> {
        taskRepo::delete_categories(&self.pool, &slug, user_id).await?;
        Ok(())
    }

    pub async fn fetch_all_task_tags(&self, task_id: Uuid) -> Result<Vec<Tagtask>, AppError> {
        let all: Vec<Tagtask> = taskRepo::fetch_all_tag_task(&self.pool, task_id).await?;
        Ok(all)
    }
    
    pub async fn create_tag_task(&self, task_id: &Uuid, tag_id: &Uuid) -> Result<(), AppError> {
        taskRepo::create_tag_task(&self.pool, task_id, tag_id).await?;
        Ok(())
    }
}
