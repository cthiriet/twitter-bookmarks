use crate::data::Bookmark;
use serde_json::Value;

/// Parse the results from the Twitter API
pub fn extract_bookmarks_from_response(bookmarks: &Value) -> (Vec<Bookmark>, Option<String>) {
    let mut bookmarks_db: Vec<Bookmark> = Vec::new();
    let mut cursor: Option<String> = None;

    let tweets =
        &bookmarks["data"]["bookmark_timeline_v2"]["timeline"]["instructions"][0]["entries"];

    for tweet in tweets.as_array().expect("Expected an array") {
        let entry_id = tweet["entryId"]
            .as_str()
            .expect("Expected a string")
            .replace("tweet-", "");

        if entry_id.starts_with("cursor-") {
            cursor = Some(tweet["content"]["value"].to_string());
            continue;
        }

        let tweet_typename = tweet["content"]["itemContent"]["tweet_results"]["result"]
            ["__typename"]
            .as_str()
            .expect("Expected a string");

        let mut full_text = String::new();
        let mut author = String::new();
        let mut author_id = String::new();
        let user: Value;

        if tweet_typename == "TweetUnavailable" {
            continue;
        }

        if tweet_typename == "TweetWithVisibilityResults" {
            if tweet["content"]["itemContent"]["tweet_results"]["result"]["tweet"]["note_tweet"]
                .is_object()
            {
                full_text = tweet["content"]["itemContent"]["tweet_results"]["result"]["tweet"]
                    ["note_tweet"]["note_tweet_results"]["result"]["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
            } else {
                full_text = tweet["content"]["itemContent"]["tweet_results"]["result"]["tweet"]
                    ["legacy"]["full_text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
            }

            user = tweet["content"]["itemContent"]["tweet_results"]["result"]["tweet"]["core"]
                ["user_results"]["result"]["legacy"]
                .clone();

            author = user["name"]
                .as_str()
                .unwrap_or("Unknown Author")
                .to_string();

            author_id = user["screen_name"]
                .as_str()
                .unwrap_or("Unknown Author")
                .to_string();
        } else if tweet_typename == "Tweet" {
            if tweet["content"]["itemContent"]["tweet_results"]["result"]["note_tweet"].is_object()
            {
                full_text = tweet["content"]["itemContent"]["tweet_results"]["result"]
                    ["note_tweet"]["note_tweet_results"]["result"]["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
            } else {
                full_text = tweet["content"]["itemContent"]["tweet_results"]["result"]["legacy"]
                    ["full_text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
            }

            user = tweet["content"]["itemContent"]["tweet_results"]["result"]["core"]
                ["user_results"]["result"]["legacy"]
                .clone();

            author = user["name"]
                .as_str()
                .unwrap_or("Unknown Author")
                .to_string();

            author_id = user["screen_name"]
                .as_str()
                .unwrap_or("Unknown Author")
                .to_string();
        }

        bookmarks_db.push(Bookmark {
            id: entry_id.to_string(),
            author_id: author_id.to_string(),
            author_name: author,
            full_text: full_text,
            tweet_url: format!("https://twitter.com/{}/status/{}", author_id, entry_id),
        });
    }

    (bookmarks_db, cursor)
}
