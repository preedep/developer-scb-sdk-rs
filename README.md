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
