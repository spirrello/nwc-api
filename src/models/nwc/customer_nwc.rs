use bitcoin::secp256k1::SecretKey;
use nostr::Keys;
use nostr_sdk::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct CustomerNwc {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub server_key: String,
    pub user_key: String,
    pub uri: String,
    pub app_service: String,
    pub budget: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CustomerNwcResponse {
    pub server_key: SecretKey,
    pub user_key: SecretKey,
    pub uri: NostrWalletConnectURI,
}

impl CustomerNwcResponse {
    pub fn generate() -> Self {
        let nostr_relay = std::env::var("NOSTR_RELAY").expect("NOSTR_RELAY not set");
        let server_key = Keys::generate();
        let user_key = Keys::generate();
        let user_secret_key = **user_key.secret_key().unwrap();

        let nwc_uri: NostrWalletConnectURI = NostrWalletConnectURI::new(
            server_key.public_key(),
            nostr_relay.parse().unwrap(),
            user_secret_key.into(),
            None,
        );

        CustomerNwcResponse {
            server_key: **server_key.secret_key().unwrap(),
            user_key: **user_key.secret_key().unwrap(),
            uri: nwc_uri,
        }
    }
}
