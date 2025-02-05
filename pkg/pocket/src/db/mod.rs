use sqlx::SqlitePool;

pub async fn open_database(path: &str) -> crate::PocketResult<SqlitePool> {
    let pool = SqlitePool::connect(path).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> crate::PocketResult<Self> {
        Ok(Self { pool })
    }

    pub fn add(&self, items: &[crate::item::Item]) -> crate::PocketResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_open_in_memory() {
        let con = open_database(":memory:").await;
        assert!(con.is_ok());
    }
}
