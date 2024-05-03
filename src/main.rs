use axum::{
    routing::{get, post},
    Json, Router,
};
use nostr_sdk::prelude::*;
use serde_json::{json, Value};
use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod cache;
mod models;
mod views;
use crate::views::nwc::handlers::{create_customer_nwc, get_customer_nwc};
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

    // init_env();

    // dotenv::dotenv().ok();

    let _nostr_relay = std::env::var("NOSTR_RELAY").expect("NOSTR_RELAY not set");

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
        .route("/nwc/:id", get(get_customer_nwc))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn health() -> Json<Value> {
    Json(json!({ "health": "ok" }))
}

// fn init_env() {
//     let database_url = "DATABASE_URL=";
//     let database_url_environment = std::env::var("DATABASE_URL").expect("DATABASE_URL not set.");
//     let database_url_environment = format!(
//         "{}{}\nSQLX_OFFLINE=true",
//         database_url, database_url_environment
//     );

//     let mut data_file = File::create(".env").expect("creation failed");
//     data_file
//         .write_all(database_url_environment.as_bytes())
//         .expect("write failed");

//     println!("Created a file .env");
// }
