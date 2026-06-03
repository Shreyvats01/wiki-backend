
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::modules::{progress::service::ProgressService, todo::service::TodoService, user::service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
    pub todo_service: TodoService,
    pub user_service: UserService,
    pub progress_service: ProgressService,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // current timestamp
}
