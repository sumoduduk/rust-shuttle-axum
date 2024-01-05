mod error;
mod remote_work;
mod utils;

use axum::{routing::get, Router};
use remote_work::get_job;
use shuttle_secrets::SecretStore;

#[derive(Clone)]
pub struct AppState {
    uri: String,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret_uri = secret_store.get("URI_ENDPOINT").unwrap();
    dbg!(&secret_uri);

    let app_state = AppState { uri: secret_uri };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/search_job", get(get_job))
        .with_state(app_state);

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

