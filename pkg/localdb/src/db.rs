use crate::{Author, DBError, HasImage, HasVideo, Image, Item, ItemStatus, Tag};
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

    // tag infos
    pub tag: Option<String>,
    pub tag_id: Option<i32>,
    pub tag_name: Option<String>,

    // author infos
    pub author_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,

    // image infos
    pub img_id: Option<i32>,
    pub img_src: Option<String>,
    pub img_width: Option<i32>,
    pub img_height: Option<i32>,
    pub img_caption: Option<String>,
    pub img_credit: Option<String>,

    // video details
    pub video_id: Option<i32>,
    pub video_src: Option<String>,
    pub video_width: Option<i32>,
    pub video_height: Option<i32>,
    pub video_kind: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
struct RowId {
    pub id: i32,
}

impl LocalDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_image(&mut self, img: &Image) -> crate::Result<i32> {
        let result = sqlx::query(
            "INSERT INTO images (src, width, height, caption, credit) VALUES (?, ?, ?, ?, ?) ON CONFLICT (src) DO NOTHING RETURNING id",
        )
        .bind(&img.src)
        .bind(img.width)
        .bind(img.height)
        .bind(&img.caption)
        .bind(&img.credit)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 0 && result.last_insert_rowid() != 0 {
            return Ok(result.last_insert_rowid() as i32);
        }

