use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub full_text: String,
    pub tweet_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub auth_bearer: String,
    pub auth_token: String,
    pub csrf: String,
    pub url: String,
}
