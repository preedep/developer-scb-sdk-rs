# SCB SDK for Rust
SCB SDK for Rust is a library that provides a simple way to interact with the SCB API (https://developer.scb/#/home). 

## How to use SCB SDK for Rust

```chatinput
    let application_name = std::env::var("APP_NAME").unwrap();
    let application_key = std::env::var("APP_KEY").unwrap();
    let secret_key = std::env::var("APP_SECRET").unwrap();
```
APP_NAME , APP_KEY , APP_SECRET are the credentials that you will get from SCB API Portal


```chatinput
    let client = scb_sdk_rust::Client::new(application_name, application_key, secret_key);
    let response = client.get_account_balance("1234567890").await;
    println!("{:?}", response);
```
## Create QR Code
QR code generation of Thai QR Code standard Tag 30 (QR 30) and QR Card Scheme (QR CS). There are 3 use cases.
- Generate QR 30 Only
- Generate QR CS Only
- Generate QR 30 and QR CS together in a single QR Code

```chatinput
     let mut qr_code_req_builder = QRCodeRequestBuilder::new(&QRCodeType::PP,
                                                            &"100.00".to_string());

    let qr_code_req_builder = qr_code_req_builder.for_qr_tag30(
            &"BILLERID".to_string(),
            &"123456789012345".to_string(),
            &"REFERENCE1".to_string(),
            &"SCB".to_string(),
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
```