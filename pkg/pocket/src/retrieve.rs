use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use util::der::bool_from_number;
use util::ser::serialize_option_bool_as_int;

use super::item::Item;

#[derive(Serialize)]
pub enum State {
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "all")]
    All,
}

#[derive(Serialize)]
pub enum DetailType {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Serialize)]
pub enum Tag {
    #[serde(rename = "_untagged_")]
    Untagged,
    Value(String),
}

#[derive(Serialize)]
pub enum ContentType {
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "image")]
    Image,
}

#[derive(Serialize)]
pub enum SortBy {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "site")]
    Site,
}

#[derive(Deserialize)]
pub struct RetrieveResponse {
    pub status: i32,
    pub error: Option<String>,
    #[serde(deserialize_with = "bool_from_number")]
    pub complete: bool,
    pub since: i32,
    pub list: HashMap<String, Item>,
}

#[derive(Serialize)]
pub struct GetOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_bool_as_int")]
    pub favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<DetailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    pub count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl Default for GetOptions {
    fn default() -> GetOptions {
        GetOptions {
            state: Some(State::Unread),
            favorite: None,
            tag: None,
            content_type: None,
            sort: None,
            detail_type: None,
            search: None,
            domain: None,
            since: None,
            count: 10,
            offset: None,
        }
    }
}
