use sqlx::PgPool;
use time::Date;
use uuid::Uuid;

use crate::{
    common::error::{AppError},
    modules::progress::{
            model::{CompleteDailyProgressTodo, DailyProgress, DailyProgressTodo, DailyProgressTodoDto, DailyProgressTodoResponse, ProgressTodoRespons},
            repository::ProgressRepo,
        },
};

#[derive(Debug, Clone)]
pub struct ProgressService {
    pub pool: PgPool,
}

impl ProgressService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool }
    }

    pub async fn create_daily_progress(
        &self,
        user_id: &Uuid,
        day: Date,
    ) -> Result<DailyProgress, AppError> {
        let daily_progress = ProgressRepo::create_daily_progress(&self.pool, user_id, day)
            .await?;

        Ok(daily_progress)
    }

    pub async fn create_daily_progress_todo(
        &self,
        progress_id: &Uuid,
        user_id: &Uuid,
        dto: DailyProgressTodoResponse
    ) -> Result<DailyProgressTodoDto, AppError> {
        let progress_todo =
            ProgressRepo::create_daily_progress_todo(&self.pool, progress_id, user_id, dto)
                .await?;

        Ok(progress_todo)
    }

    pub async fn toggle_daily_progress_todo(&self, progress_todo_id: &Uuid, user_id: &Uuid) -> Result<DailyProgressTodo, AppError>{
        let todo = ProgressRepo::toggle_daily_progress_todo(&self.pool, progress_todo_id, user_id).await?;

        Ok(todo)
    }

    pub async fn fetch_all_daily_progress_todo(&self, daily_progress_id: &Uuid) -> Result<Vec<CompleteDailyProgressTodo>, AppError> {
        let progress_todos= ProgressRepo::fetch_all_daily_progress_todos(&self.pool, daily_progress_id).await?;
        Ok(progress_todos)
    }

    pub async fn fetch_daily_progress_todo_id(&self, progress_todo_id: &Uuid)-> Result<ProgressTodoRespons, AppError> {
        let task: ProgressTodoRespons = ProgressRepo::fetch_daily_progress_todo_by_id(&self.pool, progress_todo_id).await?;

        Ok(task)
    }

    pub async fn fetch_progress_id(&self, user_id: &Uuid, day: Date) -> Result<Option<Uuid>, AppError> {
        let progress = ProgressRepo::get_progress_id(&self.pool, user_id, day).await?;

        Ok(progress)
    }

    pub async fn delete_daily_progress_todo(&self, id: &Uuid) -> Result<(), AppError> {
        ProgressRepo::delete_daily_progress_todo(&self.pool, id).await?;

        Ok(())
    }
}

