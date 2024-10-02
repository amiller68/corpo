use axum::extract::{Json, State};
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use time::OffsetDateTime;
use serde_json::Value;

use crate::app::AppState;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ItemMetadata {
    description: String,
    title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ItemData {
    created_at: [i64; 9],
    updated_at: [i64; 9],
    metadata: ItemMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    title: String,
    description: String,
    created_at: OffsetDateTime,
}

pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, GetItemsError> {
        let leaky_url = state.leaky_url.clone();
        let client = Client::new();
        let response: Vec<Value> = client.get(leaky_url.join("/writing").unwrap())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut posts: Vec<Item> = response
            .iter()
            .filter_map(parse_item_data)
            .collect();
        
        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok((StatusCode::OK, Json(posts)))
}

#[derive(Debug, thiserror::Error)]
pub enum GetItemsError {}

impl IntoResponse for GetItemsError {
    fn into_response(self) -> Response {
        match self {
        }
    }
}

fn parse_item_data(value: &Value) -> Option<Item> {
    let v_array = value.as_array().unwrap();

    // first value in the array is the file name
    let v_name: String = v_array.get(0).unwrap().as_str().unwrap().to_string();
    // second value is the associated metadata
    let v_values = v_array.get(1);

    let v = v_values
        // Second value in this array if it exists is the metadata
        .and_then(|values| values.get(1))
        .and_then(|data| serde_json::from_value::<ItemData>(data.clone()).ok())
        .map(|data| Item {
            name: v_name,
            title: data.metadata.title,
            description: data.metadata.description,
            created_at: OffsetDateTime::from_unix_timestamp(
                data.created_at[0] * 31_557_600 +
                data.created_at[1] * 86_400 +
                data.created_at[2] * 3_600 +
                data.created_at[3] * 60 +
                data.created_at[4]
            ).unwrap(),
        });

    v
}