use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use util::der::bool_from_string;
use util::der::i32_from_string;
use util::der::u64_from_string;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub item_id: String,
    pub resolved_id: String,
    pub given_url: String,
    pub given_title: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub favorite: bool,
    pub status: ItemStatus,
    #[serde(deserialize_with = "i32_from_string")]
    pub time_added: i32,
    #[serde(deserialize_with = "i32_from_string")]
    pub time_updated: i32,
    #[serde(deserialize_with = "i32_from_string")]
    pub time_read: i32,
    #[serde(deserialize_with = "i32_from_string")]
    pub time_favorited: i32,
    pub sort_id: i32,
    pub resolved_title: String,
    pub resolved_url: String,
    pub excerpt: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub is_article: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub is_index: bool,
    pub has_video: HasVideo,
    pub has_image: HasImage,
    #[serde(deserialize_with = "i32_from_string")]
    pub word_count: i32,
    pub lang: String,
    pub time_to_read: i32,
    pub top_image_url: Option<String>,
    pub listen_duration_estimate: i32,
    pub tags: HashMap<String, Tag>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tag {
    pub tag: String,
    #[serde(deserialize_with = "u64_from_string")]
    pub item_id: u64,
}

#[derive(Deserialize, Debug)]
#[repr(i32)]
pub enum ItemStatus {
    #[serde(rename = "0")]
    Unread = 0,
    #[serde(rename = "1")]
    Archived = 1,
    #[serde(rename = "2")]
    Deleted = 2,
}

#[derive(Deserialize, Debug)]
#[repr(i32)]
pub enum HasVideo {
    #[serde(rename = "0")]
    No = 0,
    #[serde(rename = "1")]
    Yes = 1,
    #[serde(rename = "2")]
    IsVideo = 2,
}

#[derive(Deserialize, Debug)]
#[repr(i32)]
pub enum HasImage {
    #[serde(rename = "0")]
    No = 0,
    #[serde(rename = "1")]
    Yes = 1,
    #[serde(rename = "2")]
    IsImage = 2,
}
