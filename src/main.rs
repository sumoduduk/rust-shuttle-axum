mod error;
mod remote_work;
mod utils;

use axum::{http::Method, routing::get, Router};
use remote_work::get_job;
use shuttle_secrets::SecretStore;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    uri: String,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret_uri = secret_store.get("URI_ENDPOINT").unwrap();

    let app_state = AppState { uri: secret_uri };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any)
        .allow_origin(Any);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/search_job", get(get_job))
        .layer(cors)
        .with_state(app_state);

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

