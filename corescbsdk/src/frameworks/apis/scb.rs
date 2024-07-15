use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::entities::base::{AccessToken, SCBAccessTokenRequest};
use crate::entities::bill_pay::{BillPaymentInquiryRequest, BillPaymentTransaction, BillPaymentTransactionSlip};
use crate::entities::qr_code::{QRCodeRequest, QRCodeResponse};
use crate::errors::scb_error::SCBAPIError;
use crate::frameworks::apis::api_utils::{api_url, generate_header, map_result, OAUTH_TOKEN_V1_URL};
use crate::frameworks::apis::payments::bill_pay;
use crate::frameworks::apis::payments::qr_code::qr_code_create;

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

    async fn request_access_token(&mut self) -> Result<(), SCBAPIError> {
        let request = SCBAccessTokenRequest {
            application_key: self.application_key.to_string(),
            application_secret: self.secret_key.to_string(),
            auth_code: None,
            state: None,
            code_challenge: None,
        };

        let req = create_client()
            .post(api_url(OAUTH_TOKEN_V1_URL))
            .headers(generate_header(&self.application_name, &None))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .map_err(|e| SCBAPIError::HttpRequestError(e));

        let res = map_result::<AccessToken>(req).await;
        match res {
            Ok(token) => {
                self.access_token = Some(token);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn qr_code_create(
        &mut self,
        qr_code_params: &QRCodeRequest,
    ) -> Result<QRCodeResponse, SCBAPIError> {
        self.get_access_token_if_need().await?;
        debug!("Request: {:#?}", qr_code_params);
        let client = create_client();
        let access_token = self.access_token.as_ref().unwrap();
        let application_key = self.application_key.clone();

        qr_code_create(&application_key, &client, access_token, qr_code_params).await
    }

    pub async fn get_slip_verification_qr30(
        &mut self,
        trans_ref: &String,
        sending_bank: &String,
    ) -> Result<BillPaymentTransactionSlip, SCBAPIError> {
        self.get_access_token_if_need().await?;
        let client = create_client();
        let access_token = self.access_token.as_ref().unwrap();
        let application_key = self.application_key.clone();

        bill_pay::get_bill_payment_transaction(
            &application_key,
            &client,
            access_token,
            trans_ref,
            sending_bank,
        )
        .await
    }

    pub async fn query_bill_payment_transaction(
        &mut self,
        params: &BillPaymentInquiryRequest,
    ) -> Result<Vec<BillPaymentTransaction>, SCBAPIError> {
        self.get_access_token_if_need().await?;
        let client = create_client();
        let access_token = self.access_token.as_ref().unwrap();
        let application_key = self.application_key.clone();

        bill_pay::query_bill_payment_transaction(&application_key, &client, access_token, params)
            .await
    }

    async fn get_access_token_if_need(&mut self) -> Result<(), SCBAPIError> {
        if self.access_token.is_none() {
            let req = self.request_access_token().await;
            match req {
                Ok(_) => {
                    info!("Authentication success");
                }
                Err(e) => {
                    error!("Authentication failed: {:?}", e);
                    return Err(e);
                }
            }
        } else {
            let expired_at = self.access_token.as_ref().unwrap().expires_at;
            let current_time = chrono::Utc::now().timestamp();
            debug!("Current Time: {}", current_time);
            debug!("Expired Time: {}", expired_at);

            if current_time >= expired_at {
                let req = self.request_access_token().await;
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
        }
        Ok(())
    }
}
