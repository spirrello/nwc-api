use crate::{
    models::nwc::customer_nwc::{CustomerNwc, CustomerNwcResponse},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
// use uuid::Uuid;

use crate::views::{into_axum_error_response, into_axum_success_response};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewNwcRequest {
    uuid: String,
    app_service: String,
    budget: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DeleteNwcRequest {
    uuid: String,
    app_service: String,
}

pub async fn create_customer_nwc(
    State(shared_state): State<Arc<AppState>>,
    Json(req): Json<NewNwcRequest>,
) -> impl IntoResponse {
    info!(
        "create_customer_nwc: {:?}",
        serde_json::to_string(&req).unwrap()
    );
    let customer_nwc_response = CustomerNwcResponse::generate();

    let customer_nwc_response_uri = customer_nwc_response.uri.clone().to_string();
    let customer_nwc_response_server_key: String = customer_nwc_response
        .server_key
        .clone()
        .display_secret()
        .to_string();

    let customer_nwc_response_user_key = customer_nwc_response
        .user_key
        .clone()
        .display_secret()
        .to_string();
    match sqlx::query_as!(
        CustomerNwc,
        "INSERT INTO customer_nwc (uuid, server_key, user_key, uri, app_service, budget) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        req.uuid.clone(),
        customer_nwc_response_server_key,
        customer_nwc_response_user_key,
        customer_nwc_response_uri,
        req.app_service.clone(),
        req.budget
    )
    .fetch_all(&shared_state.db)
    .await {
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    info!("nwc not found: {}", e);
                    let error_message = "row not found";
                    return (StatusCode::NOT_FOUND, into_axum_error_response(error_message));
                },
                sqlx::Error::Database(err) if err.message().contains("duplicate") => {
                    error!("duplicate entry: {}", err);
                    let error_message = "duplicate entry";
                    return (StatusCode::BAD_REQUEST, into_axum_error_response(error_message));
                },
                _ => {
                    error!("database error: {}", e);
                    let error_message = "database error";
                    return (StatusCode::INTERNAL_SERVER_ERROR, into_axum_error_response(error_message));
                }
            }
        },
        Ok(_) => {
            info!("CustomerNwc generated successfully");
            let data_vec = vec![customer_nwc_response];
            return (StatusCode::OK, into_axum_success_response(data_vec));
        },
    };
}

pub async fn get_customer_nwc(
    State(shared_state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    info!("searching for {}", uuid);

    match sqlx::query_as!(
        CustomerNwc,
        "SELECT * FROM customer_nwc WHERE uuid = $1",
        uuid
    )
    .fetch_all(&shared_state.db)
    .await
    {
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                info!("nwc not found: {}", e);
                let error_message = "nwc not found";
                return (
                    StatusCode::NOT_FOUND,
                    into_axum_error_response(error_message),
                );
            }
            _ => {
                error!("database error: {}", e);
                let error_message = "database error";
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    into_axum_error_response(error_message),
                );
            }
        },
        Ok(nwc) => {
            let data_vec = vec![nwc];
            return (StatusCode::OK, into_axum_success_response(data_vec));
        }
    };
}

pub async fn update_customer_nwc(
    State(shared_state): State<Arc<AppState>>,
    Json(req): Json<CustomerNwc>,
) -> impl IntoResponse {
    info!("updating {:?}", req.uuid);

    let app_service = req.app_service.clone();
    let budget = req.budget;
    let customer_nwc_server_key = req.server_key.clone();
    let customer_nwc_user_key = req.user_key.clone();
    let customer_nwc_uri = req.uri.clone();
    let uuid = req.uuid.clone().unwrap();

    let query = format!("UPDATE customer_nwc SET server_key = '{customer_nwc_server_key}', user_key = '{customer_nwc_user_key}', uri = '{customer_nwc_uri}', app_service = '{app_service}', budget = '{budget}' WHERE uuid = '{uuid}'");
    match sqlx::query(&query).fetch_one(&shared_state.db).await {
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                info!("nwc not found: {}", e);
                let error_message = "nwc not found";
                return (
                    StatusCode::NOT_FOUND,
                    into_axum_error_response(error_message),
                );
            }
            _ => {
                error!("database error: {}", e);
                let error_message = "database error";
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    into_axum_error_response(error_message),
                );
            }
        },
        Ok(_) => {
            let data_vec = vec![req];
            return (StatusCode::ACCEPTED, into_axum_success_response(data_vec));
        }
    };
}

pub async fn delete_customer_nwc(
    State(shared_state): State<Arc<AppState>>,
    Path((uuid, app_service)): Path<(String, String)>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        CustomerNwc,
        "DELETE FROM customer_nwc WHERE uuid = $1 AND app_service = $2",
        uuid,
        app_service
    )
    .execute(&shared_state.db)
    .await
    {
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                info!("nwc not found: {}", e);
                let error_message = "row not found";
                return (
                    StatusCode::NOT_FOUND,
                    into_axum_error_response(error_message),
                );
            }
            _ => {
                error!("database error: {}", e);
                let error_message = "database error";
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    into_axum_error_response(error_message),
                );
            }
        },
        Ok(_) => {
            let data_vec = vec![""];

            return (StatusCode::OK, into_axum_success_response(data_vec));
        }
    };
}
