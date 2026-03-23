use std::collections::HashMap;

use crate::{
    common::{
        error::{AppError, NotFoundError},
        response::ApiResponse,
    },
    modules::{
        rooms::{
            model::{ClientEvent, MessageDto, PresenceKind, RoomDto, ServerEvent},
            repository::RoomRepo,
            service::RoomService,
        },
        user::{model::UserId, repository::UserRepo},
    },
    state::{AppState, RoomState},
};
use axum::{
    Extension, Json,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;

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

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let can_join = state
        .room_service
        .clone()
        .get_user_join_status(&room_id, &user_id.0)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if !can_join {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(ws.on_upgrade(move |socket| handler_socket(socket, state, room_id, user_id.0)))
}

pub async fn handler_socket(socket: WebSocket, state: AppState, room_id: Uuid, user_id: Uuid) {
    let mut rooms = state.rooms.lock().await;

    let room_state: &mut RoomState = rooms.entry(room_id).or_insert_with(|| RoomState {
        members: HashMap::new(),
    });

    drop(rooms);

    let username = match UserRepo::fetch_by_id(&state.pool, user_id).await {
        Ok(Some(user)) => user.username,
        Ok(None) => {
            eprintln!("User not found for id: {user_id}");
            return;
        }
        Err(err) => {
            eprintln!("Failed to fetch user {user_id}: {err}");
            return;
        }
    };

    let (mut out_tx, mut out_rx) = mpsc::channel::<ServerEvent>(100);

    // Join Notification
    RoomService::register_member(&state, room_id, user_id, username.clone(), out_tx).await;
    RoomService::broadcast_presence(&state, &room_id, username.clone(), PresenceKind::Join).await;

    let (mut sender, mut receiver) = socket.split();

    // send history
    if let Ok(history) = RoomRepo::load_recent_messages(&state.pool, room_id).await {
        if let Ok(send) = serde_json::to_string(&ServerEvent::History(history)) {
            let _ = sender.send(Message::Text(send.into())).await;
        } else {
            eprintln!("Failed to convert history to string for room: {room_id}");
        }
    }

    // fan-out task
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            match serde_json::to_string(&msg) {
                Err(_) => {
                    break;
                }
                Ok(v) => {
                    if sender.send(Message::Text(v.into())).await.is_err() {
                        break;
                    }
                }
            };
        }
    });

    // receive task
    let pool: sqlx::Pool<sqlx::Postgres> = state.pool.clone();
    let state_clone = state.clone();
    let username_clone = username.clone();

    let mut recv_task = tokio::spawn(async move {
        let mut receiver = receiver;
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(evt) = serde_json::from_str::<ClientEvent>(&text) {
                match evt {
                    ClientEvent::ChatSend { content, parent_id } => {
                        if let Ok(dto) = MessageDto::validate(MessageDto { room_id, content, parent_id }) {
                            if let Ok(saved) = RoomRepo::create_message(&pool, dto, &user_id, parent_id).await
                            {
                                RoomService::broadcast_message(
                                    &state_clone,
                                    &room_id,
                                    ServerEvent::ChatMessage(saved),
                                )
                                .await;
                            }
                        }
                    }
                    ClientEvent::Ping => {
                        RoomService::broadcast_message(&state_clone, &room_id, ServerEvent::Pong)
                            .await;
                    }
                    ClientEvent::Typing { is_typing } => {
                        RoomService::broadcast_message(
                            &state_clone,
                            &room_id,
                            ServerEvent::Typing {
                                username: username_clone.clone(),
                                is_typing,
                            },
                        )
                        .await;
                    },
                    ClientEvent::ActiveMembers => {
                        if let Ok(members) = RoomService::get_active_members(&state_clone, &room_id).await {
                            RoomService::return_message(&state_clone, &room_id, &user_id, ServerEvent::ActiveMembers(members)).await;
                        }
                    },
                    ClientEvent::AllMembers => {
                       let memebers = RoomRepo::get_room_members(&pool, &room_id).await;
                       if let Ok(mem) = memebers {
                        RoomService::return_message(&state_clone, &room_id, &user_id, ServerEvent::AllMembers(mem)).await;
                       } 
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort()
        }
        _ = &mut recv_task => {
            send_task.abort()
        }
    }

    RoomService::unregister_member(&state, room_id.clone(), &user_id).await;
    RoomService::broadcast_message(
        &state,
        &room_id,
        ServerEvent::Presence {
            user: username.clone(),
            kind: PresenceKind::Leave,
        },
    )
    .await;

    if let Ok(members) = RoomService::get_active_members(&state, &room_id).await {
        RoomService::broadcast_message(&state, &room_id, ServerEvent::ActiveMembers(members)).await;
    };

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

pub async fn get_room_members() {}