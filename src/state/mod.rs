use std::{collections::{HashMap, HashSet}, sync::Arc};

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;
use tokio::sync::{broadcast, Mutex};

use crate::modules::{progress::service::ProgressService, rooms::{model::{ServerEvent}, service::RoomService}, todo::service::TodoService, user::service::UserService};

#[derive(Clone)]
pub struct RoomState {
    pub tx: broadcast::Sender<ServerEvent>,
    pub members: HashSet<String>
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
    pub todo_service: TodoService,
    pub user_service: UserService,
    pub progress_service: ProgressService,
    pub room_service: RoomService,
    pub rooms: Arc<Mutex<HashMap<String, RoomState>>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    // pub role: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // current timestamp
}


