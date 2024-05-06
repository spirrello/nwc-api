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
mod models;
mod views;
use crate::views::nwc::handlers::*;
use cache::*;

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

    init_env();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set.");
    let pool = sqlx::postgres::PgPool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let redis_pool = tokio::task::spawn_blocking(create_redis_pool)
        .await
        .unwrap();
    let state = AppState {
        db: pool,
        cache: redis_pool,
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/health", get(health))
        .route("/nwc", post(create_customer_nwc))
        .route("/nwc/:id", get(get_customer_nwc).post(update_customer_nwc))
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

fn init_env() {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    std::env::var("NOSTR_RELAY").expect("NOSTR_RELAY is not set.");
    std::env::var("REDIS_URL").expect("REDIS_URL not set.");
}
