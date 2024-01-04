use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    remote_work::begin_opt::{populate_data, request_bytes},
    AppState,
};

mod begin_opt;
mod html_parse;
mod mapped_detail;
mod parse_xml;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobPost {
    pub title: String,
    pub link: String,
    pub category: String,
    pub detail: HashMap<String, String>,
    pub posted_on: String,
    pub posted_timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct QueryMap {
    recency: Option<String>,
    q: Option<String>,
}

pub async fn get_job(
    Query(query_data): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Json<Vec<Value>> {
    let uri = &state.uri;

    let bytes_data = request_bytes(uri, query_data).await;

    match bytes_data {
        Ok(data) => match populate_data(data) {
            Ok(arr_data) => HttpResponse::Ok().json(arr_data),
            _ => HttpResponse::InternalServerError().finish(),
        },
        _ => HttpResponse::InternalServerError().finish(),
    }
}
