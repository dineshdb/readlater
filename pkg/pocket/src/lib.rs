mod db;
mod error;
mod item;
pub mod modify;
mod retrieve;

pub use error::{PocketError, PocketResult};
use modify::{Action, AddUrlRequest, ModifyItem, PocketSendRequest};
use reqwest::{
    header::{self, HeaderValue},
    Client, Response,
};
use retrieve::{GetOptions, RetrieveResponse};
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

    async fn request<Req: Serialize>(&self, url: &str, request: &Req) -> PocketResult<Response> {
        let request = serde_json::to_string(request).map_err(PocketError::SerdeJson)?;
        let app_json = "application/json";

        let res = self
            .client
            .post(url)
            .header(X_ACCEPT, HeaderValue::from_static(app_json))
            .header(header::CONTENT_TYPE, HeaderValue::from_static(app_json))
            .body(request)
            .send()
            .await
            .map_err(|e| PocketError::Reqwest {
                url: url.to_string(),
                source: e,
            })?;

        if let Some(code) = res.headers().get(X_ERROR_CODE) {
            let code = code.to_str().expect("X-Error-Code is malformed").into();
            return Err(PocketError::Proto(
                code,
                res.headers()
                    .get(X_ERROR)
                    .map(|v| v.to_str().expect("X-Code is malformed").into())
                    .unwrap_or("unknown protocol".into()),
                res.text().await.ok(),
            ));
        }
        res.error_for_status()
            .map_err(|http_error| PocketError::Http(http_error, None))
    }

    pub async fn get(&mut self, req: GetOptions) -> PocketResult<RetrieveResponse> {
        let request = PockeRequest::new(self.consumer_key, self.access_token, req);
        self.request(GET_URL, &request)
            .await?
            .json::<RetrieveResponse>()
            .await
            .map_err(|e| PocketError::Reqwest {
                url: GET_URL.to_string(),
                source: e,
            })
    }

    pub async fn add(&mut self, req: Vec<AddUrlRequest>) -> PocketResult<()> {
        let request = PocketSendRequest::new(self.consumer_key, self.access_token, req);
        let _res = self.request(SEND_URL, &request).await?;
        Ok(())
    }

    pub async fn archive(&mut self, req: Vec<u64>) -> PocketResult<()> {
        let actions: Vec<_> = req
            .into_iter()
            .map(|item_id| ModifyItem::new(Action::Archive, item_id))
            .collect();
        let request = PocketSendRequest::new(self.consumer_key, self.access_token, actions);
        let _res = self.request(SEND_URL, &request).await?;
        Ok(())
    }
}
