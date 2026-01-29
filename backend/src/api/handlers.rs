use axum::{
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use crate::db::Repository;

pub async fn get_profile(State(pool): State<PgPool>) -> impl IntoResponse {
    let repo = Repository::new(pool);
    match repo.get_profile().await {
        Ok(Some(profile)) => (StatusCode::OK, Json(profile)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Profile not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch profile: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn get_projects(State(pool): State<PgPool>) -> impl IntoResponse {
    let repo = Repository::new(pool);
    match repo.get_projects().await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch projects: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn get_skills(State(pool): State<PgPool>) -> impl IntoResponse {
    let repo = Repository::new(pool);
    match repo.get_skills().await {
        Ok(skills) => (StatusCode::OK, Json(skills)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch skills: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
