use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone, PartialEq, Eq, Hash)]
pub struct Video {
    pub id: i32,
    pub src: String,
    pub width: i32,
    pub height: i32,
    pub kind: Option<String>,
}

impl From<pocket::item::Videos> for Video {
    fn from(image: pocket::item::Videos) -> Self {
        Self {
            id: image.video_id,
            src: image.src,
            width: image.width,
            height: image.height,
            kind: Some(image.kind),
        }
    }
}

impl Default for Video {
    fn default() -> Self {
        Self {
            id: 0,
            src: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
            width: 0,
            height: 0,
            kind: Some("".to_string()),
        }
    }
}
