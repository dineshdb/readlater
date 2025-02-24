pub mod auth;
mod error;
pub mod item;
pub mod modify;
mod req;
pub mod retrieve;

pub use error::{PocketError, PocketResult};
pub use item::Item;
use modify::{Action, AddUrlRequest, ModifyItem, PocketSendRequest};
pub use req::req;
use reqwest::Client;
pub use retrieve::*;
use serde::Serialize;

const X_ACCEPT: &str = "X-Accept";
const X_ERROR: &str = "X-Error";
const X_ERROR_CODE: &str = "X-Error-Code";
const GET_URL: &str = "https://getpocket.com/v3/get";
const SEND_URL: &str = "https://getpocket.com/v3/send";

pub struct PocketClient<'a> {
    consumer_key: &'a str,
    access_token: &'a str,
    client: Client,
}

#[derive(Serialize)]
pub struct PockeRequest<'a, T> {
    consumer_key: &'a str,
    access_token: &'a str,
    #[serde(flatten)]
    extra: T,
}

impl<'a, T> PockeRequest<'a, T> {
    pub fn new(consumer_key: &'a str, access_token: &'a str, data: T) -> PockeRequest<'a, T> {
        PockeRequest {
            consumer_key,
            access_token,
            extra: data,
        }
    }
}

impl<'a> PocketClient<'a> {
    pub fn new(consumer_key: &'a str, access_token: &'a str) -> PocketClient<'a> {
        PocketClient {
            consumer_key,
            access_token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&mut self, request: GetOptions) -> PocketResult<RetrieveResponse> {
        let request = PockeRequest::new(self.consumer_key, self.access_token, request);
        req(&self.client, GET_URL, &request)
            .await?
            .json::<RetrieveResponse>()
            .await
            .map_err(|e| PocketError::Reqwest {
                url: GET_URL.to_string(),
                source: e,
            })
    }

    pub async fn add(&mut self, request: Vec<AddUrlRequest>) -> PocketResult<()> {
        let request = PocketSendRequest::new(self.consumer_key, self.access_token, request);
        let _res = req(&self.client, SEND_URL, &request).await?;
        Ok(())
    }

    pub async fn archive(&mut self, request: Vec<u64>) -> PocketResult<()> {
        let actions: Vec<_> = request
            .into_iter()
            .map(|item_id| ModifyItem::new(Action::Archive, item_id))
            .collect();
        let request = PocketSendRequest::new(self.consumer_key, self.access_token, actions);
        let _res = req(&self.client, SEND_URL, &request).await?;
        Ok(())
    }
}
