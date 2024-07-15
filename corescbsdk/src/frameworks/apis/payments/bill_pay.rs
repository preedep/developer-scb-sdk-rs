use log::debug;
use reqwest::Client;

use crate::entities::base::AccessToken;
use crate::entities::bill_pay::{BillPaymentInquiryRequest, BillPaymentTransaction, BillPaymentTransactionSlip};
use crate::errors::scb_error::SCBAPIError;
use crate::frameworks::apis::api_utils::{api_url, BILL_PAYMENT_TRANSACTION_V1_URL, generate_header, INQUIRY_BILL_PAYMENT_TRANSACTION_V1_URL, map_result};

pub async fn get_bill_payment_transaction(
    application_key: &String,
    client: &Client,
    access_token: &AccessToken,
    trans_ref: &String,
    sending_bank: &String,
) -> Result<BillPaymentTransactionSlip, SCBAPIError> {
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

    debug!("Request : {:#?}", req);

    let req = client
        .execute(req)
        .await
        .map_err(|e| SCBAPIError::HttpRequestError(e));
    map_result::<BillPaymentTransactionSlip>(req).await
}

pub async fn query_bill_payment_transaction(
    application_key: &String,
    client: &Client,
    access_token: &AccessToken,params: &BillPaymentInquiryRequest) -> Result<Vec<BillPaymentTransaction>,SCBAPIError>{

    let req = client.get(api_url(INQUIRY_BILL_PAYMENT_TRANSACTION_V1_URL))
        .query(params)
        .headers(generate_header(
            &application_key,
            &Some(access_token.clone()),
        )).build().expect("Failed to build request");

    debug!("Request : {:#?}", req);

    let req = client
        .execute(req)
        .await
        .map_err(|e| SCBAPIError::HttpRequestError(e));
    map_result::<Vec<BillPaymentTransaction>>(req).await
}