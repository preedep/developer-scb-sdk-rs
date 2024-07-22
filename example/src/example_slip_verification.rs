use log::info;

use corescbsdk::entities::bill_pay::BillPaymentInquiryRequest;
use corescbsdk::frameworks::apis::scb::SCBClientAPI;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let application_name = std::env::var("APP_NAME").unwrap();
    let application_key = std::env::var("APP_KEY").unwrap();
    let secret_key = std::env::var("APP_SECRET").unwrap();

    let mut scb_client = SCBClientAPI::new(&application_name, &application_key, &secret_key);
    let r = scb_client
        .get_slip_verification_qr30(&"12345".to_string(), &"014".to_string())
        .await;

    match r {
        Ok(res) => {
            info!("Response: {:#?}", res);
        }
        Err(e) => {
            info!("Error: {:#?}", e);
        }
    }

    let r = scb_client
        .query_bill_payment_transaction(&BillPaymentInquiryRequest {
            event_code: "00300100".to_string(),
            transaction_date: "2019-08-22".to_string(),
            biller_id: None,
            reference1: None,
            reference2: None,
            partner_transaction_id: None,
            amount: None,
        })
        .await;

    match r {
        Ok(res) => {
            info!("Response: {:#?}", res);
        }
        Err(e) => {
            info!("Error: {:#?}", e);
        }
    }
}
