use crate::item::{self, HasImage, HasVideo, ItemStatus};
use itertools::Itertools;
use sqlx::SqlitePool;

pub async fn open_database(path: &str) -> crate::Result<SqlitePool> {
    let pool = SqlitePool::connect(path).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub struct LocalDb {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
struct ItemRow {
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
    pub time_added: i32,
    pub time_updated: Option<i32>,
    pub time_read: Option<i32>,
    pub time_favorited: Option<i32>,
    pub tag_id: Option<i32>,
    pub tag: Option<String>,
    pub tag_name: Option<String>,
}

impl LocalDb {
    pub fn new(pool: SqlitePool) -> crate::Result<Self> {
        Ok(Self { pool })
    }

    pub async fn add(&mut self, item: &item::Item) -> crate::Result<()> {
        sqlx::query(
            "INSERT INTO items (
            pocket_id, 
            title, 
            url, 
            excerpt, 
            is_article, 
            is_index, 
            has_video, 
            word_count, 
            lang, 
            listen_duration_estimate,
            time_to_read,
            top_image_url,
            status,
            time_added,            
            time_updated,
            time_read,
            time_favorited
        ) VALUES (
            ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?
        ) ON CONFLICT(pocket_id) DO UPDATE SET
            status = excluded.status,
            time_added = excluded.time_added,
            time_updated = excluded.time_updated,
            time_read = excluded.time_read,
            time_favorited = excluded.time_favorited
         ",
        )
        .bind(item.pocket_id)
        .bind(&item.title)
        .bind(&item.url)
        .bind(&item.excerpt)
        .bind(item.is_article)
        .bind(item.is_index)
        .bind(item.has_video)
        .bind(item.word_count)
        .bind(&item.lang)
        .bind(item.listen_duration_estimate)
        .bind(item.time_to_read)
        .bind(&item.top_image_url)
        .bind(item.status)
        .bind(item.time_added)
        .bind(item.time_updated)
        .bind(item.time_read)
        .bind(item.time_favorited)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_items(&self) -> crate::Result<Vec<item::Item>> {
        let rows: Vec<ItemRow> = sqlx::query_as(
            r#"
            select items.*,
                tags.id as tag_id,
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

        let rows = rows
            .into_iter()
            .into_grouping_map_by(|row| row.id)
            .fold(item::Item::default(), |mut item, _, row| {
                item.title = row.title;
                item.url = row.url;
                item.id = row.id;
                item.excerpt = row.excerpt;
                item.is_article = row.is_article;
                item.is_index = row.is_index;
                item.has_video = row.has_video;
                item.has_image = row.has_image;
                item.word_count = row.word_count;
                item.lang = row.lang;
                item.time_to_read = row.time_to_read;
                item.top_image_url = row.top_image_url;
                item.listen_duration_estimate = row.listen_duration_estimate;
                item.time_added = row.time_added;
                item.time_updated = row.time_updated;
                item.time_read = row.time_read;
                item.time_favorited = row.time_favorited;

                item.status = row.status;
                item.tags.insert(item::Tag {
                    id: row.tag_id.unwrap_or_default(),
                    tag: row.tag.unwrap_or_default(),
                    name: row.tag_name,
                });
                item
            })
            .into_values()
            .collect_vec();

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
        let db = LocalDb::new(pool).unwrap();
        let items = db.get_items().await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_add_item() {
        let pool = open_database(":memory:").await.unwrap();
        let mut db = LocalDb::new(pool).unwrap();
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
