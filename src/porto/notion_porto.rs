use reqwest::Client;
use serde_json::{json, Value};

pub async fn get_porto(payload: &Value, bearer: &str, url: &str) -> Result<Value, reqwest::Error> {
    let response: Value = Client::new()
        .post(url)
        .header("Authorization", bearer)
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-06-28")
        .json(payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub fn extract_payload(value: &Value) -> Option<Vec<Value>> {
    let results = value.get("results")?;
    let arr_res = results.as_array()?;

    let mut val_vec = Vec::new();

    for res in arr_res {
        let properties = res.get("properties")?;

        let title = &properties["Name"]["title"][0]["plain_text"];
        let banner = &properties["Banner"]["files"][0]["name"];
        let description = &properties["Description"]["rich_text"][0]["plain_text"];
        let id_description = &properties["Iddescription"]["rich_text"][0]["plain_text"];
        let link = &properties["link"]["url"];

        let stacks_raw = &properties["Stack"]["multi_select"].clone();
        let stacks = stacks_raw.as_array()?;
        let len = stacks.len();

        let mut new_stacks = Vec::with_capacity(len);

        for stack in stacks {
            let st = stack.get("name")?;

            new_stacks.push(st)
        }

        let json_val = json!({
        "title" : title,
        "banner" : banner,
        "description": description,
        "descriptionId": id_description,
        "link": link,
        "stacks" : new_stacks,
        });
        val_vec.push(json_val);
    }

    dbg!(&val_vec);

    Some(val_vec)
}

