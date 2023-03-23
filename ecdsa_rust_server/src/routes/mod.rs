mod app_error;
mod handlers;

use axum::{
    http::{header, Method},
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use handlers::{address_balance, send_to_address, Degen};

pub struct AppState {
    balances: Mutex<HashMap<String, Degen>>,
}

pub fn create_routes() -> Router {
    let balances = Mutex::new(HashMap::new());
    let app_state = Arc::new(AppState { balances });
    app_state
        .balances
        .lock()
        .unwrap()
        .insert("0x1".to_string(), Degen { balance: 100 });
    app_state
        .balances
        .lock()
        .unwrap()
        .insert("0x2".to_string(), Degen { balance: 50 });
    app_state
        .balances
        .lock()
        .unwrap()
        .insert("0x3".to_string(), Degen { balance: 75 });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(vec![header::CONTENT_TYPE]);

    Router::new()
        .route("/balance/:address", get(address_balance))
        .route("/send", post(send_to_address))
        .layer(Extension(app_state))
        .layer(cors)
}
