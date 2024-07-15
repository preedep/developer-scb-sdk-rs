use reqwest::Client;

use crate::entities::base::AccessToken;
use crate::entities::bill_pay_tx::BillPaymentTransaction;
use crate::errors::scb_error::SCBAPIError;
use crate::frameworks::apis::api_utils::{
    api_url, generate_header, map_result, BILL_PAYMENT_TRANSACTION_V1_URL,
};

pub async fn get_bill_payment_transaction(
    application_key: &String,
    client: &Client,
    access_token: &AccessToken,
    trans_ref: &String,
    sending_bank: &String,
) -> Result<BillPaymentTransaction, SCBAPIError> {
    let url = format!("{}/{}", BILL_PAYMENT_TRANSACTION_V1_URL, trans_ref);

    let req = client
        .get(api_url(&url))
        .query(&[("sendingBank", sending_bank)])
        .headers(generate_header(
            &application_key,
            &Some(access_token.clone()),
        ))
        .build()
        .expect("Failed to build request");

    let req = client
        .execute(req)
        .await
        .map_err(|e| SCBAPIError::HttpRequestError(e));
    map_result::<BillPaymentTransaction>(req).await
}
