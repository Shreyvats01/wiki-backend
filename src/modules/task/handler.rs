use axum::{
    Extension, Json,
    extract::{Path, State},
};

use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    common::{error::AppError, response::ApiResponse},
    modules::{
        task::{
            model::{CreateLabelDto, UpdateTaskCredentials},
            service::TaskService,
        },
        user::model::UserId,
    },
    state::AppState,
};

pub async fn delete_task_handler(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.task_service.delete(task_id).await?;

    Ok(Json(ApiResponse::success(
        "task deleted successfuly",
        None::<()>,
    )))
}

#[debug_handler]
pub async fn update_task_handler(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(update): Json<UpdateTaskCredentials>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.task_service.update(update, task_id).await?;

    Ok(Json(ApiResponse::success(
        "task fetch successfuly",
        None::<()>,
    )))
}

#[debug_handler]
pub async fn create_tag_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateLabelDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tag = CreateLabelDto::validate(dto)?;

    let service_tag = TaskService::create_tag(&state.task_service, user_id.0, tag).await?;

    Ok(Json(ApiResponse::success(
        "Tag created successfuly",
        service_tag,
    )))
}

pub async fn fetch_all_tags_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tags = TaskService::fetch_all_tags(&state.task_service, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "All tasks fetch successfuly",
        tags,
    )))
}

pub async fn delete_tag_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    TaskService::delete_tag(&state.task_service, slug, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "Tag successfuly deleted",
        None::<()>,
    )))
}

pub async fn create_category_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateLabelDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let new_category_playload = CreateLabelDto::validation(dto)?;

    let category =
        TaskService::create_category(&state.task_service, user_id.0, new_category_playload).await?;

    Ok(Json(ApiResponse::success(
        "Category created successfuly",
        category,
    )))
}

pub async fn fetch_all_categories_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let categories = TaskService::fetch_all_categories(&state.task_service, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "Fetch all categories successfully",
        categories,
    )))
}

pub async fn delete_category_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    TaskService::delete_category(&state.task_service, slug, user_id.0).await?;
    Ok(Json(ApiResponse::success(
        "Category deleted successfuly",
        None::<()>,
    )))
}
