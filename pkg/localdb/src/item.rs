use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub excerpt: Option<String>,

    /// Extended metadata about the url
    pub is_article: Option<bool>,
    pub is_index: Option<bool>,
    pub has_video: Option<HasVideo>,
    pub has_image: Option<HasImage>,
    pub word_count: Option<i32>,
    pub lang: Option<String>,
    pub time_to_read: Option<i32>,
    pub top_image_url: Option<String>,
    pub listen_duration_estimate: Option<i32>,
    pub tags: HashSet<Tag>,

    // fields related to the status of the url with respect to the user
    pub status: ItemStatus,
    pub time_added: i64,
    pub time_updated: Option<i64>,
    pub time_read: Option<i64>,
    pub time_favorited: Option<i64>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            id: 0,
            title: "Example URL".to_string(),
            url: "http://example.com".to_string(),
            excerpt: None,
            status: ItemStatus::Unread,
            time_added: 0,
            time_updated: None,
            time_read: None,
            time_favorited: None,
            tags: HashSet::new(),

            is_article: None,
            is_index: None,
            has_video: None,
            has_image: None,
            word_count: None,
            lang: None,
            time_to_read: None,
            top_image_url: None,
            listen_duration_estimate: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub id: i64,
    pub tag: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, sqlx::Type, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum ItemStatus {
    Unread = 0,
    Archived = 1,
    Deleted = 2,
}

impl From<pocket::item::ItemStatus> for ItemStatus {
    fn from(status: pocket::item::ItemStatus) -> Self {
        match status {
            pocket::item::ItemStatus::Unread => ItemStatus::Unread,
            pocket::item::ItemStatus::Archived => ItemStatus::Archived,
            pocket::item::ItemStatus::Deleted => ItemStatus::Deleted,
        }
    }
}

#[derive(Deserialize, Debug, Clone, sqlx::Type, Copy)]
#[repr(i32)]
pub enum HasVideo {
    No = 0,
    Yes = 1,
    IsVideo = 2,
}

impl From<pocket::item::HasVideo> for HasVideo {
    fn from(has_video: pocket::item::HasVideo) -> Self {
        match has_video {
            pocket::item::HasVideo::No => HasVideo::No,
            pocket::item::HasVideo::Yes => HasVideo::Yes,
            pocket::item::HasVideo::IsVideo => HasVideo::IsVideo,
        }
    }
}

#[derive(Deserialize, Debug, Clone, sqlx::Type, Copy)]
#[repr(i32)]
pub enum HasImage {
    No = 0,
    Yes = 1,
    IsImage = 2,
}

impl From<pocket::item::HasImage> for HasImage {
    fn from(has_image: pocket::item::HasImage) -> Self {
        match has_image {
            pocket::item::HasImage::No => HasImage::No,
            pocket::item::HasImage::Yes => HasImage::Yes,
            pocket::item::HasImage::IsImage => HasImage::IsImage,
        }
    }
}
