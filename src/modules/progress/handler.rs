use axum::{Extension, Json, extract::{Path, State}};
use axum_macros::debug_handler;
use uuid::Uuid;
use time::{Date, format_description::well_known::Iso8601};

use crate::{
    common::{error::AppError, response::ApiResponse},
    modules::{progress::{model::{DailyProgressDto, DailyProgressTaskResponse, IsExitsResponse}, service::ProgressService}, user::model::UserId},
    state::AppState,
};

#[debug_handler]
pub async fn create_daily_progress_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<DailyProgressDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {

    
    let parsed = Date::parse(&dto.day, &Iso8601::DATE)
    .map_err(|_| AppError::Failed("Failed to convert into Date".into()))?;
    
    let daily_progress = state
        .progress_service
        .create_daily_progress(&user_id.0, parsed)
        .await?;

    Ok(Json(ApiResponse::success(
        "Today, canvas successfuly created",
        daily_progress,
    )))
}
#[debug_handler]
pub async fn create_daily_progress_task_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(daily_progress_id): Path<Uuid>,
    Json(dto): Json<DailyProgressTaskResponse>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    println!("data inside field: {:?}", dto);
    let daily_progress_task = state.progress_service.create_daily_progress_task(&daily_progress_id, &user_id.0, dto).await?;

    Ok(Json(ApiResponse::success("Successfuly created progress task", daily_progress_task)))
}

pub async fn fetch_daily_progress_task_by_id(
    State(state): State<AppState>,
    Path(progress_task_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_task = ProgressService::fetch_daily_progress_task_id(&state.progress_service, &progress_task_id).await?;

    Ok(Json(ApiResponse::success("task updated successfuly", daily_progress_task)))

}
pub async fn toggle_daily_progress_task_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(progress_task_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let daily_progress_task = ProgressService::toggle_daily_progress_task(&state.progress_service, &progress_task_id, &user_id.0).await?;


    Ok(Json(ApiResponse::success("Toggle task successfuly", daily_progress_task)))
}
pub async fn fetch_all_daily_progress_tasks(
    State(state): State<AppState>,
    Path(daily_progress_id): Path<Uuid>
)-> Result<Json<ApiResponse<impl serde::Serialize>>, AppError>  {
    let tasks = ProgressService::fetch_all_daily_progress_task(&state.progress_service, &daily_progress_id).await?;

    Ok(Json(ApiResponse::success("fetched all successfuly", tasks)))
}

pub async fn is_progress_exits_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(day): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let day = Date::parse(&day, &Iso8601::DATE)
    .map_err(|_| AppError::Failed("Invalid date. Use YYYY-MM-DD".into()))?;

    if let Some(id) = ProgressService::fetch_progress_id(&state.progress_service, &user_id.0, day).await? {
        return Ok(Json(ApiResponse::success("Progress exits!", IsExitsResponse {id: Some(id), is_exits: true})));
    } else {
        return Ok(Json(ApiResponse::success("Progress doesn't exits!", IsExitsResponse {id: None, is_exits: false})));
    };

}

pub async fn delete_daily_progress_task_handler(
    State(state): State<AppState>,
    Path(progress_task_id): Path<Uuid>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    ProgressService::delete_daily_progress_task(&state.progress_service, &progress_task_id).await?;

    Ok(Json(ApiResponse::success("Successfully deleted daily progress task", None::<()>)))
}