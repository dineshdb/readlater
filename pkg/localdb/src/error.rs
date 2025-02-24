#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("file error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error while migrating database: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;
