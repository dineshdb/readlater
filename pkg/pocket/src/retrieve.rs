use super::item::Item;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use util::der::bool_from_number;
use util::der::opt_from_string;
use util::ser::ser_opt_as_str;
use util::ser::serialize_option_bool_as_int;

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
#[serde(rename_all = "lowercase")]
pub enum DetailType {
    Simple,
    Complete,
}

#[derive(Serialize)]
pub enum Tag {
    #[serde(rename = "_untagged_")]
    Untagged,
    #[serde(untagged)]
    Value(String),
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Article,
    Video,
    Image,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Newest,
    Oldest,
    Title,
    Site,
}

#[derive(Deserialize, Debug)]
pub struct RetrieveResponse {
    pub status: i32,
    pub error: Option<String>,
    #[serde(deserialize_with = "bool_from_number")]
    pub complete: bool,
    pub since: i32,
    pub offset: Option<i32>,
    pub count: Option<i32>,
    #[serde(deserialize_with = "opt_from_string")]
    pub total: Option<i32>,
    pub list: BTreeMap<String, Item>,
}

impl RetrieveResponse {
    pub fn has_more(&self) -> crate::PocketResult<bool> {
        if let Some(total) = self.total {
            let count = self.count.unwrap_or(self.list.len() as i32);
            let offset = self.offset.unwrap_or(0);
            return Ok(total > count + offset);
        }

        Err(crate::PocketError::InvalidPagintionRequest)
    }
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
    #[serde(rename = "detailType")]
    pub detail_type: Option<DetailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "ser_opt_as_str")]
    pub since: Option<i32>,
    pub count: i32,
    pub total: IncludeTotal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Serialize)]
pub enum IncludeTotal {
    #[serde(rename = "1")]
    Include,
    #[serde(rename = "0")]
    Exclude,
}

impl Default for GetOptions {
    fn default() -> Self {
        Self {
            state: Some(State::Unread),
            favorite: None,
            tag: None,
            content_type: None,
            sort: Some(SortBy::Oldest),
            detail_type: Some(DetailType::Simple),
            search: None,
            domain: None,
            total: IncludeTotal::Include,
            since: None,
            // 30 is the maximum value permitted
            count: 30,
            offset: None,
        }
    }
}

impl GetOptions {
    pub fn for_pagination() -> Self {
        Self {
            total: IncludeTotal::Include,
            ..Default::default()
        }
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.count = count;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(&mut self) -> GetOptions {
        let mut options = GetOptions::default();
        std::mem::swap(self, &mut options);
        options
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_options_default() {
        let options = GetOptions::default();
        insta::assert_json_snapshot!(options);
    }

    #[test]
    fn test_get_options() {
        let options = GetOptions {
            state: Some(State::Archive),
            favorite: Some(true),
            tag: Some(Tag::Value("linux".to_string())),
            content_type: Some(ContentType::Article),
            sort: Some(SortBy::Title),
            detail_type: Some(DetailType::Complete),
            search: Some("example".to_string()),
            domain: Some("example.org".to_string()),
            since: Some(1738297033),
            count: 10,
            total: IncludeTotal::Include,
            offset: Some(10),
        };
        insta::assert_json_snapshot!(options);
    }
}
