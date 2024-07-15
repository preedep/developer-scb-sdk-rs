use log::debug;
use reqwest::Client;

use crate::entities::base::{AccessToken};
use crate::entities::qr_code::{QRCodeRequest, QRCodeResponse};
use crate::errors::scb_error::SCBAPIError;
use crate::frameworks::apis::api_utils::{
    api_url, generate_header, map_result, QRCODE_CREATE_V1_URL,
};

pub async fn qr_code_create(
    application_key: &String,
    client: &Client,
    access_token: &AccessToken,
    qrcode_request: &QRCodeRequest,
) -> Result<QRCodeResponse, SCBAPIError> {
    let req = client
        .post(api_url(QRCODE_CREATE_V1_URL))
        .headers(generate_header(
            &application_key,
            &Some(access_token.clone()),
        ))
        .json(qrcode_request)
        .build()
        .expect("Failed to build request");

    debug!("Request : {:#?}", req);
    if let Some(body) = req.body() {
        let bytes = body.as_bytes().unwrap_or(&[]);
        let body_str = String::from_utf8_lossy(bytes);
        debug!("Request Body: {}", body_str);
    }
    let req = client
        .execute(req)
        .await
        .map_err(|e| SCBAPIError::HttpRequestError(e));
    map_result::<QRCodeResponse>(req).await
}
