#[derive(Debug, thiserror::Error)]
pub enum PocketError {
    #[error("HTTP error: {0}")]
    Http(reqwest::Error, Option<String>),
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJson(serde_json::Error),
    #[error("Request error for URL <{url}>: {source}")]
    Reqwest { url: String, source: reqwest::Error },
    #[error("Pocket protocol error: {1} ({0})")]
    Proto(String, String, Option<String>),
    #[error("X-Error-Code is malformed UTF-8")]
    ReqwwestStrError(#[from] reqwest::header::ToStrError),

    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error while migrating database: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("An unknown error occured")]
    Unknown,
}

pub type PocketResult<T> = Result<T, PocketError>;
