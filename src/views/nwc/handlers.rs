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
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use tracing::{error, info};
// use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NwcRequest {
    uuid: String,
    app_service: String,
    budget: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NwcResponseErrorMessage {
    status: String,
    error: String,
}

impl fmt::Display for NwcResponseErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"status\": \"{}\",\"error\": \"{}\"}}",
            self.status, self.error
        )
    }
}
impl Error for NwcResponseErrorMessage {}

pub async fn create_customer_nwc(
    State(shared_state): State<Arc<AppState>>,
    Json(req): Json<NwcRequest>,
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
    .fetch_one(&shared_state.db)
    .await {
        Err(e) => {
            error!("error creating nwc: {}", e);
            // The return is required otherwise the HTTP response code will be 200
            let error_message = NwcResponseErrorMessage {
                status: "failed".to_string(),
                error: e.to_string(),
            };
            return (StatusCode::BAD_REQUEST, axum::response::Json(error_message)).into_response();
        },
        Ok(_) => {
            info!("CustomerNwc generated successfully");
            let response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "nwc": customer_nwc_response
                })
            });
            return axum::response::Json(response).into_response();

            // waiting to add redis
            // match redis_conn.set::<String, String, String>(customer.email.clone(), uuid).await {
            //     Ok(_) => {
            //         tracing::info!("customer setup successful");
            //         Ok((successCode::OK, "customer setup successful").into_response())
            //     }
            //     Err(e) => Ok((successCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
            // }
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
        "select * from customer_nwc where uuid = $1",
        uuid
    )
    .fetch_one(&shared_state.db)
    .await
    {
        Err(e) => {
            error!("customer nwc not found: {}", e);
            let error_message = NwcResponseErrorMessage {
                status: "failed".to_string(),
                error: e.to_string(),
            };
            return (StatusCode::NOT_FOUND, axum::response::Json(error_message)).into_response();
        }
        Ok(nwc) => {
            let response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "nwc": nwc
                })
            });
            return axum::response::Json(response).into_response();
        }
    };
}
