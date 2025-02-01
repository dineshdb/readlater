pub mod install;

use native_messaging::host::{get_message, send_message};
use pocket::{modify::AddUrlRequest, PocketClient};

use crate::config::Config;

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
    let mut pocket = PocketClient::new(&config.consumer_key, &config.access_token);

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
