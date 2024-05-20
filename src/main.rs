use axum::{
    routing::{delete, get, post},
    Json, Router,
};
use nostr_sdk::prelude::*;
use serde_json::{json, Value};
use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::info;
use tracing::Level;

mod cache;
mod init_db;
mod models;
mod settings;
mod views;
use crate::views::nwc::handlers::*;
use cache::*;
use init_db::*;
use settings::*;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
    cache: RedisPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let service_config = ServiceConfig::new();
    run_migrations(&service_config).await;
    let pool = sqlx::postgres::PgPool::connect(&service_config.db_url).await?;
    let redis_url = service_config.redis_url.clone();
    let redis_pool = tokio::task::spawn_blocking(move || create_redis_pool(redis_url))
        .await
        .unwrap();

    // Try to get a connection from the pool
    match redis_pool.get().await {
        Ok(_conn) => info!("Redis pool is valid."),
        Err(e) => panic!("Failed to get connection from Redis pool: {}", e),
    }

    let state = AppState {
        db: pool,
        cache: redis_pool,
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/health", get(health))
        .route("/nwc", post(create_customer_nwc))
        .route(
            "/nwc/:id",
            get(get_customer_nwc_from_cache).post(update_customer_nwc),
        )
        .route("/nwc/:uuid/:app_service", delete(delete_customer_nwc))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(shared_state);

    let port = std::env::var("PORT").unwrap_or("9090".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Running on {}", address);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn health() -> Json<Value> {
    Json(json!({ "health": "ok" }))
}
