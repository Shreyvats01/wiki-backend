use std::collections::HashMap;

use sqlx::PgPool;
use tokio::sync::mpsc;


use uuid::Uuid;

use crate::{
    common::error::AppError,
    modules::{rooms::{
        model::{Members, MessageDto, MessageResponse, PresenceKind, ServerEvent},
        repository::RoomRepo,
    }, user::repository::UserRepo},
    state::{AppState, Member, RoomState},
};

pub type Username = String;

#[derive(Debug, Clone)]
pub struct RoomService {
    pub pool: PgPool,
}

impl RoomService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(
        self,
        message_dto: MessageDto,
        user_id: &Uuid,
        parent_id: Option<Uuid>
    ) -> Result<MessageResponse, AppError> {
        let message = RoomRepo::create_message(&self.pool, message_dto, user_id, parent_id).await?;

        Ok(message)
    }

    pub async fn get_room_messages(self, room_id: Uuid) -> Result<Vec<MessageResponse>, AppError> {
        let message = RoomRepo::load_recent_messages(&self.pool, room_id).await?;
        Ok(message)
    }

    pub async fn join_room(self, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        RoomRepo::join_room(&self.pool, room_id, user_id).await?;
        Ok(())
    }

    pub async fn leave_room(self, room_id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        RoomRepo::leave_room(&self.pool, room_id, user_id).await?;
        Ok(())
    }

    pub async fn get_user_join_status(
        self,
        room_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        let is_member = RoomRepo::is_member(&self.pool, user_id, room_id)
            .await?
            .ok_or_else(|| AppError::Failed("Failed to fetch user join status".into()))?;

        Ok(is_member)
    }

    pub async fn register_member(
        state: &AppState,
        room_id: Uuid,
        user_id: Uuid,
        username: String,
        tx: mpsc::Sender<ServerEvent>,
    ) {
        let mut rooms = state.rooms.lock().await;
        let room = rooms.entry(room_id).or_insert(RoomState {members: HashMap::new()});

        room.members.insert(user_id, Member {username, tx});
    }

    pub async fn unregister_member(state: &AppState, room_id: Uuid, user_id: &Uuid) {
        let mut rooms = state.rooms.lock().await;

        if let Some(room) = rooms.get_mut(&room_id) {
            room.members.remove(user_id);
        }
    }

    pub async fn broadcast_presence(state: &AppState, room_id: &Uuid, username: String, kind: PresenceKind) {
        let rooms = state.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            for member in room.members.values() {
                let _ = member.tx.send(ServerEvent::Presence { user: username.clone(), kind }).await;
            }
        }
    }

    pub async fn broadcast_message(state: &AppState, room_id: &Uuid, server_event: ServerEvent) {
        let rooms = state.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            for m in room.members.values() {
                let _ = m.tx.send(server_event.clone()).await;
            }
        }
    }

    pub async fn get_active_members(state: &AppState, room_id: &Uuid) -> Result<Vec<Members>, AppError> {
        let rooms = state.rooms.lock().await;
        let mut members: Vec<Username> = Vec::new();

        if let Some(room) = rooms.get(room_id) {

            for m in room.members.values() {
                members.push(m.username.to_string());
            }
        }

        UserRepo::fetch_users_by_username(&state.pool, members).await
    }

    pub async fn return_message(state: &AppState, room_id: &Uuid, user_id: &Uuid, server_event: ServerEvent) {
        let rooms = state.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            if let Some(member) = room.members.get(user_id) {
                let _ = member.tx.send(server_event).await;
            }
        }
    }
}