
use crate::{
    common::{
        error::{AppError, NotFoundError},
        response::ApiResponse,
    },
    modules::{
        rooms::{
            model::{RoomDto},
            repository::RoomRepo,
            service::RoomService,
        },
        user::{model::UserId},
    },
    state::{AppState},
};
use axum::{
    Extension, Json,
    extract::{
        Path, State
    },
    http::StatusCode
};

use uuid::Uuid;

pub async fn create_room_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(dto): Json<RoomDto>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    let room = RoomDto::validate(dto)?;

    let room = RoomRepo::create_room(&state.pool, room, user_id.0).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Room created successfully", room)),
    ))
}

pub async fn get_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    match RoomRepo::get_room(&state.pool, room_id).await? {
        Some(value) => {
            return Ok((
                StatusCode::OK,
                Json(ApiResponse::success("Successfully fetch room", value)),
            ));
        }
        None => return Err(AppError::NotFound(NotFoundError::RoomNotFound)),
    };
}

pub async fn get_all_rooms_handler(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    let rooms = RoomRepo::get_all_rooms(&state.pool).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Successfully fetch all rooms", rooms)),
    ))
}

pub async fn join_room_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.room_service.join_room(&room_id, &user_id.0).await?;

    Ok(Json(ApiResponse::success(
        "User joined the room successfully",
        None::<()>,
    )))
}

pub async fn leave_room_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.room_service.leave_room(&room_id, &user_id.0).await?;

    Ok(Json(ApiResponse::success("", None::<()>)))
}

pub async fn get_room_membership_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let room = state
        .room_service
        .get_user_join_status(&room_id, &user_id.0)
        .await?;

    Ok(Json(ApiResponse::success(
        "User join ststus fetch successfuly",
        room,
    )))
}

pub async fn get_room_members_handler(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<impl serde::Serialize>>), AppError> {
    let room_members = RoomService::get_room_messages(state.room_service, room_id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("All user fetch successfully", room_members))
    ))
}

pub async fn send_chat_messages(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>
) {

}