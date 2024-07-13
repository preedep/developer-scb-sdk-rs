use log::{debug, error, info};
use reqwest::header::{ACCEPT_LANGUAGE, CONTENT_TYPE, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::base::{AccessToken, SCBAccessTokenRequest, SCBResponse};
use crate::entities::qrcode::{QRCodeRequest, QRCodeResponse};
use crate::errors::scb_error::SCBAPIError;

const SANDBOX_OAUTH_TOKEN_V1_URL: &str =
    "https://api-sandbox.partners.scb/partners/sandbox/v1/oauth/token";
const SANDBOX_QRCODE_CREATE_V1_URL: &str =
    "https://api-sandbox.partners.scb/partners/sandbox/v1/payment/qrcode/create";

#[derive(Debug, Serialize, Deserialize)]
pub struct SCBClientAPI {
    application_name: String,
    application_key: String,
    secret_key: String,
    access_token: Option<AccessToken>,
}

fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}
fn generate_header(resource_owner_id: &String) -> reqwest::header::HeaderMap {
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

    headers
}
impl SCBClientAPI {
    pub fn new(
        application_name: &String,
        application_key: &String,
        secret_key: &String,
    ) -> SCBClientAPI {
        SCBClientAPI {
            application_name: application_name.to_string(),
            application_key: application_key.to_string(),
            secret_key: secret_key.to_string(),
            access_token: None,
        }
    }
    async fn authentication(&mut self) -> Result<(), SCBAPIError> {
        let request = SCBAccessTokenRequest {
            application_key: self.application_key.to_string(),
            application_secret: self.secret_key.to_string(),
            auth_code: None,
            state: None,
            code_challenge: None,
        };

        let req = create_client()
            .post(SANDBOX_OAUTH_TOKEN_V1_URL)
            .headers(generate_header(&self.application_name))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await;

        match req {
            Ok(response) => {
                let body = response.json::<SCBResponse<AccessToken>>().await;
                match body {
                    Ok(body) => {
                        if body.status.code != 1000 {
                            return Err(SCBAPIError::SCBError(body.status.description));
                        }
                        self.access_token = Some(body.data.unwrap());
                        Ok(())
                    }
                    Err(e) => {
                        return Err(SCBAPIError::SCBError(e.to_string()));
                    }
                }
            }
            Err(e) => Err(SCBAPIError::HttpRequestError(e)),
        }
    }
    pub async fn qr_code_create(
        &mut self,
        qr_code_params: &QRCodeRequest,
    ) -> Result<QRCodeResponse, SCBAPIError> {
        if self.access_token.is_none() {
            let req = self.authentication().await;
            match req {
                Ok(_) => {
                    info!("Authentication success");
                }
                Err(e) => {
                    error!("Authentication failed: {:?}", e);
                    return Err(e);
                }
            }
        }
        debug!("Request: {:#?}", qr_code_params);
        let client = create_client();
        let req = client
            .post(SANDBOX_QRCODE_CREATE_V1_URL)
            .headers(generate_header(&self.application_key))
            .header(
                "authorization",
                format!(
                    "Bearer {}",
                    self.access_token.as_ref().unwrap().access_token
                ),
            )
            .json(qr_code_params)
            .build()
            .expect("Failed to build request");

        debug!("Request : {:#?}", req);
        if let Some(body) = req.body() {
            let bytes = body.as_bytes().unwrap_or(&[]);
            let body_str = String::from_utf8_lossy(bytes);
            debug!("Request Body: {}", body_str);
        }

        let req = client.execute(req).await;
        match req {
            Ok(response) => {
                debug!("Response: {:#?}", response);
                let body = response.json::<SCBResponse<QRCodeResponse>>().await;
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
            Err(e) => Err(SCBAPIError::HttpRequestError(e)),
        }
    }
}
