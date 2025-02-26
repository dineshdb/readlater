use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone, PartialEq, Eq, Hash)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
}

impl From<pocket::item::Authors> for Author {
    fn from(author: pocket::item::Authors) -> Self {
        Self {
            id: author.author_id,
            name: author.name,
            url: author.url,
        }
    }
}

impl Default for Author {
    fn default() -> Self {
        Author {
            id: 0,
            name: "John Doe".to_string(),
            url: Some("https://example.com/John-Doe".to_string()),
        }
    }
}
