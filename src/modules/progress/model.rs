use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub day: Date,

    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, FromRow, Serialize)]
pub struct DailyProgressTask {
    pub id: Uuid,
    pub task_id: Uuid,
    pub daily_progress_id: Uuid,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct DailyTaskDto {
    pub task_id: Uuid,
    pub daily_task_id: Uuid,
    pub title: String,
    pub description: String,
    pub category_id: Option<Uuid>,
    pub tags_id: Vec<Uuid>,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct DailyTaskCTO {
    pub title: String,
    pub description: String,
    pub category_id: Uuid,
    pub tags: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ProgressTaskRespons {
    pub daily_task_id: Uuid,
    pub task_id: Uuid,
    pub daily_progress_id: Uuid,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
    pub created_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CompleteDailyTask {
    pub daily_progress_task_id: Uuid,
    pub task_id: Uuid,
    pub task_title: String,
    pub task_description: String,
    pub is_done: bool,
    pub created_at: PrimitiveDateTime,
    pub category_slug: String,
    pub category_name: String,
    pub tag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct DailyProgressDto {
    pub day: String,
}

// #[derive(Debug, Deserialize)]
// pub struct DailyProgresstaskDto {
//     pub task_id: Uuid,
//     pub is_done: bool
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct IsExitsResponse {
    pub id: Option<Uuid>,
    pub is_exits: bool,
}
