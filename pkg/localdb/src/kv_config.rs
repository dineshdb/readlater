use sqlx::SqlitePool;
use std::str::FromStr;

use crate::KvDB;

pub struct KvConfig(KvDB);

const POCKET_ACCESS_TOKEN_KEY: &str = "pocket_access_token";
const POCKET_SINCE: &str = "pocket_since";
const POCKET_OFFSET: &str = "pocket_offset";

impl KvConfig {
    pub fn new(pool: SqlitePool) -> Self {
        Self(KvDB::new(pool))
    }

    pub async fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.0.get_kv::<T>(key).await.ok().map(|kv| kv.value)
    }

    pub async fn set<T: ToString>(&mut self, key: &str, value: T) -> crate::Result<()> {
        self.0
            .set_kv(&(key.to_string(), value.to_string()).into())
            .await
    }

    pub async fn get_pocket_access_token(&self) -> Option<String> {
        self.get(POCKET_ACCESS_TOKEN_KEY).await
    }

    pub async fn set_pocket_access_token(&mut self, token: &str) -> crate::Result<()> {
        self.set(POCKET_ACCESS_TOKEN_KEY, token).await
    }

    pub async fn get_pocket_since(&self) -> Option<i32> {
        self.get(POCKET_SINCE).await
    }

    pub async fn set_pocket_since(&mut self, since: i32) -> crate::Result<()> {
        self.set(POCKET_SINCE, since).await
    }

    pub async fn get_pocket_offset(&self) -> Option<i32> {
        self.get(POCKET_OFFSET).await
    }

    pub async fn set_pocket_offset(&mut self, offset: i32) -> crate::Result<()> {
        self.set(POCKET_OFFSET, offset).await
    }
}