        let result: RowId = sqlx::query_as("SELECT id FROM images where src = ?")
            .bind(&img.src)
            .fetch_one(&self.pool)
            .await
            .map_err(DBError::SqlxError)?;
        Ok(result.id)
    }

    pub async fn get_images(&self) -> crate::Result<Vec<Image>> {
        let res: Vec<Image> =
            sqlx::query_as("SELECT id, src, width, height, caption, credit FROM images")
                .fetch_all(&self.pool)
                .await?;
        Ok(res)
    }

    pub async fn link_image(&mut self, image: i32, item: i32) -> crate::Result<()> {
        sqlx::query(
            "INSERT INTO items_images (item_id, image_id) VALUES (?, ?) ON CONFLICT(item_id, image_id) DO NOTHING",
        )
        .bind(item)
        .bind(image)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_video(&mut self, video: &crate::Video) -> crate::Result<i32> {
        let result = sqlx::query(
            "INSERT INTO videos (pocket_id, src, width, height, kind) VALUES (?, ?, ?, ?, ?) ON CONFLICT (src) DO NOTHING RETURNING id",
        )
        .bind(video.id)
        .bind(&video.src)
        .bind(video.width)
        .bind(video.height)
        .bind(&video.kind)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 0 && result.last_insert_rowid() != 0 {
            return Ok(result.last_insert_rowid() as i32);
        }

        let result: RowId = sqlx::query_as("SELECT id FROM videos where src = ?")
            .bind(&video.src)
            .fetch_one(&self.pool)
            .await
            .map_err(DBError::SqlxError)?;
        Ok(result.id)
    }

    pub async fn get_videos(&self) -> crate::Result<Vec<crate::Video>> {
        let res: Vec<crate::Video> =
            sqlx::query_as("SELECT id, src, width, height, kind FROM videos")
                .fetch_all(&self.pool)
                .await?;
        Ok(res)
    }

    pub async fn link_video(&mut self, video: i32, item: i32) -> crate::Result<()> {
        sqlx::query(
            "INSERT INTO items_videos (item_id, video_id) VALUES (?, ?) ON CONFLICT(item_id, video_id) DO NOTHING",
        )
        .bind(item)
        .bind(video)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_author(&mut self, author: &Author) -> crate::Result<i32> {
        let result = sqlx::query(
            "INSERT INTO authors (name, url) VALUES (?, ?) ON CONFLICT (url) DO NOTHING RETURNING id",
        )
        .bind(&author.name)
        .bind(&author.url)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 0 && result.last_insert_rowid() != 0 {
            return Ok(result.last_insert_rowid() as i32);
        }

        let result: RowId = sqlx::query_as("SELECT id FROM authors where url = ?")
            .bind(&author.url)
            .fetch_one(&self.pool)
            .await
            .map_err(DBError::SqlxError)?;

        Ok(result.id)
    }

    pub async fn get_authors(&mut self) -> crate::Result<Vec<Author>> {
        let res: Vec<Author> = sqlx::query_as("SELECT id, name, url FROM authors")
            .fetch_all(&self.pool)
            .await?;
        Ok(res)
    }

    pub async fn link_authors(&mut self, author: i32, item: i32) -> crate::Result<()> {
        sqlx::query(
            "INSERT INTO items_authors (item_id, author_id) VALUES (?, ?) ON CONFLICT(item_id, author_id) DO NOTHING",
        )
        .bind(item)
        .bind(author)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_tag(&mut self, tag: &Tag) -> crate::Result<i32> {
        let result = sqlx::query(
            "INSERT INTO tags (name, tag) VALUES (?, ?) ON CONFLICT (tag) DO NOTHING RETURNING id",
        )
        .bind(&tag.name)
        .bind(tag.tag.to_lowercase())
        .execute(&self.pool)
        .await?;
        if result.rows_affected() != 0 && result.last_insert_rowid() != 0 {
            return Ok(result.last_insert_rowid() as i32);
        }

        let result = self.get_tag(&tag.tag).await?;
        Ok(result.id)
    }

    pub async fn get_tag(&mut self, tag: &str) -> crate::Result<Tag> {
        let res: Tag = sqlx::query_as("SELECT id, tag, name FROM tags where tag = ?")
            .bind(tag)
            .fetch_one(&self.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_tags(&mut self) -> crate::Result<Vec<Tag>> {
        let res: Vec<Tag> = sqlx::query_as("SELECT id, tag, name FROM tags")
            .fetch_all(&self.pool)
            .await?;
        Ok(res)
    }

    pub async fn link_tag(&mut self, tag: i32, item: i32) -> crate::Result<()> {
        sqlx::query(
            "INSERT INTO items_tags (item_id, tag_id) VALUES (?, ?) ON CONFLICT(item_id, tag_id) DO NOTHING",
        )
        .bind(item)
        .bind(tag)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add(&mut self, item: &Item) -> crate::Result<i32> {
        let result = sqlx::query(
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
        .await
        .map_err(DBError::SqlxError)?;

        let item_id = if result.last_insert_rowid() != 0 && result.rows_affected() != 0 {
            result.last_insert_rowid() as i32
        } else {
            let result: RowId = sqlx::query_as("SELECT id FROM items where url = ?")
                .bind(&item.url)
                .fetch_one(&self.pool)
                .await
                .map_err(DBError::SqlxError)?;
            result.id
        };

        for tag in item.tags.iter() {
            let tag_id = self.add_tag(tag).await?;
            self.link_tag(tag_id, item_id).await?;
        }

        for author in item.authors.iter() {
            let author_id = self.add_author(author).await?;
            self.link_authors(author_id, item_id).await?;
        }

        for image in item.images.iter() {
            let image_id = self.add_image(image).await?;
            self.link_image(image_id, item_id).await?;
        }

        for video in item.videos.iter() {
            let video_id = self.add_video(video).await?;
            self.link_video(video_id, item_id).await?;
        }

        Ok(item_id)
    }

    pub async fn get_items(&self) -> crate::Result<Vec<Item>> {
        let rows: Vec<ItemRow> = sqlx::query_as(
            r#"
            select items.*,
                tags.id as tag_id,
                tags.name as tag_name, 
                tags.tag as tag,
                authors.id as author_id,
                authors.url as author_url,
                authors.name as author_name,
                images.id as img_id,
                images.src as img_src,
                images.width as img_width,
                images.height as img_height,
                images.caption as img_caption,
                images.credit as img_credit,
                videos.id as video_id,
                videos.src as video_src,
                videos.width as video_width,
                videos.height as video_height,
                videos.kind as video_kind
            from items 
            left join items_tags 
                on items.id = items_tags.item_id 
            left join tags 
                on items_tags.tag_id = tags.id
            left join items_authors
                on items.id = items_authors.item_id
            left join authors 
                on items_authors.author_id = authors.id
            left join items_images
                on items.id = items_images.item_id
            left join images
                on items_images.image_id = images.id
            left join items_videos
                on items.id = items_videos.item_id
            left join videos
                on items_videos.video_id = videos.id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let rows = rows
            .into_iter()
            .into_grouping_map_by(|row| row.id)
            .fold(Item::default(), |mut item, _, row| {
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

                if let Some(tag_id) = row.tag_id {
                    item.tags.insert(Tag {
                        id: tag_id,
                        tag: row.tag.unwrap_or_default(),
                        name: row.tag_name,
                    });
                }

                if let Some(image_id) = row.img_id {
                    item.images.insert(Image {
                        id: image_id,
                        src: row.img_src.unwrap_or_default(),
                        width: row.img_width.unwrap_or_default(),
                        height: row.img_height.unwrap_or_default(),
                        caption: row.img_caption,
                        credit: row.img_credit,
                    });
                }

                if let Some(video_id) = row.video_id {
                    item.videos.insert(crate::Video {
                        id: video_id,
                        src: row.video_src.unwrap_or_default(),
                        width: row.video_width.unwrap_or_default(),
                        height: row.video_height.unwrap_or_default(),
                        kind: row.video_kind,
                    });
                }

                if let Some(author_id) = row.author_id {
                    item.authors.insert(Author {
                        id: author_id,
                        name: row.author_name.unwrap_or_default(),
                        url: row.author_url,
                    });
                }
                item
            })
            .into_values()
            .collect_vec();

        Ok(rows)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Image, Item, ItemStatus, Tag, Video};
    use std::collections::HashSet;

    async fn get_db() -> LocalDb {
        let pool = open_database(":memory:").await.unwrap();
        LocalDb::new(pool)
    }

    #[tokio::test]
    async fn test_open_in_memory() {
        let con = open_database(":memory:").await;
        assert!(con.is_ok());
    }

    #[tokio::test]
    async fn test_get_items() {
        let db = get_db().await;
        let items = db.get_items().await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_add_item() {
        let mut db = get_db().await;

        let result = db
            .add(&Item {
                tags: HashSet::from([Tag::default()]),
                authors: HashSet::from([Default::default()]),
                images: HashSet::from([Default::default()]),
                videos: HashSet::from([Default::default()]),
                ..Default::default()
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let items = db.get_items().await.unwrap();
        assert_eq!(items.len(), 1);

        let item = &items[0];
        assert_eq!(item.id, 1);
        assert_eq!(item.title, Item::default().title);
        assert_eq!(item.url, Item::default().url);
        assert_eq!(item.status, ItemStatus::Unread);

        assert_eq!(item.tags.len(), 1);
        assert!(item.tags.iter().any(|t| t.tag == Tag::default().tag));

        assert_eq!(item.authors.len(), 1);
        assert!(item
            .authors
            .iter()
            .any(|a| a.name == Author::default().name));

        assert_eq!(item.images.len(), 1);
        assert!(item.images.iter().any(|i| i.src == Image::default().src));

        assert_eq!(item.videos.len(), 1);
        assert!(item.videos.iter().any(|i| i.src == Video::default().src));
    }

    #[tokio::test]
    async fn test_add_image() {
        let mut db = get_db().await;
        let img = Image::default();
        let id = db.add_image(&img).await.unwrap();
        assert_eq!(id, 1);
        let id = db.add_image(&img).await.unwrap();
        assert_eq!(id, 1);
    }

    #[tokio::test]
    async fn test_get_images() {
        let mut db = get_db().await;
        let id = db.add_image(&Default::default()).await.unwrap();
        assert_eq!(id, 1);

        let img = Image {
            src: "https://example.com/image2.jpg".to_string(),
            ..Default::default()
        };
        let id = db.add_image(&img).await.unwrap();
        assert_eq!(id, 2);

        let images = db.get_images().await;
        assert!(images.is_ok());
        let images = images.unwrap();
        assert_eq!(images.len(), 2);
    }

    #[tokio::test]
    async fn test_link_image() {
        let mut db = get_db().await;
        let img_id = db.add_image(&Default::default()).await.unwrap();
        let item_id = db.add(&Default::default()).await.unwrap();
        let res = db.link_image(img_id, item_id).await;
        assert!(res.is_ok());

        let items = db.get_items().await.unwrap();
        assert_eq!(items.len(), 1);

        let item = &items[0];
        assert_eq!(item.images.len(), 1);

        let image = item.images.iter().next().unwrap();
        assert_eq!(image.id, 1);
        assert_eq!(image.src, Image::default().src);
    }

    #[tokio::test]
    async fn test_add_tag() {
        let mut db = get_db().await;
        let result = db.add_tag(&Default::default()).await;
        assert!(result.is_ok());

        let tag = db.get_tag("example").await.unwrap();
        assert_eq!(tag.tag, "example");
        assert_eq!(result.unwrap(), tag.id);
    }

    #[tokio::test]
    async fn test_add_author() {
        let mut db = get_db().await;
        let result = db.add_author(&Default::default()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let authors = db.get_authors().await.unwrap();
        assert_eq!(authors.len(), 1);
        assert_eq!(authors[0].id, 1);
        assert_eq!(authors[0].name, "John Doe");
    }

    #[tokio::test]
    async fn test_add_duplicate_author() {
        let mut db = get_db().await;
        let result = db.add_author(&Default::default()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let result = db.add_author(&Default::default()).await;
        assert!(result.is_ok());
        dbg!(&result);

        let authors = db.get_authors().await.unwrap();
        dbg!(&authors);
        assert_eq!(authors.len(), 1);
        assert_eq!(authors[0].id, 1);
        assert_eq!(authors[0].name, "John Doe");

        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_link_authors() {
        let mut db = get_db().await;
        let author_id = db.add_author(&Default::default()).await.unwrap();
        let item_id = db.add(&Default::default()).await.unwrap();
        let res = db.link_authors(author_id, item_id).await;
        assert!(res.is_ok());

        let items = db.get_items().await.unwrap();
        assert_eq!(items.len(), 1);

        let item = &items[0];
        assert_eq!(item.authors.len(), 1);

        let author = item.authors.iter().next().unwrap();
        assert_eq!(author.id, 1);
        assert_eq!(author.name, "John Doe");
    }

    #[tokio::test]
    async fn test_add_video() {
        let mut db = get_db().await;
        let video = Video::default();
        let id = db.add_video(&video).await.unwrap();
        assert_eq!(id, 1);
        let id = db.add_video(&video).await.unwrap();
        assert_eq!(id, 1);

        let videos = db.get_videos().await;
        assert!(videos.is_ok());
        let videos = videos.unwrap();
        assert_eq!(videos.len(), 1);
        let video = &videos[0];
        assert_eq!(video.id, 1);
        assert_eq!(video.src, Video::default().src);
    }

    #[tokio::test]
    async fn test_link_video() {
        let mut db = get_db().await;
        let video_id = db.add_video(&Default::default()).await.unwrap();
        let item_id = db.add(&Default::default()).await.unwrap();
        let res = db.link_video(video_id, item_id).await;
        assert!(res.is_ok());

        let items = db.get_items().await;
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(items.len(), 1);

        let item = &items[0];
        let videos = &item.videos;
        dbg!(item);
        assert_eq!(videos.len(), 1);
        let video = videos.iter().next().unwrap();
        assert_eq!(video.id, 1);
        assert_eq!(video.src, Video::default().src);
    }

    #[tokio::test]
    async fn test_add_duplicate_tag() {
        let mut db = get_db().await;
        db.add_tag(&Default::default())
            .await
            .expect("add_tag failed");
        db.add_tag(&Default::default())
            .await
            .expect("add tag failed");

        let tags = db.get_tags().await.unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[tokio::test]
    async fn test_link_tag() {
        let mut db = get_db().await;
        let tag_id = db.add_tag(&Tag::default()).await.unwrap();
        let item_id = db.add(&Default::default()).await.unwrap();
        let res = db.link_tag(tag_id, item_id).await;
        assert!(res.is_ok());

        let items = db.get_items().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tags.len(), 1);
        let tag = items[0].tags.clone().into_iter().next().unwrap();
        assert_eq!(tag.id, 1);
        assert_eq!(tag.tag, "example");
    }
}
