use crate::data::{Bookmark, RequestData};
use crate::extract::extract_bookmarks_from_response;
use meilisearch_sdk::client::*;
use reqwest;
use reqwest::header;

/// Index bookmarks for a given pagination cursor
pub async fn index_bookmarks(
    request_data: &RequestData,
    cursor: Option<&str>,
    meili_client: &Client,
) -> Result<(Option<String>, Option<Vec<Bookmark>>), reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", &request_data.auth_bearer)).unwrap(),
    );
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!(
            "auth_token={}; ct0={};",
            &request_data.auth_token, &request_data.csrf
        ))
        .unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::HeaderName::from_static("x-csrf-token"),
        header::HeaderValue::from_str(&request_data.csrf).unwrap(),
    );

    let old_cursor = cursor.clone();

    let url = parse_url_params(&request_data.url, cursor);

    println!("Fetching {}", url);

    let response = client.get(url).headers(headers).send().await?;

    // Ensure the request was successful
    response.error_for_status_ref()?;

    // Parse the JSON response
    let json_result: serde_json::Value = response.json().await?;

    let (bookmarks, cursor) = extract_bookmarks_from_response(&json_result);

    if old_cursor == cursor.as_deref() {
        return Ok((None, None));
    }

    if bookmarks.len() == 0 {
        return Ok((None, None));
    }

    meili_client
        .index("bookmarks")
        .add_documents(&bookmarks, Some("id"))
        .await
        .unwrap();

    println!("Indexed {} bookmarks", bookmarks.len());

    Ok((cursor, Some(bookmarks)))
}

/// Take a base API URL and construct a new one with an updated cursor
fn parse_url_params(api_url: &str, cursor: Option<&str>) -> String {
    let mut parsed_url = url::Url::parse(api_url).unwrap();

    let params: Vec<(String, String)> = parsed_url
        .query_pairs()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();

    let new_params: Vec<(String, String)> = params
        .iter()
        .map(|(key, value)| {
            if key == "variables" {
                if cursor.is_some() {
                    (
                        key.to_string(),
                        format!(
                            "{{\"count\":100,\"cursor\":{},\"includePromotedContent\":true}}",
                            cursor.unwrap()
                        ),
                    )
                } else {
                    (
                        key.to_string(),
                        "{\"count\":100,\"includePromotedContent\":true}".to_string(),
                    )
                }
            } else {
                (key.to_string(), value.to_string())
            }
        })
        .collect();

    // clear the query pairs
    parsed_url.query_pairs_mut().clear();

    // create a new url
    let new_url = url::Url::parse_with_params(
        parsed_url.as_str(),
        new_params
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str())),
    )
    .unwrap();

    new_url.to_string()
}
