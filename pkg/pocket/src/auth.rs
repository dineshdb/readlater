use crate::{req, PocketError, PocketResult};
use serde::{Deserialize, Serialize};

const AUTH_REQUEST: &str = "https://getpocket.com/v3/oauth/request";
const AUTH_URL: &str = "https://getpocket.com/v3/oauth/authorize";

#[derive(Serialize)]
pub struct RequestTokenRequest<'a> {
    pub consumer_key: &'a str,
    pub redirect_uri: &'a str,
    /// This string will be returned in all subsequent authentication responses.
    pub state: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct AuthorizeResponse {
    pub code: String,
    pub state: Option<String>,
}

pub struct PocketAuthClient {
    consumer_key: String,
    redirect_uri: String,
    state: Option<String>,
    client: reqwest::Client,
}

impl PocketAuthClient {
    pub fn new(consumer_key: String, redirect_uri: String) -> PocketAuthClient {
        PocketAuthClient {
            consumer_key,
            redirect_uri,
            state: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn login_code(&self) -> PocketResult<String> {
        let request = RequestTokenRequest {
            consumer_key: self.consumer_key.as_str(),
            redirect_uri: self.redirect_uri.as_str(),
            state: self.state.as_deref(),
        };

        let res = req(&self.client, AUTH_REQUEST, &request)
            .await?
            .json::<AuthorizeResponse>()
            .await
            .map_err(|e| PocketError::Reqwest {
                url: AUTH_REQUEST.to_string(),
                source: e,
            })?;
        Ok(res.code)
    }

    pub async fn access_token(&self, code: &str) -> PocketResult<PocketLoginResponse> {
        let request = PocketLoginRequest {
            consumer_key: self.consumer_key.to_string(),
            code: code.to_string(),
        };

        let res = req(&self.client, AUTH_URL, &request)
            .await?
            .json::<PocketLoginResponse>()
            .await
            .map_err(|e| PocketError::Reqwest {
                url: AUTH_URL.to_string(),
                source: e,
            })?;
        Ok(res)
    }
}

#[derive(Serialize, Debug)]
pub struct PocketLoginRequest {
    pub consumer_key: String,
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct PocketLoginResponse {
    pub access_token: String,
    pub username: String,
}

pub fn redirection_uri(request_token: &str, redirect_uri: &str) -> String {
    format!(
        "https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
        request_token, redirect_uri
    )
}
