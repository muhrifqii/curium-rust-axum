use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};

use crate::{
    types::AppError,
    user::{self, domain::User},
};

pub fn router(api: Arc<user::Service>) -> Router {
    Router::new()
        .route("/", post(create).get(list))
        .route("/{user_id}", get(find_by_id).put(update).delete(delete_user))
        .with_state(api)
}

async fn create(
    State(api): State<Arc<user::Service>>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    let result = api.create(user).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

async fn list(
    State(api): State<Arc<user::Service>>,
) -> Result<impl IntoResponse, AppError> {
    let result = api.list().await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn find_by_id(
    State(api): State<Arc<user::Service>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let result = api.find_by_id(&id).await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(result)))
}

async fn update(
    State(api): State<Arc<user::Service>>,
    Path(id): Path<u64>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    let result = api.update(id, user).await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(result)))
}

async fn delete_user(
    State(api): State<Arc<user::Service>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let deleted = api.delete(&id).await?;
    if !deleted {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
