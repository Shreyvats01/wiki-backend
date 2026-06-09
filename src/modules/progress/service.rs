use sqlx::PgPool;
use time::Date;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::progress::{
        model::{
            CompleteDailyTask, DailyProgress, DailyProgressTask, DailyTaskCTO, DailyTaskDto,
            ProgressTaskRespons,
        },
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
        let daily_progress = ProgressRepo::create_daily_progress(&self.pool, user_id, day).await?;

        Ok(daily_progress)
    }

    pub async fn create_daily_task(
        &self,
        progress_id: &Uuid,
        user_id: &Uuid,
        dto: DailyTaskCTO,
    ) -> Result<DailyTaskDto, AppError> {
        let progress_task =
            ProgressRepo::create_daily_tasks(&self.pool, progress_id, user_id, dto).await?;

        Ok(progress_task)
    }

    pub async fn toggle_daily_task(
        &self,
        progress_task_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DailyProgressTask, AppError> {
        let task = ProgressRepo::toggle_daily_task(&self.pool, progress_task_id, user_id).await?;

        Ok(task)
    }

    pub async fn fetch_all_daily_progress_task(
        &self,
        daily_progress_id: &Uuid,
    ) -> Result<Vec<CompleteDailyTask>, AppError> {
        let progress_tasks =
            ProgressRepo::fetch_all_daily_tasks(&self.pool, daily_progress_id).await?;
        Ok(progress_tasks)
    }

    pub async fn fetch_daily_progress_task_id(
        &self,
        progress_task_id: &Uuid,
    ) -> Result<ProgressTaskRespons, AppError> {
        let task: ProgressTaskRespons =
            ProgressRepo::fetch_daily_task_by_id(&self.pool, progress_task_id).await?;

        Ok(task)
    }

    pub async fn fetch_progress_id(
        &self,
        user_id: &Uuid,
        day: Date,
    ) -> Result<Option<Uuid>, AppError> {
        let progress = ProgressRepo::get_progress_id(&self.pool, user_id, day).await?;

        Ok(progress)
    }

    pub async fn delete_daily_progress_task(&self, id: &Uuid) -> Result<(), AppError> {
        ProgressRepo::delete_daily_task(&self.pool, id).await?;

        Ok(())
    }
}
