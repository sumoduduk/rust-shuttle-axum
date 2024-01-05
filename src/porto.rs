mod notion_contact;
mod notion_porto;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use notion_contact::{convert_to_json, push_notion};
use notion_porto::get_porto;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{porto::notion_porto::extract_payload, AppState};

#[derive(Serialize, Deserialize)]
pub struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

pub async fn push_contact(
    State(app_state): State<AppState>,
    Json(payload): Json<ContactPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let porto_state = app_state.porto_state;
    let notion_db = porto_state.db_id;
    let notion_url = porto_state.porto_url;
    let bearer = porto_state.porto_bearer;
    let extracted_val = convert_to_json(&payload, &notion_db);
    let response_json = push_notion(extracted_val, &notion_url, &bearer).await;

    match response_json {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}

pub async fn api_porto(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Value>>, (StatusCode, String)> {
    let porto_state = app_state.porto_state;
    let bearer = porto_state.porto_bearer;

    let bearer = format!("Bearer {bearer}");
    let porto_id = porto_state.porto_id;
    let notion_url = format!("https://api.notion.com/v1/databases/{}/query", porto_id);

    let payload = json!({
          "filter": {
              "property": "Status",
              "status": {
                  "equals":"Done",
              },
          },
    "sorts": [
      {
        "property": "priority",
        "direction": "ascending"
      }
    ]
      });

    let res = get_porto(&payload, &bearer, &notion_url).await;

    match res {
        Ok(data) => {
            let value_response = extract_payload(&data);
            match value_response {
                Some(formated_resp) => Ok(Json(formated_resp)),
                None => Err((
                    StatusCode::NOT_FOUND,
                    "Error when extracting data".to_owned(),
                )),
            }
        }
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}

pub async fn porto_route() -> Router {
    Router::new()
        .route("/get_porto", get(api_porto))
        .route("/contact_form", post(push_contact))
}

