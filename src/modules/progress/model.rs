use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use time::{Date, PrimitiveDateTime};


#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub day: Date,

    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime
}

#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgressTask {
    pub id: Uuid,
    pub task_id: Uuid,
    pub daily_progress_id: Uuid,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(Debug, Serialize)]
pub struct DailyProgressTaskDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Uuid,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(Debug, Deserialize)]
pub struct DailyProgressTaskResponse {
    pub title: String,
    pub description: String,
    pub category_slug: String,
}

#[derive(Debug, Serialize)]
pub struct ProgressTaskRespons {
    pub progress_task_id: Uuid,
    pub task_id: Uuid,
    pub daily_progress_id: Uuid,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CompleteDailyProgressTask {
    pub daily_progress_task_id: Uuid,
    pub task_id: Uuid,
    pub task_title: String,
    pub task_description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
    pub category_slug: String,
    pub category_name: String
}
// pub struct

#[derive(Debug, Deserialize)]
pub struct DailyProgressDto {
    pub day: String
}

// #[derive(Debug, Deserialize)]
// pub struct DailyProgresstaskDto {
//     pub task_id: Uuid,
//     pub is_done: bool
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct IsExitsResponse {
    pub id: Option<Uuid>,
    pub is_exits: bool
}