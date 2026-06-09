use tower_http::cors::CorsLayer;

use crate::{
    modules::{
        progress::service::ProgressService, task::service::TaskService, user::service::UserService,
    },
    routes::create_app,
    state::AppState,
    utils::{config::Config, db::init_db_pool},
};
use axum::{
    http::{HeaderValue, Method, header},
    response::Result,
};
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

mod common;
mod middleware;
mod modules;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = Config::DatabaseUrl.from_env()?;
    let secret = Config::JsonWebTokenSecret.from_env()?;

    let pool: PgPool = init_db_pool(&db_url).await?;

    let state: AppState = AppState {
        pool: pool.clone(),
        jwt_decoding: DecodingKey::from_secret(secret.as_bytes()),
        jwt_encoding: EncodingKey::from_secret(secret.as_bytes()),
        task_service: TaskService::new(pool.clone()),
        user_service: UserService::new(pool.clone()),
        progress_service: ProgressService::new(pool.clone()),
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let app = create_app(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
