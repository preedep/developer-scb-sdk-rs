use log::debug;
use reqwest::header::{HeaderValue, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use uuid::Uuid;

use crate::entities::base::AccessToken;

pub const OAUTH_TOKEN_V1_URL: &str = "/v1/oauth/token";
pub const QRCODE_CREATE_V1_URL: &str = "/v1/payment/qrcode/create";
pub const BILL_PAYMENT_TRANSACTION_V1_URL: &str = "/v1/payment/billpayment/transactions/";

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
