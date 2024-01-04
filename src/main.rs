mod remote_work;
mod utils;

use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
pub struct AppState {
    uri: String,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let app_state = AppState {
        uri: secret_store.get("URI_ENDPOINT").unwrap(),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .with_state(app_state);

    Ok(router.into())
}
