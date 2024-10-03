use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

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
    let response: Vec<Value> = client
        .get(leaky_url.join("/writing").unwrap())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut posts: Vec<Item> = response.iter().filter_map(parse_item_data).collect();

    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok((StatusCode::OK, Json(posts)))
}

#[derive(Debug, thiserror::Error)]
pub enum GetItemsError {}

impl IntoResponse for GetItemsError {
    fn into_response(self) -> Response {
        match self {}
    }
}

use time::Date;

fn parse_item_data(value: &Value) -> Option<Item> {
    let v_array = value.as_array().unwrap();
    let v_name: String = v_array.get(0).unwrap().as_str().unwrap().to_string();
    let v_values = v_array.get(1);
    v_values
        .and_then(|values| values.get(1))
        .and_then(|data| serde_json::from_value::<ItemData>(data.clone()).ok())
        .map(|data| {
            let year = data.created_at[0] as i32;
            let day_of_year = data.created_at[1] as u16;
            let date = Date::from_ordinal_date(year, day_of_year).unwrap();

            Item {
                name: v_name,
                title: data.metadata.title,
                description: data.metadata.description,
                created_at: date.with_hms(0, 0, 0).unwrap().assume_utc(),
            }
        })
}
