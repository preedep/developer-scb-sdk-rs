use log::{debug, error, info};
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

        let request_uid = Uuid::new_v4();
        let req = create_client()
            .post(SANDBOX_OAUTH_TOKEN_V1_URL)
            .header("Content-Type", "application/json")
            .header("resourceOwnerId", self.application_name.to_string())
            .header("requestUId", request_uid.to_string())
            .header("accept-language", "EN")
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await;

        match req {
            Ok(response) => {
                let body = response.json::<SCBResponse<AccessToken>>().await.unwrap();
                if body.status.code != 1000 {
                    return Err(SCBAPIError::SCBError(body.status.description));
                }
                self.access_token = Some(body.data);
                Ok(())
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
        let request_uid = Uuid::new_v4();
        let req = create_client()
            .post(SANDBOX_QRCODE_CREATE_V1_URL)
            .header("Content-Type", "application/json")
            .header("resourceOwnerId", self.application_name.to_string())
            .header("requestUId", request_uid.to_string())
            .header("accept-language", "EN")
            .header(
                "authorization",
                format!(
                    "Bearer {}",
                    self.access_token.as_ref().unwrap().access_token
                ),
            )
            .body(serde_json::to_string(qr_code_params).unwrap())
            .send()
            .await;
        match req {
            Ok(response) => {
                let body = response
                    .json::<SCBResponse<QRCodeResponse>>()
                    .await;
                match body {
                    Ok(body) => {
                        debug!("Response: {:?}", body);
                        if body.status.code != 1000 {
                            return Err(SCBAPIError::SCBError(body.status.description));
                        }
                        Ok(body.data)
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                        Err(SCBAPIError::SCBError(e.to_string()))
                    }
                }
            }
            Err(e) => Err(SCBAPIError::HttpRequestError(e)),
        }
    }
}
