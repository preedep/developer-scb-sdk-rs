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

for BILLER_ID , BILLER_NAME , REF_3PREFIX get it from Developer SCB Portal , under the your application details

```chatinput
    let biller_id = std::env::var("BILLER_ID").unwrap();
    let biller_name = std::env::var("BILLER_NAME").unwrap();
    let prefix_ref3 = std::env::var("REF_3PREFIX").unwrap();


     
    async fn generate_qr_code(application_name: &String,
                          application_key: &String,
                          secret_key: &String,
                          biller_id: &String,
                            biller_name: &String,
                            prefix_ref3: &String,
) {
    let ref3 = format!("{}{}", prefix_ref3, "REFERENCE3");
    debug!("Merchant name : {} , Ref3: {}",biller_name, ref3);
    
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
```