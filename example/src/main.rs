use corescbsdk::entities::qrcode::{QRCodeRequest, QRCodeType};
use corescbsdk::frameworks::apis::scb::SCBClientAPI;
use log::{error, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let application_name = std::env::var("APP_NAME").unwrap();
    let application_key = std::env::var("APP_KEY").unwrap();
    let secret_key = std::env::var("APP_SECRET").unwrap();

    let mut scb_client = SCBClientAPI::new(&application_name, &application_key, &secret_key);

    let mut qr_code_request = QRCodeRequest::new(&QRCodeType::PP, &"100.00".to_string());

    let qr_code_request = qr_code_request
        .for_qr_tag30(
            &"BILLERID".to_string(),
            &"123456789012345".to_string(),
            &"REFERENCE1".to_string(),
            &"SCB".to_string(),
        )
        .add_ref2(&"REFERENCE2".to_string());

    let res = scb_client.qr_code_create(&qr_code_request).await;

    match res {
        Ok(qr_code) => {
            info!("QR Code: {:#?}", qr_code);
        }
        Err(e) => {
            error!("Error: {:#?}", e);
        }
    }
}
