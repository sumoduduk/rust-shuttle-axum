mod begin_opt;
mod html_parse;
mod mapped_detail;
mod parse_xml;

use crate::{
    error::{internal_error, ResponseError},
    remote_work::begin_opt::{populate_data, request_bytes},
    AppState,
};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobPost {
    pub title: String,
    pub link: String,
    pub category: String,
    pub detail: HashMap<String, String>,
    pub posted_on: String,
    pub posted_timestamp: i64,
}

pub async fn get_job(
    Query(query_data): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Value>>, ResponseError> {
    let uri = &state.uri;

    let bytes_data = request_bytes(uri, query_data)
        .await
        .map_err(internal_error)?;

    let data = populate_data(bytes_data).map_err(internal_error)?;

    Ok(Json(data))
}

