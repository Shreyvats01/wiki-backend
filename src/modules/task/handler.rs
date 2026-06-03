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
            model::{
                CreateCategoryDto, CreateTagDto, CreatetaskDto, Newtask, UpdatetaskCredentials,
                taskResponse,
            },
            service::taskService,
        },
        user::model::UserId,
    },
    state::AppState,
};

// #[debug_handler]
// pub async fn create_task_handler(
//     State(state): State<AppState>,
//     Extension(user_id): Extension<UserId>,
//     Json(dto): Json<CreatetaskDto>,
// ) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
//     let new_task: Newtask = dto.try_into()?;
//     let mut tags: Vec<CreateTagDto> = Vec::new();

//     let task = state.task_service.create_task(user_id.0, &new_task).await?;
//     // let daily_progress_task

//     for i in new_task.tags {
//         let tag = state.task_service.fetch_tag_slug(user_id.0, &i).await?;

//         state.task_service.create_tag_task(&task.id, &tag.id).await?;

//         let create_dto = CreateTagDto {
//             name: tag.name,
//             slug: tag.slug
//         };

//         tags.push(create_dto);
//     }

//     let category = state.task_service.fetch_category(&task.category_id).await?;

//     let task_response = taskResponse {
//         id: task.id,
//         title: task.title,
//         description: task.description,
//         category: category,
//         tags: tags,
//         created_at: task.created_at,
//         updated_at: task.updated_at
//     };

//     Ok(Json(ApiResponse::success(
//         "User created Successfully",
//         task_response,
//     )))
// }

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

// pub async fn get_task_handler(
//     State(state): State<AppState>,
//     Path(task_id): Path<Uuid>,
// ) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
//     let task = state.task_service.get(task_id).await?;

//     Ok(Json(ApiResponse::success("task fetch successfuly", task)))
// }

#[debug_handler]
pub async fn update_task_handler(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(update): Json<UpdatetaskCredentials>,
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
    Json(dto): Json<CreateTagDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tag = CreateTagDto::validate(dto)?;

    let service_tag = taskService::create_tag(&state.task_service, user_id.0, tag).await?;

    Ok(Json(ApiResponse::success(
        "Tag created successfuly",
        service_tag,
    )))
}

pub async fn fetch_all_tags_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let tags = taskService::fetch_all_tags(&state.task_service, user_id.0).await?;

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
    taskService::delete_tag(&state.task_service, slug, user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "Tag successfuly deleted",
        None::<()>,
    )))
}

pub async fn create_category_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<CreateCategoryDto>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let new_category_playload = CreateCategoryDto::validation(dto)?;

    let category =
        taskService::create_category(&state.task_service, user_id.0, new_category_playload).await?;

    Ok(Json(ApiResponse::success(
        "Category created successfuly",
        category,
    )))
}

pub async fn fetch_all_categories_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let categories = taskService::fetch_all_categories(&state.task_service, user_id.0).await?;

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
    taskService::delete_category(&state.task_service, slug, user_id.0).await?;
    Ok(Json(ApiResponse::success(
        "Category deleted successfuly",
        None::<()>,
    )))
}
