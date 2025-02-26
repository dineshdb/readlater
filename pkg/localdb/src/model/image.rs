use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    pub id: i32,
    pub src: String,
    pub width: i32,
    pub height: i32,
    pub caption: Option<String>,
    pub credit: Option<String>,
}

impl From<pocket::item::Images> for Image {
    fn from(image: pocket::item::Images) -> Self {
        Image {
            id: image.image_id,
            src: image.src,
            width: image.width,
            height: image.height,
            caption: Some(image.caption),
            credit: Some(image.credit),
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Image {
            id: 0,
            src: "https://s3.amazonaws.com/pocket-curatedcorpusapi-prod-images/bb16bdaf-f3e5-47fc-8bef-14ccece27fd7.jpeg".to_string(),
            width: 0,
            height: 0,
            caption: Some("".to_string()),
            credit: Some("".to_string()),
        }
    }
}
