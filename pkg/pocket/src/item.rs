use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::Deref;
use util::der::from_string;

#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(deserialize_with = "from_string")]
    pub item_id: i64,
    #[serde(deserialize_with = "from_string")]
    pub resolved_id: i64,
    pub given_url: String,
    pub given_title: String,
    pub favorite: Boolean,
    pub status: ItemStatus,
    #[serde(deserialize_with = "from_string")]
    pub time_added: i32,
    #[serde(deserialize_with = "from_string")]
    pub time_updated: i32,
    #[serde(deserialize_with = "from_string")]
    pub time_read: i32,
    #[serde(deserialize_with = "from_string")]
    pub time_favorited: i32,
    pub sort_id: i32,
    pub resolved_title: String,
    pub resolved_url: String,
    pub excerpt: String,
    pub is_article: Boolean,
    pub is_index: Boolean,
    pub has_video: HasVideo,
    pub has_image: HasImage,
    #[serde(deserialize_with = "from_string")]
    pub word_count: i32,
    pub lang: String,
    pub time_to_read: i32,
    pub top_image_url: Option<String>,
    pub listen_duration_estimate: i32,
    pub tags: HashMap<String, Tag>,
    pub authors: Option<HashMap<String, Authors>>,
    pub image: Option<Image>,
    pub images: Option<HashMap<String, Images>>,
    pub videos: Option<HashMap<String, Videos>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Authors {
    #[serde(deserialize_with = "from_string")]
    pub author_id: i32,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Videos {
    #[serde(deserialize_with = "from_string")]
    pub video_id: i32,
    pub src: String,
    #[serde(deserialize_with = "from_string")]
    pub width: i32,
    #[serde(deserialize_with = "from_string")]
    pub height: i32,
    #[serde(rename = "type")]
    pub kind: String,
    pub vid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Images {
    #[serde(deserialize_with = "from_string")]
    pub image_id: i32,
    pub src: String,
    #[serde(deserialize_with = "from_string")]
    pub width: i32,
    #[serde(deserialize_with = "from_string")]
    pub height: i32,
    pub credit: String,
    pub caption: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    #[serde(deserialize_with = "from_string")]
    pub width: i32,
    #[serde(deserialize_with = "from_string")]
    pub height: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tag {
    pub tag: String,
    #[serde(deserialize_with = "from_string")]
    pub item_id: u64,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[repr(i32)]
pub enum ItemStatus {
    #[serde(rename = "0")]
    Unread = 0,
    #[serde(rename = "1")]
    Archived = 1,
    #[serde(rename = "2")]
    Deleted = 2,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[repr(i32)]
pub enum Boolean {
    #[serde(rename = "0")]
    #[serde(alias = "false")]
    No = 0,
    #[serde(rename = "1")]
    #[serde(alias = "true")]
    Yes = 1,
}

impl Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Boolean::No => &false,
            Boolean::Yes => &true,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[repr(i32)]
pub enum HasVideo {
    #[serde(rename = "0")]
    No = 0,
    #[serde(rename = "1")]
    Yes = 1,
    #[serde(rename = "2")]
    IsVideo = 2,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[repr(i32)]
pub enum HasImage {
    #[serde(rename = "0")]
    No = 0,
    #[serde(rename = "1")]
    Yes = 1,
    #[serde(rename = "2")]
    IsImage = 2,
}
