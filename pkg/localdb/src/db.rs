use sqlx::SqlitePool;

use crate::item::{self, HasImage, HasVideo, ItemStatus};

pub async fn open_database(path: &str) -> crate::Result<SqlitePool> {
    let pool = SqlitePool::connect(path).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub struct Database {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ItemRow {
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

    // fields related to the status of the url with respect to the user
    pub status: ItemStatus,
    pub time_added: i64,
    pub time_updated: Option<i64>,
    pub time_read: Option<i64>,
    pub time_favorited: Option<i64>,
    pub tag: Option<String>,
    pub tag_name: Option<String>,
}

impl Database {
    pub fn new(pool: SqlitePool) -> crate::Result<Self> {
        Ok(Self { pool })
    }

    pub async fn add(&mut self, items: &item::Item) -> crate::Result<()> {
        sqlx::query("INSERT INTO items (title, url) VALUES (?, ?)")
            .bind(&items.title)
            .bind(&items.url)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_items(&self) -> crate::Result<Vec<ItemRow>> {
        let rows = sqlx::query_as(
            r#"
            SELECT items.*,
                tags.name as tag_name, tags.tag as tag
            from items 
            left join items_tags 
                on items.id = items_tags.item_id 
            left join tags 
                on items_tags.tag_id = tags.id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        // todo: group by item_id and add tags to vec of tags
        // return item::Item

        Ok(rows)
    }
}

#[cfg(test)]
mod test {
    use crate::item::ItemStatus;

    use super::*;

    #[tokio::test]
    async fn test_open_in_memory() {
        let con = open_database(":memory:").await;
        assert!(con.is_ok());
    }

    #[tokio::test]
    async fn test_get_items() {
        let pool = open_database(":memory:").await.unwrap();
        let db = Database::new(pool).unwrap();
        let items = db.get_items().await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_add_item() {
        let pool = open_database(":memory:").await.unwrap();
        let mut db = Database::new(pool).unwrap();
        let item = Default::default();
        db.add(&item).await.unwrap();
        let items = db.get_items().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, item.title);
        assert_eq!(items[0].url, item.url);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[0].status, ItemStatus::Unread);
    }
}
