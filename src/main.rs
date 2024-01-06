mod error;
mod porto;
mod remote_work;
mod utils;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use porto::{api_porto, push_contact};
use remote_work::get_job;
use shuttle_secrets::SecretStore;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct PortoState {
    porto_id: String,
    porto_bearer: String,
    db_id: String,
    porto_url: String,
}

#[derive(Clone)]
pub struct AppState {
    uri: String,
    porto_state: PortoState,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let porto_state = PortoState {
        porto_id: secret_store.get("NOTION_PORTO_ID").unwrap(),
        porto_bearer: secret_store.get("BEARER").unwrap(),
        db_id: secret_store.get("NOTION_DB_ID").unwrap(),
        porto_url: secret_store.get("NOTION_URL").unwrap(),
    };

    let secret_uri = secret_store.get("URI_ENDPOINT").unwrap();

    let app_state = AppState {
        uri: secret_uri,
        porto_state,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any)
        .allow_origin(Any);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/search_job", get(get_job))
        .route("/porto/get_porto", get(api_porto))
        .route("/porto/contact_form", post(push_contact))
        .layer(cors)
        .with_state(app_state);

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

