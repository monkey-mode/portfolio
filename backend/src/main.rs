mod api;
mod db;
mod models;

use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use api::handlers::{get_profile, get_projects, get_skills};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    tracing::info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
        
    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await?;
        
    tracing::info!("Migrations applied successfully.");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/profile", get(get_profile))
        .route("/api/projects", get(get_projects))
        .route("/api/skills", get(get_skills))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let role_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", role_addr);
    
    let listener = tokio::net::TcpListener::bind(role_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
