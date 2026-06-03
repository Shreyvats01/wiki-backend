use sqlx::{PgPool, Result};
use time::Date;
use uuid::Uuid;

use crate::{
    common::error::{AppError, NotFoundError},
    modules::{
        progress::model::{
            CompleteDailyProgressTask, DailyProgress, DailyProgressTask, DailyProgressTaskDto,
            DailyProgressTaskResponse, ProgressTaskRespons,
        },
        task::model::task,
    },
};

pub struct ProgressRepo;

impl ProgressRepo {
    pub async fn create_daily_progress(
        pool: &PgPool,
        user_id: &Uuid,
        day: Date,
    ) -> Result<DailyProgress> {
        let progress: DailyProgress = sqlx::query_as!(
            DailyProgress,
            r#"
            INSERT INTO daily_progress (user_id, day)
            VALUES ($1, $2)
            RETURNING id, user_id, day, created_at, updated_at
            "#,
            user_id,
            day
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn fetch_daily_progress_by_user_id_and_day(
        pool: &PgPool,
        day: &Date,
        user_id: &Uuid,
    ) -> Result<DailyProgress> {
        let progress = sqlx::query_as!(
            DailyProgress,
            r#"
            SELECT id, user_id, day, created_at, updated_at
            FROM daily_progress
            WHERE user_id = $1 AND day = $2
            "#,
            user_id,
            day
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn fetch_daily_progress_by_id(pool: &PgPool, id: &Uuid) -> Result<DailyProgress> {
        let progress = sqlx::query_as!(
            DailyProgress,
            r#"
            SELECT id, user_id, day, created_at, updated_at
            FROM daily_progress 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(progress)
    }

    pub async fn create_daily_progress_task(
        pool: &PgPool,
        daily_progress_id: &Uuid,
        user_id: &Uuid,
        new_task: DailyProgressTaskResponse,
    ) -> Result<DailyProgressTaskDto, AppError> {
        let mut tx = pool.begin().await?;

        let tasks = sqlx::query_as!(
            task,
            r#"
            INSERT INTO tasks (user_id, title, description, category_id)
            VALUES ($1, $2, $3, 
        (
            SELECT id
            FROM categories
            WHERE slug = $4 AND user_id =$1
            LIMIT 1
        )
            )
            RETURNING id, user_id, title, description, created_at, updated_at, category_id
            "#,
            user_id,
            new_task.title,
            new_task.description,
            new_task.category_slug
        )
        .fetch_one(&mut *tx)
        .await?;

        let exits = sqlx::query_scalar!(
            r#"
            SELECT 1
            FROM daily_progress
            WHERE id = $1 AND user_id = $2
            "#,
            daily_progress_id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await?;

        if exits.is_none() {
            return Err(AppError::NotFound(NotFoundError::DailyProgressNotFound));
        }

        let daily_progress_task = sqlx::query_as!(
            DailyProgressTask,
            r#"
            INSERT INTO daily_tasks (task_id, daily_progress_id, is_done)
            VALUES ($1, $2, false)
            RETURNING id, task_id, daily_progress_id, is_done, created_at 
            "#,
            tasks.id,
            daily_progress_id
        )
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

        let return_value: DailyProgressTaskDto = DailyProgressTaskDto {
            id: tasks.id,
            title: tasks.title,
            description: tasks.description,
            category_id: tasks.category_id,
            is_done: daily_progress_task.is_done,
            created_at: daily_progress_task.created_at,
        };

        Ok(return_value)
    }

    pub async fn fetch_daily_progress_task_by_id(
        pool: &PgPool,
        id: &Uuid,
    ) -> Result<ProgressTaskRespons> {
        let task = sqlx::query_as!(
            ProgressTaskRespons,
            r#"
            SELECT pt.id AS progress_task_id, pt.task_id, pt.daily_progress_id, pt.is_done, pt.created_at, t.title, t.description
            FROM daily_tasks pt
            JOIN tasks t ON pt.task_id = t.id
            WHERE pt.id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn toggle_daily_progress_task(
        pool: &PgPool,
        id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DailyProgressTask> {
        let task: DailyProgressTask = sqlx::query_as!(
            DailyProgressTask,
            r#"
            UPDATE daily_tasks dpt
            SET is_done = NOT dpt.is_done
            FROM daily_progress dp
            WHERE dpt.id = $1
            AND dpt.daily_progress_id = dp.id
            AND dp.user_id = $2
            RETURNING dpt.id, dpt.task_id, dpt.daily_progress_id, dpt.is_done, dpt.created_at
            "#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn fetch_all_daily_progress_tasks(
        pool: &PgPool,
        daily_progress_id: &Uuid,
    ) -> Result<Vec<CompleteDailyProgressTask>> {
        let tasks = sqlx::query_as!(
            CompleteDailyProgressTask,
            r#"
            SELECT
            t.id AS daily_progress_task_id,
            t.is_done,
            t.created_at,
            td.id as task_id,
            td.title AS task_title,
            td.description AS task_description,
            c.slug AS category_slug,
            c.name AS category_name

  
        FROM daily_tasks t
        JOIN tasks td ON td.id = t.task_id
        JOIN categories c ON c.id = td.category_id
        WHERE t.daily_progress_id = $1
        ORDER BY t.created_at DESC
        "#,
            daily_progress_id
        )
        .fetch_all(pool)
        .await?;

    println!("all daily_progress tasks: {:?}", tasks);

        Ok(tasks)
    }

    pub async fn get_progress_id(
        pool: &PgPool,
        user_id: &Uuid,
        day: Date,
    ) -> Result<Option<Uuid>, AppError> {
        let progress_id = sqlx::query_scalar!(
            r#"
        SELECT id
        FROM daily_progress
        WHERE user_id = $1 AND day = $2
        "#,
            user_id,
            day
        )
        .fetch_optional(pool)
        .await?;

        Ok(progress_id)
    }

    pub async fn delete_daily_progress_task(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            WITH deleted_dpt AS (
                DELETE FROM daily_tasks
                WHERE id = $1
                RETURNING task_id
            )
            DELETE FROM tasks
            WHERE id = (SELECT task_id FROM deleted_dpt)
            "#,
            id
        ).execute(pool).await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Failed("Failed to delete task".into()));
        }

        Ok(())
    }
}
