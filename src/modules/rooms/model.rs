use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{OffsetDateTime};
use uuid::Uuid;

use crate::{common::error::{AppError, ValidationError}, modules::rooms::service::Username};

#[derive(FromRow, Serialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub profile_pic: Option<String>,
    pub owner_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct Member {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

#[derive(FromRow)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub created_at: OffsetDateTime ,
}
#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub user_name: String,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub created_at: OffsetDateTime
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum ClientEvent {
    ChatSend {content: String, parent_id: Option<Uuid>},
    Ping,
    Typing {is_typing: bool},
    ActiveMembers,
    AllMembers
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum ServerEvent {
    ChatMessage (MessageResponse),
    History(Vec<MessageResponse>),
    Presence { user: String, kind: PresenceKind },
    Pong,
    Typing {username: String, is_typing: bool},
    ActiveMembers (Vec<Members>),
    AllMembers (Vec<Members>)
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum PresenceKind {
    Join,
    Leave
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Members {
    pub user_id: Uuid,
    pub name: String,
    pub username: String
}

#[derive(Debug, Deserialize)]
pub struct RoomDto {
    pub name: String,
    pub description: Option<String>,
    pub profile_pic: Option<String>
}

impl RoomDto {
    pub fn validate(dto: RoomDto) -> Result<Self, AppError> {
        let name = dto.name.trim();
        let description = dto.description;
        let profile_pic = dto.profile_pic;

        if let Some(des) = description.clone() {
            if des.trim().len() == 0 {
                return Err(AppError::Validation(ValidationError::DescriptionCanNotBeNull))
            }
        };

        if let Some(pic) = profile_pic.clone() {
            if pic.trim().len() == 0 {
                return Err(AppError::Validation(ValidationError::InvalidProfilePicUrl))
            }
        };

        Ok(Self {
            name: name.to_string(),
            description,
            profile_pic
        })
        
    }
}

pub struct MessageDto {
    pub room_id: Uuid,
    pub content: String,
    pub parent_id: Option<Uuid>,
}

impl MessageDto {
    pub fn validate(dto: MessageDto) -> Result<Self, AppError> {
        let room_id = dto.room_id;
        let content = dto.content.trim();
        let parent_id = dto.parent_id;

        if content.len() == 0 {
            return Err(AppError::Validation(ValidationError::InvalidMessage));
        }

        Ok(Self {
            content: content.to_string(),
            room_id,
            parent_id
        })
    }
}
