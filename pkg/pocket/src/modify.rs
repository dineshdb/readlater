use serde::Serialize;
use std::vec;
use url::Url;
use util::ser::serialize_vec_as_comma_separated;

#[derive(Serialize)]
pub struct PocketSendRequest<'a, T> {
    consumer_key: &'a str,
    access_token: &'a str,
    actions: Vec<T>,
}

impl<'a, T> PocketSendRequest<'a, T> {
    pub fn new(
        consumer_key: &'a str,
        access_token: &'a str,
        actions: Vec<T>,
    ) -> PocketSendRequest<'a, T> {
        PocketSendRequest {
            consumer_key,
            access_token,
            actions,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Add,
    Archive,
    Readd,
    Favorite,
    Unfavorite,
    Delete,
    TagsAdd,
    TagsRemove,
    TagsReplace,
    TagsClear,
    TagsRename,
    TagsDelete,
}

#[derive(Serialize)]
pub struct ModifyItem {
    action: Action,
    item_id: u64,
}

impl ModifyItem {
    pub fn new(action: Action, item_id: u64) -> ModifyItem {
        ModifyItem { action, item_id }
    }

    pub fn archive(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::Archive, item_id)
    }

    pub fn readd(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::Readd, item_id)
    }

    pub fn favorite(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::Favorite, item_id)
    }

    pub fn unfavorite(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::Unfavorite, item_id)
    }

    pub fn delete(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::Delete, item_id)
    }

    pub fn clear_tags(item_id: u64) -> ModifyItem {
        ModifyItem::new(Action::TagsClear, item_id)
    }
}

#[derive(Serialize)]
pub struct AddUrlRequest {
    action: Action,
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(serialize_with = "serialize_vec_as_comma_separated")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tweet_id: Option<String>,
}

impl AddUrlRequest {
    pub fn new(url: Url) -> AddUrlRequest {
        AddUrlRequest {
            action: Action::Add,
            url,
            title: None,
            tags: vec![],
            tweet_id: None,
        }
    }
}

#[derive(Serialize)]
pub struct TagRename {
    action: Action,
    old_tag: String,
    new_tag: String,
}

impl TagRename {
    pub fn new(old_tag: String, new_tag: String) -> TagRename {
        TagRename {
            action: Action::TagsRename,
            old_tag,
            new_tag,
        }
    }
}

#[derive(Serialize)]
pub struct TagDelete {
    action: Action,
    tag: String,
}

impl TagDelete {
    pub fn new(tag: String) -> TagDelete {
        TagDelete {
            action: Action::TagsDelete,
            tag,
        }
    }
}

#[derive(Serialize)]
pub struct ItemTagAction {
    action: Action,
    item_id: u64,
    #[serde(serialize_with = "serialize_vec_as_comma_separated")]
    tags: Vec<String>,
}

impl ItemTagAction {
    pub fn add_tags(item_id: u64, tags: Vec<String>) -> ItemTagAction {
        ItemTagAction {
            action: Action::TagsReplace,
            item_id,
            tags,
        }
    }

    pub fn replace_tags(item_id: u64, tags: Vec<String>) -> ItemTagAction {
        ItemTagAction {
            action: Action::TagsReplace,
            item_id,
            tags,
        }
    }

    pub fn remove_tags(item_id: u64, tags: Vec<String>) -> ItemTagAction {
        ItemTagAction {
            action: Action::TagsReplace,
            item_id,
            tags,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn archive_item() {
        insta::assert_json_snapshot!(ModifyItem::archive(123456789));
    }

    #[test]
    fn readd_item() {
        insta::assert_json_snapshot!(ModifyItem::readd(123456789));
    }

    #[test]
    fn favorite_item() {
        insta::assert_json_snapshot!(ModifyItem::favorite(123456789));
    }

    #[test]
    fn unfavorite_item() {
        insta::assert_json_snapshot!(ModifyItem::unfavorite(123456789));
    }

    #[test]
    fn delete_item() {
        insta::assert_json_snapshot!(ModifyItem::delete(123456789));
    }

    #[test]
    fn add_tags() {
        insta::assert_json_snapshot!(ItemTagAction::add_tags(
            123456789,
            vec!["tag1".to_string(), "tag2".to_string()]
        ));
    }

    #[test]
    fn clear_tags() {
        insta::assert_json_snapshot!(ModifyItem::clear_tags(123456789));
    }

    #[test]
    fn test_add_url_request() {
        use url::Url;

        let url = Url::parse("https://example.com").unwrap();
        insta::assert_json_snapshot!(AddUrlRequest::new(url));
    }

    #[test]
    fn test_tag_rename() {
        insta::assert_json_snapshot!(TagRename::new("old_tag".to_string(), "new_tag".to_string()));
    }

    #[test]
    fn test_tag_delete() {
        insta::assert_json_snapshot!(TagDelete::new("tag".to_string()));
    }
}
