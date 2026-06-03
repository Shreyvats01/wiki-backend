use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};

use crate::{
    middleware::auth::auth_middleware,
    modules::{
        main::handler::ws_handler, progress::handler::{
            create_daily_progress_handler, create_daily_progress_task_handler,
            delete_daily_progress_task_handler, fetch_all_daily_progress_tasks,
            fetch_daily_progress_task_by_id, is_progress_exits_handler,
            toggle_daily_progress_task_handler,
        }, task::handler::{
            create_category_handler, create_tag_handler, delete_category_handler,
            delete_tag_handler, delete_task_handler, fetch_all_categories_handler,
            fetch_all_tags_handler, update_task_handler,
        }, user::handler::{
            change_user_visibility_handler, create_user, delete_user_handler, get_user_by_username_handler, get_user_handler, login_user, logout
        }
    },
    state::AppState,
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", protected_routes())
        .route_layer(from_fn_with_state(state.clone(), auth_middleware))
        .nest("/api", routes())
        .with_state(state)
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/room/{room_id}", get(ws_handler))
        // .route("/task/add", post(create_task_handler))
        // .route("/task/get/{id}", get(get_task_handler))
        .route("/task/update/{id}", put(update_task_handler))
        .route("/task/remove/{id}", delete(delete_task_handler))
        .route("/user/delete", delete(delete_user_handler))
        .route("/user/me", get(get_user_handler))
        .route("/user/logout", post(logout))
        .route(
            "/user/update_visibility",
            put(change_user_visibility_handler),
        )
        .route("/tag/add", post(create_tag_handler))
        .route("/tag/{slug}", delete(delete_tag_handler))
        .route("/tag/all", get(fetch_all_tags_handler))
        .route("/category/add", post(create_category_handler))
        .route("/category/{slug}", delete(delete_category_handler))
        .route("/category/all", get(fetch_all_categories_handler))
        .route("/progress", post(create_daily_progress_handler))
        .route(
            "/progress/task/create/{daily_progress_id}",
            post(create_daily_progress_task_handler),
        )
        .route(
            "/progress/task/{progress_task_id}",
            get(fetch_daily_progress_task_by_id)
                .put(toggle_daily_progress_task_handler)
                .delete(delete_daily_progress_task_handler),
        )
        .route(
            "/progress/tasks/{daily_progress_id}",
            get(fetch_all_daily_progress_tasks),
        )
        .route("/progress/is_exits/{day}", get(is_progress_exits_handler))
    }
    
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
        .route("/user/{username}", get(get_user_by_username_handler))
}
