use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use axum_extra::extract::CookieJar;
use axum_macros::debug_handler;
use tower_cookies::Cookie;
use time::Duration;

use crate::{
    common::{error::{AppError, ValidationError}, response::ApiResponse},
    modules::user::model::{LoginCredentials, LoginDto, SignUpCredentials, SignUpDto, UpdateVisibility, UserId},
    state::AppState,
    utils::jwt::create_jwt_token,
};

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(user): Json<SignUpDto>,
) -> Result<impl IntoResponse, AppError> {
    let new_user: SignUpCredentials = user.try_into()?;

    let user = state.user_service.create(new_user).await?;

    let jwt = create_jwt_token(user.id, state.jwt_encoding)
        .await
        .map_err(|_| AppError::Validation(ValidationError::FailedToCreateToken))?;

    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));

    Ok((
        jar,
        Json(ApiResponse::success("User created successfuly", user)),
    ))
}

pub async fn login_user(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(dto): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    let new_user: LoginCredentials = dto.try_into()?;

    let user = state.user_service.login(new_user).await?;

    let jwt = create_jwt_token(user.id, state.jwt_encoding)
        .await
        .map_err(|_| AppError::Validation(ValidationError::FailedToCreateToken))?;

    let jar = cookies.add(Cookie::build(("jwt", jwt)).http_only(true).path("/"));

    Ok((
        jar,
        Json(ApiResponse::success("User login successfuly", user)),
    ))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove(
        Cookie::build(("jwt", ""))
            .http_only(true)
            .path("/")
            .max_age(Duration::seconds(0)),
    );

    (
        jar,
        Json(ApiResponse::success("User logout successfuly", None::<()>)),
    )
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {

    state.user_service.delete(user_id.0).await?;

    Ok(Json(ApiResponse::success("User deleted successfuly", None::<()>)))
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let user = state.user_service.get(user_id.0).await?;

    Ok(Json(ApiResponse::success("fetch user successfuly", user)))
}

pub async fn get_user_by_username_handler(
    State(state): State<AppState>,
    user_id: Option<Extension<UserId>>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    let user = state.user_service.get_user_by_username(&username).await?; 

    if !user.is_public {
        let requester_id = user_id
            .map(|Extension(user_id)| user_id.0)
            .ok_or(AppError::Validation(ValidationError::UnauthorizedAccess))?;

        if user.id != requester_id {
            return Err(AppError::Validation(ValidationError::UnauthorizedAccess));
        } 
    }

    Ok(Json(ApiResponse::success("fetch user successfuly", user)))
}

#[debug_handler]
pub async fn change_user_visibility_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(visibility): Json<UpdateVisibility>
) -> Result<Json<ApiResponse<impl serde::Serialize>>, AppError> {
    state.user_service.change_visibility(user_id.0, visibility.is_public).await?;

    Ok(Json(ApiResponse::success("User account visibility changed successfully", None::<()>)))
}
