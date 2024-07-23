use std::process::Command;
use image::Luma;
use log::{debug, error, info};
use qrcode::QrCode;

use corescbsdk::entities::qr_code::{QRCodeRequestBuilder, QRCodeType};
use corescbsdk::frameworks::apis::scb::SCBClientAPI;

mod example_slip_verification;

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

    generate_qr_code(
        &application_name,
        &application_key,
        &secret_key,
        &biller_id,
        &biller_name,
        &prefix_ref3,
    )
    .await;
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
                    let image_path = "qrcode.png";
                    let code = QrCode::new(qr_code.qr_raw_data.unwrap()).unwrap();
                    let image = code.render::<Luma<u8>>().build();
                    image.save(image_path).unwrap();
                    info!("QR Code generated successfully");
                    
                    // Determine the command based on the platform
                    let command = if cfg!(target_os = "macos") {
                        format!("open {}", image_path)
                    } else if cfg!(target_os = "linux") {
                        format!("xdg-open {}", image_path)
                    } else if cfg!(target_os = "windows") {
                        format!("start {}", image_path)
                    } else {
                        error!("Unsupported platform.");
                        return;
                    };
                    
                    
                    // Execute the command
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&command)
                        .output()
                        .expect("Failed to execute command");

                    // Check if the command was successful
                    if output.status.success() {
                        info!("Successfully opened the image.");
                    } else {
                        eprintln!("Failed to open the image.");
                        if let Ok(stderr) = String::from_utf8(output.stderr) {
                            error!("Error: {}", stderr);
                        }
                    }
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
