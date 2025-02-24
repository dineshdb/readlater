use crate::{PocketError, PocketResult, X_ACCEPT, X_ERROR, X_ERROR_CODE};
use reqwest::{
    header::{self, HeaderValue},
    Client, Response,
};
use serde::Serialize;

pub async fn req<Req: Serialize>(
    client: &Client,
    url: &str,
    request: &Req,
) -> PocketResult<Response> {
    let request = serde_json::to_string(request).map_err(PocketError::SerdeJson)?;
    let app_json = "application/json";

    let res = client
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
        let code = code
            .to_str()
            .expect("X-Error-Code is malformed")
            .parse()
            .expect("X-Error-Code is malformed integer");

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
