use sqlx::SqlitePool;
use std::str::FromStr;

pub struct KvDB {
    pool: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct KeyValue<T> {
    pub key: String,
    pub value: T,
}

impl<T> KeyValue<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value }
    }
}

impl From<(String, String)> for KeyValue<String> {
    fn from((key, value): (String, String)) -> Self {
        Self { key, value }
    }
}

impl From<(&str, &str)> for KeyValue<String> {
    fn from((key, value): (&str, &str)) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<(&str, i32)> for KeyValue<String> {
    fn from((key, value): (&str, i32)) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl KvDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_kv<T: FromStr>(&self, key: &str) -> crate::Result<KeyValue<T>> {
        let row: KeyValue<String> = sqlx::query_as("select key, value from kv where key = ?")
            .bind(key)
            .fetch_one(&self.pool)
            .await?;
        let value = T::from_str(&row.value).map_err(|_| crate::DBError::ParseError)?;
        Ok(KeyValue {
            key: row.key,
            value,
        })
    }

    pub async fn set_kv<T: ToString>(&mut self, kv: &KeyValue<T>) -> crate::Result<()> {
        sqlx::query(
            "insert into kv (key, value, updated_at) 
            values (?, ?, CURRENT_TIMESTAMP) 
            on conflict(key) do update set 
            value = excluded.value",
        )
        .bind(&kv.key)
        .bind(kv.value.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::open_database;

    use super::*;

    #[tokio::test]
    async fn test_open_in_memory() {
        let con = open_database(":memory:").await;
        assert!(con.is_ok());
    }

    #[tokio::test]
    async fn test_get_kv() {
        let pool = open_database(":memory:").await.unwrap();
        let db = KvDB::new(pool);
        let kv = db.get_kv::<String>("key").await;
        assert!(kv.is_err());
    }

    #[tokio::test]
    async fn test_set_kv() {
        let pool = open_database(":memory:").await.unwrap();
        let mut db = KvDB::new(pool);
        let kv = db.set_kv(&("key", "value").into()).await;
        assert!(kv.is_ok());
        let kv = db.get_kv::<String>("key").await;
        assert!(kv.is_ok());
        assert_eq!(kv.unwrap().value, "value");
    }
}
