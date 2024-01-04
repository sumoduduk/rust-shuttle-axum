use axum::body::Bytes;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

use super::parse_xml::parse_xml;

pub async fn request_bytes(
    uri: &str,
    query_data: HashMap<String, String>,
) -> Result<Bytes, reqwest::Error> {
    let response_byte = Client::new()
        .get(uri)
        .query(&query_data)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(response_byte)
}

pub fn populate_data(byte_data: Bytes) -> eyre::Result<Vec<Value>> {
    let result_data = parse_xml(&byte_data[..])?;

    let list_job: Vec<Value> = result_data
        .into_iter()
        .map(|j| {
            let budget = j.detail.get("Budget");
            let hourly = j.detail.get("Hourly Range");

            let title_job: Vec<_> = j.title.split("- Upwo").collect();

            let mut price = "Unknown".to_string();

            match (budget, hourly) {
                (Some(b), None) => {
                    price = format!("Budget : {}", b);
                }
                (None, Some(h)) => {
                    price = format!("Hourly Range : {}", h);
                }
                (_, _) => (),
            }
            let response_json = json!({ "title": title_job[0], "link": j.link, "price": price });
            response_json
        })
        .collect();

    Ok(list_job)
}
