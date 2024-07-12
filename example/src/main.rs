use log::debug;
use corescbsdk::entities::qrcode::QRCodeRequest;
use corescbsdk::frameworks::apis::scb::SCBClientAPI;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();

    let application_name = std::env::var("APP_NAME").unwrap();
    let application_key = std::env::var("APP_KEY").unwrap();
    let secret_key = std::env::var("APP_SECRET").unwrap();

    let mut scb_client = SCBClientAPI::new(&application_name, &application_key, &secret_key);

    let mut qr_code_request = QRCodeRequest::new(&"CS".to_string(), &"100.00".to_string());

    let qr_code_request = qr_code_request
        .for_qr_cs(
            &"INVOICE".to_string(),
            &"684349039613126".to_string(),
            &"379479514042628".to_string(),
        )
        .add_cs_ext_expiry_time(&"2021-12-31T23:59:59+07:00".to_string())
        .add_cs_note(&"This is a payment for the invoice".to_string())
        .add_cs_user_defined(&"This is a user defined data".to_string());

    let res = scb_client.qr_code_create(&qr_code_request).await.expect("Failed to create QR Code");
    debug!("QRCode Response : {:#?}", res);
}
