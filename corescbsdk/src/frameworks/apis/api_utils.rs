use log::debug;
use reqwest::header::{HeaderValue, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::entities::base::{AccessToken, SCBResponse};
use crate::errors::scb_error::SCBAPIError;

pub const OAUTH_TOKEN_V1_URL: &str = "/v1/oauth/token";
pub const QRCODE_CREATE_V1_URL: &str = "/v1/payment/qrcode/create";
pub const BILL_PAYMENT_TRANSACTION_V1_URL: &str = "/v1/payment/billpayment/transactions";
pub const INQUIRY_BILL_PAYMENT_TRANSACTION_V1_URL: &str = "/v1/payment/billpayment/inquiry";


const BASE_URL: &str = "https://api-sandbox.partners.scb/partners/sandbox";

pub fn api_url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}
pub fn generate_header(
    resource_owner_id: &String,
    access_token: &Option<AccessToken>,
) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    let request_uid = Uuid::new_v4();

    debug!("generate header");

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("EN"));
    headers.insert(USER_AGENT, HeaderValue::from_static("SCB-OpenAPI-SDK/1.0"));
    headers.insert(
        "resourceOwnerId",
        HeaderValue::from_str(resource_owner_id).unwrap(),
    );
    headers.insert(
        "requestUId",
        HeaderValue::from_str(&request_uid.to_string()).unwrap(),
    );
    if let Some(token) = access_token {
        let token = format!("Bearer {}", token.access_token);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    }
    headers
}
pub async fn map_result<T: DeserializeOwned + std::fmt::Debug>(
    response: Result<Response, SCBAPIError>,
) -> Result<T, SCBAPIError> {
    match response {
        Ok(response) => {
            let body = response.json::<SCBResponse<T>>().await;
            match body {
                Ok(body) => {
                    debug!("Response: {:#?}", body);
                    if body.status.code != 1000 {
                        return Err(SCBAPIError::SCBError(body.status.description));
                    }
                    Ok(body.data.unwrap())
                }
                Err(e) => Err(SCBAPIError::SCBError(e.to_string())),
            }
        }
        Err(e) => Err(e),
    }
}
