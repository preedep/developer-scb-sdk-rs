mod example_slip_verification;

use corescbsdk::entities::qr_code::{QRCodeRequestBuilder, QRCodeType};
use corescbsdk::frameworks::apis::scb::SCBClientAPI;
use log::{debug, error, info};
use std::thread::spawn;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let application_name = std::env::var("APP_NAME").unwrap();
    let application_key = std::env::var("APP_KEY").unwrap();
    let secret_key = std::env::var("APP_SECRET").unwrap();

    let biller_id = std::env::var("BILLER_ID").unwrap();
    let biller_name = std::env::var("BILLER_NAME").unwrap();
    let prefix_ref3 = std::env::var("REF_3PREFIX").unwrap();

    let mut handles = vec![];
    for _ in 0..1 {
        let application_name = application_name.clone();
        let application_key = application_key.clone();
        let secret_key = secret_key.clone();

        let biller_id = biller_id.clone();
        let biller_name = biller_name.clone();
        let prefix_ref3 = prefix_ref3.clone();
        let handle = spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                generate_qr_code(
                    &application_name,
                    &application_key,
                    &secret_key,
                    &biller_id,
                    &biller_name,
                    &prefix_ref3,
                )
                .await;
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn generate_qr_code(
    application_name: &String,
    application_key: &String,
    secret_key: &String,
    biller_id: &String,
    biller_name: &String,
    prefix_ref3: &String,
) {
    let ref3 = format!("{}{}", prefix_ref3, "REFERENCE3");
    debug!("Merchant name : {} , Ref3: {}", biller_name, ref3);

    let mut scb_client = SCBClientAPI::new(&application_name, &application_key, &secret_key);
    let mut qr_code_req_builder = QRCodeRequestBuilder::new(&QRCodeType::PP, &"100.00".to_string());
    let qr_code_req_builder = qr_code_req_builder
        .for_qr_tag30(
            &"BILLERID".to_string(),
            &biller_id,
            &"REFERENCE1".to_string(),
            &ref3,
        )
        .add_ref2(&"REFERENCE2".to_string());

    match qr_code_req_builder.build() {
        Ok(qr_code_request) => {
            let res = scb_client.qr_code_create(&qr_code_request).await;
            match res {
                Ok(qr_code) => {
                    info!("QR Code: {:#?}", qr_code);
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}
