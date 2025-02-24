pub mod install;

use crate::config::Config;
use localdb::KvDB;
use native_messaging::host::{get_message, send_message};
use pocket::{modify::AddUrlRequest, PocketClient};

#[derive(serde::Deserialize)]
struct Message {
    pub action: String,
    pub url: url::Url,
    pub title: String,
}

#[derive(serde::Serialize)]
pub enum Status {
    Ok,
    Error,
}

#[derive(serde::Serialize)]
struct Result {
    pub status: Status,
    pub message: String,
}

impl Result {
    pub fn ok(message: &str) -> Self {
        Self {
            status: Status::Ok,
            message: message.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: Status::Error,
            message: message.to_string(),
        }
    }
}

pub async fn native_host_handler(config: Config) {
    let pool = localdb::open_database(config.database_dir.to_str().unwrap())
        .await
        .unwrap();
    let kv_db = KvDB::new(pool.clone());
    let access_token = kv_db.get_kv::<String>("pocket_access_token").await.unwrap();
    let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token.value);

    match get_message().await {
        Ok(message) => {
            let msg = serde_json::from_str::<Message>(&message).unwrap();
            match msg.action.as_str() {
                "save" => {
                    let req = AddUrlRequest {
                        url: msg.url,
                        title: Some(msg.title),
                        tags: vec!["readlater".to_string()],
                        tweet_id: None,
                        action: pocket::modify::Action::Add,
                    };
                    pocket.add(vec![req]).await.unwrap();
                }
                _ => send_message(&Result::error("Invalid action"))
                    .await
                    .unwrap(),
            }
            send_message(&Result::ok("URL added to Pocket"))
                .await
                .unwrap();
        }
        Err(e) => eprintln!("Error receiving message: {}", e),
    }
}
