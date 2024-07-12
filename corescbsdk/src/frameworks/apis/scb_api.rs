use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::qrcode::QRCodeRequest;
use crate::entities::base::{AccessToken, SCBAccessTokenRequest, SCBResponse};
use crate::errors::scb_error::SCBAPIError;



const SANDBOX_OAUTH_TOKEN_V1_URL: &str = "https://api-sandbox.partners.scb/partners/sandbox/v1/oauth/token";
const SANDBOX_QRCODE_CREATE_V1_URL: &str = "https://api-sandbox.partners.scb/partners/sandbox/v1/payment/qrcode/create";


#[derive(Debug, Serialize, Deserialize)]
pub struct SCBClientAPI {
    application_name: String,
    application_key: String,
    secret_key: String,
    access_token: Option<AccessToken>
}
impl SCBClientAPI {
    pub fn new(application_name: &String,
               application_key: &String,
               secret_key: &String) -> SCBClientAPI {
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
        let req = reqwest::Client::new().post(SANDBOX_OAUTH_TOKEN_V1_URL)
            .header("Content-Type", "application/json")
            .header("resourceOwnerId", self.application_name.to_string())
            .header("requestUId",request_uid.to_string())
            .header("accept-language","EN")
            .body(serde_json::to_string(&request).unwrap())
            .send().await;

        match req {
            Ok(response) => {
                let body = response.json::<SCBResponse<AccessToken>>().await.unwrap();
                if body.status.code != 1000 {
                    return Err(SCBAPIError::SCBError(body.status.description));
                }
                self.access_token = Some(body.data);
                Ok(())
            },
            Err(e) => {
                Err(SCBAPIError::HttpRequestError(e))
            }
        }
    }

    pub async fn qr_code_create(&self,qr_code_params: &QRCodeRequest) {

    }
}