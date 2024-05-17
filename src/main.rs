mod data;
mod extract;
mod index;
use crate::data::{Bookmark, RequestData};
use dotenv::dotenv;
use index::index_bookmarks;
use meilisearch_sdk::{client::*, TaskInfo};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = Client::new("http://localhost:7700", Some("masterKey"));

    let bookmarks = client.get_index("bookmarks").await;

    if let Err(_) = bookmarks {
        client.create_index("bookmarks", Some("id")).await.unwrap();

        let searchable_attributes = ["id", "author_id", "author_name", "full_text"];

        let task: TaskInfo = client
            .index("bookmarks")
            .set_searchable_attributes(&searchable_attributes)
            .await
            .unwrap();

        client.wait_for_task(task, None, None).await.unwrap();
    }

    let auth_bearer = env::var("AUTH_BEARER").expect("AUTH_BEARER not set in .env");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set in .env");
    let csrf = env::var("CSRF").expect("CSRF not set in .env");
    let url = env::var("URL").expect("URL not set in .env");

    let request_data = RequestData {
        auth_bearer,
        auth_token,
        csrf,
        url,
    };

    let mut cursor: Option<String> = None;
    let mut all_indexed_bookmarks: Vec<Bookmark> = Vec::new();

    loop {
        println!("cursor: {:?}", cursor);

        let (new_cursor, indexed_bookmarks) =
            index_bookmarks(&request_data, cursor.as_deref(), &client)
                .await
                .unwrap();

        if let Some(bookmarks) = indexed_bookmarks {
            if !bookmarks.is_empty() {
                all_indexed_bookmarks.extend(bookmarks);
            }
        }

        if new_cursor.is_none() {
            break;
        }

        cursor = new_cursor;
    }

    // Write the bookmarks to a JSON file and indent this JSON.
    if all_indexed_bookmarks.len() > 0 {
        let json = serde_json::to_string_pretty(&all_indexed_bookmarks).unwrap();
        std::fs::write("bookmarks.json", json).unwrap();
    }
}
